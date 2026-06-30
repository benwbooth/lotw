//! Realtime music server for the live-edit editor.
//!
//! Plays a song through the actual ported sound engine + APU out to SDL3, and
//! speaks a tiny line protocol on stdin/stdout so an editor extension can drive
//! it. On an edit it recompiles the song source (the `music-jit` cdylib),
//! reloads, and fast-forwards to the current tick so playback keeps its place.
//!
//! stdin commands (one per line):
//!   play | stop | rewind | loop on|off
//!   rom <idx>            play a compiled-in ROM song (no recompile)
//!   src <path> <idx>     compile <path> via music-jit, play song <idx>, keep position
//! stdout events:
//!   pos <tick> <s0> <s1> <s2> <s3>   per-frame section index per channel (-1 = none)
//!   ok <msg> | err <msg> | loaded <idx>

mod common;

use std::io::{BufRead, Write};
use std::sync::mpsc;
use std::time::{Duration, Instant};

use lotw::{Engine, RoutineContext, audio, game, music};
use sdl3::audio::{AudioFormat, AudioSpec};

fn cpu_to_prg(cpu: usize, song: usize) -> Option<usize> {
    let (lo, hi) = if song < 10 { (0x14000, 0x16000) } else { (0x18000, 0x1A000) };
    match cpu {
        0x8000..0xA000 => Some(lo + cpu - 0x8000),
        0xA000..0xC000 => Some(hi + cpu - 0xA000),
        _ => None,
    }
}

/// The four channel byte streams + per-channel section-start token indices, and
/// (for an extracted section) how many leading tokens are the prepended
/// parameter state — so the player can report section-relative positions.
struct SongData {
    channels: [Vec<u8>; 4],
    section_starts: [Vec<usize>; 4],
    prefix_toks: [usize; 4],
}

impl SongData {
    /// A song from the compiled-in DSL (ROM song).
    fn from_dsl(idx: usize) -> Option<SongData> {
        let s = music::get(idx)?;
        let channels = std::array::from_fn(|c| audio::assemble(&s.channels[c].1));
        Some(SongData { channels, section_starts: s.section_starts.clone(), prefix_toks: [0; 4] })
    }

    /// Extract a single section as a playable (looping) mini-song: each channel
    /// is the section's tokens, prefixed with the last duty/volume/flags/pitch/
    /// sweep command set before it (so it sounds right in isolation) and
    /// terminated so it loops.
    fn extract_section(&self, n: usize) -> SongData {
        let mut channels: [Vec<u8>; 4] = Default::default();
        let mut prefix_toks = [0usize; 4];
        for c in 0..4 {
            let bytes = &self.channels[c];
            let starts = &self.section_starts[c];
            if n >= starts.len() {
                channels[c] = vec![0x00];
                continue;
            }
            let start_tok = starts[n];
            let end_tok = starts.get(n + 1).copied().unwrap_or(usize::MAX);
            let mut last_cmd = [None; 5];
            let mut prefix = Vec::new();
            let mut section = Vec::new();
            let (mut bi, mut ti) = (0usize, 0usize);
            while bi < bytes.len() {
                if ti == start_tok {
                    for (id, arg) in last_cmd.iter().enumerate() {
                        if let Some(a) = arg {
                            prefix.extend_from_slice(&[0xFF, id as u8, *a]);
                        }
                    }
                }
                let b = bytes[bi];
                let size = match b {
                    0x00 => 1,
                    0xFF => 3,
                    x if x & 0x80 != 0 => 1,
                    _ if c == 3 => 1, // noise hit = 1 byte
                    _ => 2,
                };
                if ti >= start_tok && ti < end_tok && b != 0x00 {
                    section.extend_from_slice(&bytes[bi..bi + size]);
                }
                if ti < start_tok && b == 0xFF && (bytes[bi + 1] as usize) < 5 {
                    last_cmd[bytes[bi + 1] as usize] = Some(bytes[bi + 2]);
                }
                bi += size;
                ti += 1;
            }
            prefix_toks[c] = prefix.len() / 3; // each prepended command is 3 bytes / 1 token
            prefix.extend_from_slice(&section);
            prefix.push(0x00); // terminate / loop point
            channels[c] = prefix;
        }
        SongData { channels, section_starts: Default::default(), prefix_toks }
    }

