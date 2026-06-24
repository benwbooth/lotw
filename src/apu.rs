//! Software shim for the NES 2A03 APU (Audio Processing Unit).
//!
//! This is not a cycle-accurate emulation. It models the four tone channels the
//! game uses (two pulse/square channels, one triangle channel, one noise
//! channel) at a behavioral level: register writes set up channel state, the
//! frame sequencer clocks the length counters, and [`Apu::generate`] synthesizes
//! audio samples directly at the host sample rate ([`APU_SR`]) by phase-stepping
//! each channel's waveform. The DMC (delta modulation / sample) channel is not
//! implemented.
//!
//! It also provides minimal little-endian WAV file export helpers used to dump
//! generated audio to disk.

use std::{fs::File, io::Write, path::Path};

use crate::engine::APU_SR;

/// NES CPU/APU clock frequency in Hz (NTSC 2A03). All channel timer dividers are
/// derived from this base clock to compute output tone frequencies.
const CPU_HZ: f64 = 1_789_773.0;

/// APU length-counter load table. The 5-bit length index in the high byte of a
/// channel's timer register selects one of these 32 entries, giving the number
/// of frame-counter ticks the channel plays before its length counter silences
/// it. This is the canonical 2A03 length lookup table.
const LEN_TBL: [i32; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];

/// Noise channel period table (in CPU clocks per LFSR shift). The low 4 bits of
/// the noise period register ($400E) index this table; larger entries produce
/// lower-pitched noise. These are the canonical NTSC 2A03 noise periods.
const NOISE_PER: [i32; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];

/// Pulse-channel duty cycles, expressed as the fraction of the waveform period
/// spent in the high state. Indexed by the 2-bit duty field, these correspond to
/// the 2A03's 12.5%, 25%, 50%, and 75% (inverted 25%) duty settings.
const DUTY: [f64; 4] = [0.125, 0.25, 0.5, 0.75];

/// State of one pulse (square) channel.
#[derive(Clone, Copy, Default)]
struct Pulse {
    /// Whether the channel is enabled via the status register ($4015).
    enabled: bool,
    /// Length counter: ticks remaining before the channel auto-silences.
    len: i32,
    /// 11-bit timer/period value controlling the output tone frequency.
    period: i32,
    /// Constant volume / envelope level (0..=15).
    vol: i32,
    /// Duty-cycle index (0..=3) into [`DUTY`].
    duty: i32,
    /// Length-counter halt flag (also envelope loop); when set, the length
    /// counter does not decrement.
    halt: bool,
    /// Normalized waveform phase accumulator in [0, 1).
    phase: f64,
}

/// State of the triangle channel.
#[derive(Clone, Copy, Default)]
struct Tri {
    /// Whether the channel is enabled via the status register ($4015).
    enabled: bool,
    /// Length counter: ticks remaining before the channel auto-silences.
    len: i32,
    /// 11-bit timer/period value controlling the output tone frequency.
    period: i32,
    /// Linear counter reload value (0..=127); zero silences the channel here.
    linear: i32,
    /// Linear-counter control flag (doubles as length-counter halt).
    control: bool,
    /// Normalized waveform phase accumulator in [0, 1).
    phase: f64,
}

/// State of the noise channel.
#[derive(Clone, Copy)]
struct Noise {
    /// Whether the channel is enabled via the status register ($4015).
    enabled: bool,
    /// Length counter: ticks remaining before the channel auto-silences.
    len: i32,
    /// Period in CPU clocks per LFSR shift (looked up from [`NOISE_PER`]).
    period: i32,
    /// Constant volume / envelope level (0..=15).
    vol: i32,
    /// Mode flag: 0 = long (32767-step) sequence, non-zero = short sequence.
    mode: i32,
    /// Length-counter halt flag; when set, the length counter does not decrement.
    halt: bool,
    /// 15-bit linear-feedback shift register driving the pseudo-random output.
    lfsr: u32,
    /// Sub-shift phase accumulator; one LFSR shift occurs each time it wraps 1.0.
    phase: f64,
}

impl Default for Noise {
    fn default() -> Self {
        Self {
            enabled: false,
            len: 0,
            period: 0,
            vol: 0,
            mode: 0,
            halt: false,
            // The LFSR must be seeded non-zero (hardware resets it to 1);
            // an all-zero register would stay stuck at zero.
            lfsr: 1,
            phase: 0.0,
        }
    }
}

/// The software APU: the four tone channels plus the cached status register.
#[derive(Default)]
pub struct Apu {
    /// Pulse channel 1 (registers $4000-$4003).
    p1: Pulse,
    /// Pulse channel 2 (registers $4004-$4007).
    p2: Pulse,
    /// Triangle channel (registers $4008-$400B).
    tr: Tri,
    /// Noise channel (registers $400C-$400F).
    nz: Noise,
    /// Last value written to the status register ($4015).
    status: i32,
}