    /// Parse a `song_blob` from the JIT cdylib (see music-jit/src/lib.rs).
    fn from_blob(b: &[u8]) -> Option<SongData> {
        fn rd(b: &[u8], p: &mut usize) -> Option<usize> {
            let v = u32::from_le_bytes(b.get(*p..*p + 4)?.try_into().ok()?) as usize;
            *p += 4;
            Some(v)
        }
        let mut p = 0usize;
        if rd(b, &mut p)? != 4 {
            return None;
        }
        let mut channels: [Vec<u8>; 4] = Default::default();
        let mut starts: [Vec<usize>; 4] = Default::default();
        for c in 0..4 {
            let len = rd(b, &mut p)?;
            channels[c] = b.get(p..p + len)?.to_vec();
            p += len;
            let n = rd(b, &mut p)?;
            for _ in 0..n {
                starts[c].push(rd(b, &mut p)?);
            }
        }
        Some(SongData { channels, section_starts: starts, prefix_toks: [0; 4] })
    }
}

/// Playback state: the engine, what's loaded, and the now-playing maps.
struct Player {
    rom: Vec<u8>,
    prg0: usize, // file offset of PRG start (16)
    engine: Engine,
    r: RoutineContext,
    idx: usize,
    tick: usize,
    playing: bool,
    looping: bool,
    // Per channel: (prg_offset, token_index) for the loaded streams + section starts.
    tok_at: [Vec<(usize, usize)>; 4],
    section_starts: [Vec<usize>; 4],
    prefix_toks: [usize; 4],
    last_pos: [i64; 4],
    tmp: &'static str, // temp .nes path this player rebuilds from (distinct per voice)
}

impl Player {
    fn new(rom_path: &str, tmp: &'static str) -> Result<Player, Box<dyn std::error::Error>> {
        let rom = std::fs::read(rom_path)?;
        let engine = common::load_rom(rom_path, false)?;
        Ok(Player {
            rom,
            prg0: 16,
            engine,
            r: RoutineContext::default(),
            idx: 0,
            tick: 0,
            playing: false,
            looping: true,
            prefix_toks: [0; 4],
            tok_at: Default::default(),
            section_starts: Default::default(),
            last_pos: [-1; 4],
            tmp,
        })
    }

    /// Patch the song's channel streams into the ROM, (re)load the engine, and
    /// fast-forward to `to_tick` (so an edit keeps its place).
    fn load(&mut self, idx: usize, data: &SongData, to_tick: usize) -> Result<(), String> {
        let chans = audio::song_channels(&self.rom[self.prg0..self.prg0 + self.rom[4] as usize * 16_384])
            .into_iter()
            .find(|(i, _)| *i == idx)
            .map(|(_, c)| c)
            .ok_or("song index has no ROM slot")?;
        self.tok_at = Default::default();
        for (ci, off) in chans.iter().enumerate() {
            let Some(off) = off else { continue };
            let bytes = &data.channels[ci];
            let end = self.prg0 + off + bytes.len();
            if end > self.rom.len() {
                return Err("channel stream too long to patch".into());
            }
            self.rom[self.prg0 + off..end].copy_from_slice(bytes);
            // token offsets for now-playing
            let mut o = *off;
            let mut ti = 0;
            let mut bi = 0;
            while bi < bytes.len() {
                self.tok_at[ci].push((o, ti));
                bi += match bytes[bi] {
                    0x00 => 1,
                    0xFF => 3,
                    b if b & 0x80 != 0 => 1,
                    _ if ci == 3 => 1, // noise hit = 1 byte
                    _ => 2,
                };
                o = *off + bi;
                ti += 1;
            }
        }
        self.section_starts = data.section_starts.clone();
        self.prefix_toks = data.prefix_toks;

        // Rebuild the engine from the patched ROM.
        let tmp = self.tmp;
        std::fs::write(tmp, &self.rom).map_err(|e| e.to_string())?;
        self.engine = common::load_rom(tmp, false).map_err(|e| e.to_string())?;
        self.idx = idx;
        self.restart();
        // Fast-forward (silently re-tick) to preserve position across an edit.
        for _ in 0..to_tick {
            game::sound_tick(&mut self.engine, &mut self.r);
        }
        self.tick = to_tick;
        Ok(())
    }