impl Apu {
    /// Construct a fresh APU with all channels in their reset state.
    pub fn new() -> Self {
        // Start from the derived defaults, then run the explicit reset so the
        // (non-default) noise LFSR seed is established consistently.
        let mut apu = Self::default();
        apu.reset();
        apu
    }

    /// Clear all channel state back to power-on defaults.
    pub fn reset(&mut self) {
        // Reset every channel and the cached status register to silence.
        self.p1 = Pulse::default();
        self.p2 = Pulse::default();
        self.tr = Tri::default();
        self.nz = Noise::default();
        self.status = 0;
    }

    /// Handle a CPU write to an APU register in the $4000-$4017 range.
    ///
    /// `addr` is the memory-mapped register address and `val` the byte written.
    /// Both are masked to their valid widths. Each register decodes per the 2A03
    /// layout: channel control bytes carry duty/volume/halt, the two timer bytes
    /// carry the low and high (plus length-index) bits of the 11-bit period, and
    /// the status register ($4015) enables/disables channels and zeroes the
    /// length counters of any channel it disables. Unhandled/unused addresses are
    /// silently ignored.
    pub fn write(&mut self, addr: i32, val: i32) {
        // Mask the written value to a single byte.
        let val = val & 0xFF; // 255
        // Mask the address to the 16-bit memory space, then dispatch by register.
        match addr & 0xFFFF {
            // 0xFFFF = 65535
            // $4000: pulse 1 control — duty (bits 6-7), halt (bit 5), volume (bits 0-3).
            0x4000 => {
                // 0x4000 = 16384
                self.p1.duty = val >> 6; // top two bits select duty cycle
                self.p1.halt = (val & 0x20) != 0; // bit 5 = length halt / envelope loop
                self.p1.vol = val & 0x0F; // bits 0-3 = volume (0..=15)
            }
            // $4001: pulse 1 sweep unit — not modeled, ignored.
            0x4001 => {} // 16385
            // $4002: pulse 1 timer low byte — replace low 8 bits of the period.
            0x4002 => self.p1.period = (self.p1.period & 0x700) | val, // 0x700 = 1792 keeps high bits
            // $4003: pulse 1 timer high bits + length index; also restarts phase.
            0x4003 => {
                // 16387
                self.p1.period = (self.p1.period & 0xFF) | ((val & 0x07) << 8); // 0xFF=255 low byte, bits 0-2 = period high
                self.p1.len = LEN_TBL[((val >> 3) & 0x1F) as usize]; // bits 3-7 = length index (0x1F = 31)
                self.p1.phase = 0.0; // writing $4003 resets the sequencer phase
            }
            // $4004: pulse 2 control — same layout as $4000.
            0x4004 => {
                // 16388
                self.p2.duty = val >> 6;
                self.p2.halt = (val & 0x20) != 0;
                self.p2.vol = val & 0x0F;
            }
            // $4005: pulse 2 sweep unit — not modeled, ignored.
            0x4005 => {} // 16389
            // $4006: pulse 2 timer low byte.
            0x4006 => self.p2.period = (self.p2.period & 0x700) | val, // 16390
            // $4007: pulse 2 timer high bits + length index; also restarts phase.
            0x4007 => {
                // 16391
                self.p2.period = (self.p2.period & 0xFF) | ((val & 0x07) << 8);
                self.p2.len = LEN_TBL[((val >> 3) & 0x1F) as usize];
                self.p2.phase = 0.0;
            }
            // $4008: triangle linear counter — control flag (bit 7) + reload (bits 0-6).
            0x4008 => {
                // 16392
                self.tr.control = (val & 0x80) != 0; // bit 7 = control / length halt
                self.tr.linear = val & 0x7F; // bits 0-6 = linear counter reload (0x7F = 127)
            }
            // $400A: triangle timer low byte.
            0x400A => self.tr.period = (self.tr.period & 0x700) | val, // 16394
            // $400B: triangle timer high bits + length index.
            0x400B => {
                // 16395
                self.tr.period = (self.tr.period & 0xFF) | ((val & 0x07) << 8);
                self.tr.len = LEN_TBL[((val >> 3) & 0x1F) as usize];
            }
            // $400C: noise control — halt (bit 5) + volume (bits 0-3).
            0x400C => {
                // 16396
                self.nz.halt = (val & 0x20) != 0;
                self.nz.vol = val & 0x0F;
            }
            // $400E: noise period (bits 0-3 index NOISE_PER) + mode (bit 7).
            0x400E => {
                // 16398
                self.nz.period = NOISE_PER[(val & 0x0F) as usize]; // low nibble selects period
                self.nz.mode = (val >> 7) & 1; // bit 7 = short/long LFSR mode
            }
            // $400F: noise length index (bits 3-7).
            0x400F => self.nz.len = LEN_TBL[((val >> 3) & 0x1F) as usize], // 16399
            // $4015: status register — per-channel enable bits; disabling a
            // channel immediately zeroes its length counter.
            0x4015 => {
                // 16405
                self.status = val; // cache the raw status byte
                self.p1.enabled = (val & 1) != 0; // bit 0 = pulse 1
                self.p2.enabled = (val & 2) != 0; // bit 1 = pulse 2
                self.tr.enabled = (val & 4) != 0; // bit 2 = triangle
                self.nz.enabled = (val & 8) != 0; // bit 3 = noise
                // Clearing an enable bit forces that channel's length counter to 0.
                if (val & 1) == 0 {
                    self.p1.len = 0;
                }
                if (val & 2) == 0 {
                    self.p2.len = 0;
                }
                if (val & 4) == 0 {
                    self.tr.len = 0;
                }
                if (val & 8) == 0 {
                    self.nz.len = 0;
                }
            }
            // Any other address (sweep regs, DMC, frame counter, etc.) is ignored.
            _ => {}
        }
    }

    /// Clock the frame sequencer's length-counter step once.
    ///
    /// On the 2A03 the frame counter periodically clocks the length counters; here
    /// that is collapsed into a single per-call decrement. Each channel's length
    /// counter ticks down toward zero (which silences the channel) unless it is at
    /// zero already or its halt flag (the triangle's `control` flag) is set.
    pub fn frame(&mut self) {
        // Pulse 1: decrement unless halted or already expired.
        if self.p1.len != 0 && !self.p1.halt {
            self.p1.len -= 1;
        }
        // Pulse 2: decrement unless halted or already expired.
        if self.p2.len != 0 && !self.p2.halt {
            self.p2.len -= 1;
        }
        // Triangle: the control flag doubles as the length-counter halt.
        if self.tr.len != 0 && !self.tr.control {
            self.tr.len -= 1;
        }
        // Noise: decrement unless halted or already expired.
        if self.nz.len != 0 && !self.nz.halt {
            self.nz.len -= 1;
        }
    }

    /// Compute one output sample (in [-1, 1]) for a pulse channel and advance its
    /// phase. Returns silence when the channel is disabled, its length counter has
    /// expired, its period is too low to produce a valid tone (< 8), or its volume
    /// is zero.
    fn pulse_out(p: &mut Pulse) -> f64 {
        // Mute conditions that match hardware silencing rules.
        if !p.enabled || p.len == 0 || p.period < 8 || p.vol == 0 {
            return 0.0;
        }
        // Output frequency: the pulse timer divides the CPU clock by 16*(period+1).
        let f = CPU_HZ / (16.0 * (p.period + 1) as f64);
        // Advance the normalized phase by one sample step at the host rate.
        p.phase += f / APU_SR as f64;
        // Wrap the phase back into [0, 1) by subtracting the integer part.
        p.phase -= p.phase as i32 as f64;
        // Square wave: high for the duty-cycle fraction of the period, else low.
        let lvl = if p.phase < DUTY[p.duty as usize] {
            1.0
        } else {
            -1.0
        };
        // Scale the +/-1 level by the normalized volume (vol/15).
        lvl * (p.vol as f64 / 15.0)
    }

    /// Compute one output sample (in [-1, 1]) for the triangle channel and advance
    /// its phase. Returns silence when disabled, length-expired, the linear counter
    /// is zero, or the period is too low (< 2) to produce a valid tone.
    fn tri_out(&mut self) -> f64 {
        // Mute conditions for the triangle channel.
        if !self.tr.enabled || self.tr.len == 0 || self.tr.linear == 0 || self.tr.period < 2 {
            return 0.0;
        }
        // Output frequency: the triangle timer divides the CPU clock by 32*(period+1).
        let f = CPU_HZ / (32.0 * (self.tr.period + 1) as f64);
        // Advance and wrap the normalized phase.
        self.tr.phase += f / APU_SR as f64;
        self.tr.phase -= self.tr.phase as i32 as f64;
        // Build a 0..1 triangle ramp: rising on the first half of the period,
        // falling on the second half (0.5 = midpoint).
        let t = if self.tr.phase < 0.5 {
            self.tr.phase * 2.0
        } else {
            (1.0 - self.tr.phase) * 2.0
        };
        // Remap the 0..1 ramp into the bipolar [-1, 1] range.
        t * 2.0 - 1.0
    }