    /// Re-init the current song to its start (channel pointers back to the stream
    /// heads), tick 0. The ROM stays patched, so this works for an edited song.
    fn restart(&mut self) {
        self.r = RoutineContext::default();
        game::ram_state_init(&mut self.engine, &mut self.r);
        game::farcall_bank_0C0D_seed(&mut self.engine, &mut self.r);
        self.engine.state.song = self.idx as u8;
        self.engine.state.sound_paused = 0;
        game::song_init(&mut self.engine, &mut self.r);
        self.engine.device_write(lotw::engine::reg::APU_STATUS, 0x0F);
        self.tick = 0;
        self.last_pos = [-1; 4];
    }

    fn live_cpu(&self, c: usize) -> usize {
        (self.engine.state.sound_channel_byte(2, (c * 16) as i32) | self.engine.state.sound_channel_byte(3, (c * 16) as i32) << 8) as usize
    }

    /// Current token index per channel (-1 if unknown), from each channel's live
    /// stream pointer -> PRG offset -> token index. The editor maps this to the
    /// source element being played.
    fn tokens(&self) -> [i64; 4] {
        std::array::from_fn(|c| {
            let Some(po) = cpu_to_prg(self.live_cpu(c), self.idx) else { return -1 };
            // The stream pointer sits at the *next* token while the current note
            // sounds, so the playing token is the one strictly before it.
            let Some(ti) = self.tok_at[c].iter().take_while(|t| t.0 < po).last().map(|t| t.1) else { return -1 };
            // For an isolated section, subtract the prepended parameter tokens so
            // the index is relative to the section's source notes.
            ti as i64 - self.prefix_toks[c] as i64
        })
    }
}

/// Compile the song source at `path` via the music-jit cdylib and read song
/// `idx`'s data out through the C-ABI.
/// Compile the song source at `path` via the `music-jit` cdylib and read one of
/// its blob exports (`song_blob` / `sfx_blob`) for `idx`. Returns the raw blob.
fn jit_blob(path: &str, symbol: &[u8], idx: usize) -> Result<Vec<u8>, String> {
    std::fs::copy(path, "music-jit/src/songs.rs").map_err(|e| e.to_string())?;
    let out = std::process::Command::new("cargo")
        .args(["build", "-p", "music-jit"])
        .env("NIX_LDFLAGS", "")
        .env("RUSTFLAGS", "-C debuginfo=0 -C link-arg=-fuse-ld=mold")
        .output()
        .map_err(|e| e.to_string())?;
    if !out.status.success() {
        return Err(format!("compile failed: {}", String::from_utf8_lossy(&out.stderr).lines().rev().take(3).collect::<Vec<_>>().join(" | ")));
    }
    // Load a fresh copy of the .so so recompiles never fight a mapped file.
    let so = format!("/tmp/ben/scratch/jit_{}.so", std::process::id());
    std::fs::copy("target/debug/libmusic_jit.so", &so).map_err(|e| e.to_string())?;
    let mut buf = vec![0u8; 1 << 20];
    let len = unsafe {
        let lib = libloading::Library::new(&so).map_err(|e| e.to_string())?;
        let f: libloading::Symbol<unsafe extern "C" fn(u32, *mut u8, usize) -> usize> = lib.get(symbol).map_err(|e| e.to_string())?;
        f(idx as u32, buf.as_mut_ptr(), buf.len())
    };
    if len == 0 {
        return Err("blob returned 0 (missing entry / buffer too small)".into());
    }
    buf.truncate(len);
    Ok(buf)
}

fn jit_compile(path: &str, idx: usize) -> Result<SongData, String> {
    let buf = jit_blob(path, b"song_blob", idx)?;
    SongData::from_blob(&buf).ok_or_else(|| "bad song blob".into())
}

/// Compile and return the assembled bytes of SFX `idx` from the source at `path`.
fn jit_sfx(path: &str, idx: usize) -> Result<Vec<u8>, String> {
    jit_blob(path, b"sfx_blob", idx)
}

/// Build a one-note song for the preview voice: the parsed note `token` on
/// channel `chan` with default timbre commands, other channels silent. Returns
/// the song and the note's tick duration.
fn preview_song(chan: usize, token: &str) -> Option<(SongData, u8)> {
    let toks = audio::parse(token).ok()?;
    let note = toks.iter().copied().find(|t| matches!(t, audio::Tok::Note { .. } | audio::Tok::Hit { .. } | audio::Tok::Rest { .. }))?;
    let dur = match note {
        audio::Tok::Note { dur, .. } | audio::Tok::Hit { dur } | audio::Tok::Rest { dur } => dur,
        _ => 12,
    };
    let mut channels: [Vec<u8>; 4] = std::array::from_fn(|_| vec![0x00]);
    let mut s = if chan == 3 {
        vec![0xFF, 4, 0x0c, 0xFF, 1, 254, 0xFF, 0, 2] // noise: sweep(period), volume, duty
    } else {
        vec![0xFF, 0, 32, 0xFF, 1, 255] // pulse/triangle: duty, volume
    };
    s.extend_from_slice(&audio::assemble(std::slice::from_ref(&note)));
    s.push(0x00);
    channels[chan] = s;
    Some((SongData { channels, section_starts: Default::default(), prefix_toks: [0; 4] }, dur))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    let rom = args.get(1).map(String::as_str).unwrap_or("rom/lotw.nes");
    let mut player = Player::new(rom, "/tmp/ben/scratch/music_server.nes")?;
    // A dedicated voice for note previews (type-to-hear) so it never disturbs
    // the main playback; its audio is mixed in for the previewed note's length.
    let mut preview = Player::new(rom, "/tmp/ben/scratch/music_preview.nes")?;
    let mut preview_frames = 0usize;
    let mut preview_buf = vec![0i16; common::SPF];

    // SDL3 mono 16-bit playback stream at the APU sample rate.
    let sdl = sdl3::init()?;
    let audio = sdl.audio()?;
    let spec = AudioSpec { freq: Some(lotw::engine::APU_SR as i32), channels: Some(1), format: Some(AudioFormat::s16_sys()) };
    let stream = audio.default_playback_device().open_device_stream(Some(&spec)).ok();
    if let Some(s) = &stream {
        let _ = s.resume();
    }

    // Read stdin commands on a thread so the audio loop never blocks.
    let (tx, rx) = mpsc::channel::<String>();
    std::thread::spawn(move || {
        for line in std::io::stdin().lock().lines().map_while(Result::ok) {
            if tx.send(line).is_err() {
                break;
            }
        }
    });

    let stdout = std::io::stdout();
    let say = |m: &str| {
        let mut o = stdout.lock();
        let _ = writeln!(o, "{m}");
        let _ = o.flush();
    };

    let mut audio_buf = vec![0i16; common::SPF];
    let mut next = Instant::now();
    loop {
        // Handle any queued commands.
        while let Ok(line) = rx.try_recv() {
            let mut it = line.split_whitespace();
            match it.next() {
                Some("play") => player.playing = true,  // resume
                Some("stop") => player.playing = false, // pause (keep position)
                Some("reset") => {
                    // Stop and return to the start of the current song/section.
                    player.restart();
                    player.playing = false;
                    say(&format!("pos {} -1 -1 -1 -1", player.tick));
                }
                Some("rewind") => {
                    let (i, d) = (player.idx, SongData::from_dsl(player.idx));
                    if let Some(d) = d {
                        let _ = player.load(i, &d, 0);
                    }
                }
                Some("loop") => player.looping = it.next() != Some("off"),
                Some("preview") => {
                    // preview <chan 0..3> <note token, e.g. c4e / hite> — hear a note.
                    let chan = it.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0).min(3);
                    if let Some((data, dur)) = it.next().and_then(|tok| preview_song(chan, tok)) {
                        if preview.load(0, &data, 0).is_ok() {
                            preview.playing = true;
                            preview_frames = (dur as usize).max(16) + 6;
                        }
                    }
                }
                Some("sfx") => {
                    // sfx <idx> — play ROM sound effect `idx` alone on the preview voice.
                    let idx = it.next().and_then(|s| s.parse::<usize>().ok()).unwrap_or(0);
                    let dur = music::sfx(idx).map(|s| audio::channel_ticks(&s)).unwrap_or(60);
                    // Load any song to map the SFX bank, then mute the music bed and
                    // request the sfx via the engine's overlay (prompt_state).
                    if let Some(d) = SongData::from_dsl(0) {
                        if preview.load(0, &d, 0).is_ok() {
                            preview.engine.state.sound_paused = 1;
                            preview.engine.state.prompt_state = idx as u8;
                            preview.engine.state.prompt_argument = 0xFF; // max priority
                            preview.playing = true;
                            preview_frames = (dur as usize).max(20) + 8;
                            say(&format!("loaded sfx {idx}"));
                        }
                    }
                }
                Some("sfxsrc") => {
                    // sfxsrc <path> <idx> — compile + play the *edited* sound effect as
                    // a one-channel song (pulse2 = the SFX, others silent) so it reuses
                    // the whole song path: play/pause, loop, pos events, highlighting.
                    let path = it.next().unwrap_or("");
                    let idx: usize = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                    let res = jit_sfx(path, idx).and_then(|bytes| {
                        let mut channels: [Vec<u8>; 4] = std::array::from_fn(|_| vec![0x00]);
                        channels[1] = bytes; // pulse2
                        let mut section_starts: [Vec<usize>; 4] = Default::default();
                        section_starts[1] = vec![0];
                        let data = SongData { channels, section_starts, prefix_toks: [0; 4] };
                        player.load(0, &data, 0)
                    });
                    match res {
                        Ok(()) => {
                            player.playing = true;
                            say(&format!("loaded {idx}"));
                        }
                        Err(e) => say(&format!("err {e}")),
                    }
                }
                Some("rom") => match it.next().and_then(|s| s.parse().ok()).and_then(|i: usize| SongData::from_dsl(i).map(|d| (i, d))) {
                    Some((i, d)) => match player.load(i, &d, 0) {
                        Ok(()) => {
                            player.playing = true;
                            say(&format!("loaded {i}"));
                        }
                        Err(e) => say(&format!("err {e}")),
                    },
                    None => say("err bad rom index"),
                },
                Some("src") => {
                    let path = it.next().unwrap_or("");
                    let idx: usize = it.next().and_then(|s| s.parse().ok()).unwrap_or(0);
                    // Optional 4th arg = a section to play in isolation (looping).
                    let section: Option<usize> = it.next().and_then(|s| s.parse().ok());
                    // Whole-song reloads keep the playhead; section plays restart.
                    let keep = if section.is_some() { 0 } else { player.tick };
                    let res = jit_compile(path, idx).map(|d| match section {
                        Some(n) => d.extract_section(n),
                        None => d,
                    });
                    match res.and_then(|d| player.load(idx, &d, keep)) {
                        Ok(()) => {
                            player.playing = true;
                            say(&format!("loaded {idx}"));
                        }
                        Err(e) => say(&format!("err {e}")),
                    }
                }
                Some(other) => say(&format!("err unknown command {other}")),
                None => {}
            }
        }

        // Advance one frame of audio while the main voice plays and/or a note
        // preview is ringing (the preview is mixed on top, never disturbing the
        // main playhead).
        if player.playing || preview_frames > 0 {
            if player.playing {
                game::sound_tick(&mut player.engine, &mut player.r);
                player.engine.apu.frame();
                player.engine.apu.generate(&mut audio_buf);
            } else {
                audio_buf.iter_mut().for_each(|s| *s = 0);
            }
            if preview_frames > 0 {
                game::sound_tick(&mut preview.engine, &mut preview.r);
                preview.engine.apu.frame();
                preview.engine.apu.generate(&mut preview_buf);
                for (a, p) in audio_buf.iter_mut().zip(&preview_buf) {
                    *a = (*a as i32 + *p as i32).clamp(i16::MIN as i32, i16::MAX as i32) as i16;
                }
                preview_frames -= 1;
                if preview_frames == 0 {
                    preview.playing = false;
                }
            }
            if let Some(s) = &stream {
                if s.queued_bytes().unwrap_or(0) < (audio_buf.len() * 2 * 4) as i32 {
                    let _ = s.put_data_i16(&audio_buf);
                }
            }
        }
        if player.playing {
            player.tick += 1;
            let toks = player.tokens();
            if toks != player.last_pos {
                player.last_pos = toks;
                say(&format!("pos {} {} {} {} {}", player.tick, toks[0], toks[1], toks[2], toks[3]));
            }
        }

        next += Duration::from_nanos(1_000_000_000 / common::FPS as u64);
        let now = Instant::now();
        if next > now {
            std::thread::sleep(next - now);
        } else {
            next = now;
        }
    }
}