    /// Compute one output sample (in [-1, 1]) for the noise channel and advance its
    /// LFSR. Returns silence when disabled, length-expired, or volume is zero.
    fn noise_out(&mut self) -> f64 {
        // Mute conditions for the noise channel.
        if !self.nz.enabled || self.nz.len == 0 || self.nz.vol == 0 {
            return 0.0;
        }
        // Shift rate: one LFSR step per `period` CPU clocks.
        let f = CPU_HZ / self.nz.period as f64;
        // Accumulate sub-shift phase at the host sample rate.
        self.nz.phase += f / APU_SR as f64;
        // Each time a whole shift-period elapses, clock the LFSR once.
        while self.nz.phase >= 1.0 {
            self.nz.phase -= 1.0;
            // Feedback is XOR of bit 0 with either bit 6 (short mode) or bit 1 (long mode).
            let b0 = self.nz.lfsr & 1;
            let b1 = if self.nz.mode != 0 {
                (self.nz.lfsr >> 6) & 1 // short (93-step) mode taps bit 6
            } else {
                (self.nz.lfsr >> 1) & 1 // long (32767-step) mode taps bit 1
            };
            // Shift right and insert the feedback bit at bit 14 (15-bit register).
            self.nz.lfsr = (self.nz.lfsr >> 1) | (((b0 ^ b1) & 1) << 14);
        }
        // Output level is set by the LFSR's low bit, scaled by normalized volume.
        (if (self.nz.lfsr & 1) != 0 { -1.0 } else { 1.0 }) * (self.nz.vol as f64 / 15.0)
    }

    /// Synthesize a buffer of mono 16-bit PCM samples by mixing all channels.
    ///
    /// For each output slot the four channel outputs are summed with empirically
    /// chosen per-channel weights, scaled by a master gain, clamped to [-1, 1],
    /// and quantized to 16-bit signed samples. The mixing coefficients are
    /// non-linear approximations of the real APU mixer.
    pub fn generate(&mut self, out: &mut [i16]) {
        for sample in out {
            // Mix the two pulse channels at unity with weighted triangle and noise.
            let mut s = Self::pulse_out(&mut self.p1)
                + Self::pulse_out(&mut self.p2)
                + 0.8 * self.tri_out() // triangle mix weight
                + 0.6 * self.noise_out(); // noise mix weight
            // Apply master gain to keep the summed signal in headroom.
            s *= 0.22; // master mix gain
            // Hard-limit to the valid normalized amplitude range.
            s = s.clamp(-1.0, 1.0);
            // Scale to 16-bit signed (leaving a little headroom below i16::MAX).
            *sample = (s * 30000.0) as i16; // peak amplitude in PCM units
        }
    }
}

/// Write a sequence of 16-bit mono PCM `samples` to `path` as a canonical
/// little-endian WAV (RIFF/WAVE) file.
///
/// `rate` is the sample rate in Hz. The file uses PCM format (uncompressed),
/// 1 channel, 16 bits per sample. All multi-byte header fields are written
/// little-endian as required by the WAV format.
pub fn wav_write(path: impl AsRef<Path>, samples: &[i16], rate: usize) -> std::io::Result<()> {
    // Create (or truncate) the destination file.
    let mut f = File::create(path)?;
    // Audio payload size in bytes: each sample is 2 bytes (16-bit).
    let data = samples.len() * 2;

    // RIFF chunk descriptor: tag + total file size minus the 8-byte RIFF header.
    f.write_all(b"RIFF")?;
    write_u32(&mut f, 36 + data as u32)?; // 36 = size of all headers after this field
    f.write_all(b"WAVE")?;

    // "fmt " sub-chunk: describes the audio format.
    f.write_all(b"fmt ")?;
    write_u32(&mut f, 16)?; // fmt chunk body size (16 for PCM)
    write_u16(&mut f, 1)?; // audio format code 1 = PCM (uncompressed)
    write_u16(&mut f, 1)?; // number of channels = 1 (mono)
    write_u32(&mut f, rate as u32)?; // sample rate (Hz)
    write_u32(&mut f, (rate * 2) as u32)?; // byte rate = rate * channels * bytesPerSample (1 * 2)
    write_u16(&mut f, 2)?; // block align = channels * bytesPerSample (1 * 2)
    write_u16(&mut f, 16)?; // bits per sample

    // "data" sub-chunk: tag, payload byte count, then the samples themselves.
    f.write_all(b"data")?;
    write_u32(&mut f, data as u32)?;
    // Emit each sample little-endian.
    for s in samples {
        f.write_all(&s.to_le_bytes())?;
    }
    Ok(())
}

/// Write a `u32` to the file in little-endian byte order (WAV header helper).
fn write_u32(f: &mut File, value: u32) -> std::io::Result<()> {
    f.write_all(&value.to_le_bytes())
}

/// Write a `u16` to the file in little-endian byte order (WAV header helper).
fn write_u16(f: &mut File, value: u16) -> std::io::Result<()> {
    f.write_all(&value.to_le_bytes())
}
