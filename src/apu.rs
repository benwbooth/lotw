use std::{fs::File, io::Write, path::Path};

use crate::engine::APU_SR;

const CPU_HZ: f64 = 1_789_773.0;
const LEN_TBL: [i32; 32] = [
    10, 254, 20, 2, 40, 4, 80, 6, 160, 8, 60, 10, 14, 12, 26, 14, 12, 16, 24, 18, 48, 20, 96, 22,
    192, 24, 72, 26, 16, 28, 32, 30,
];
const NOISE_PER: [i32; 16] = [
    4, 8, 16, 32, 64, 96, 128, 160, 202, 254, 380, 508, 762, 1016, 2034, 4068,
];
const DUTY: [f64; 4] = [0.125, 0.25, 0.5, 0.75];

#[derive(Clone, Copy, Default)]
struct Pulse {
    enabled: bool,
    len: i32,
    period: i32,
    vol: i32,
    duty: i32,
    halt: bool,
    phase: f64,
}

#[derive(Clone, Copy, Default)]
struct Tri {
    enabled: bool,
    len: i32,
    period: i32,
    linear: i32,
    control: bool,
    phase: f64,
}

#[derive(Clone, Copy)]
struct Noise {
    enabled: bool,
    len: i32,
    period: i32,
    vol: i32,
    mode: i32,
    halt: bool,
    lfsr: u32,
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
            lfsr: 1,
            phase: 0.0,
        }
    }
}

#[derive(Default)]
pub struct Apu {
    p1: Pulse,
    p2: Pulse,
    tr: Tri,
    nz: Noise,
    status: i32,
}

impl Apu {
    pub fn new() -> Self {
        let mut apu = Self::default();
        apu.reset();
        apu
    }

    pub fn reset(&mut self) {
        self.p1 = Pulse::default();
        self.p2 = Pulse::default();
        self.tr = Tri::default();
        self.nz = Noise::default();
        self.status = 0;
    }

    pub fn write(&mut self, addr: i32, val: i32) {
        let val = val & 0xff;
        match addr & 0xffff {
            0x4000 => {
                self.p1.duty = val >> 6;
                self.p1.halt = (val & 0x20) != 0;
                self.p1.vol = val & 0x0f;
            }
            0x4001 => {}
            0x4002 => self.p1.period = (self.p1.period & 0x700) | val,
            0x4003 => {
                self.p1.period = (self.p1.period & 0xff) | ((val & 7) << 8);
                self.p1.len = LEN_TBL[((val >> 3) & 0x1f) as usize];
                self.p1.phase = 0.0;
            }
            0x4004 => {
                self.p2.duty = val >> 6;
                self.p2.halt = (val & 0x20) != 0;
                self.p2.vol = val & 0x0f;
            }
            0x4005 => {}
            0x4006 => self.p2.period = (self.p2.period & 0x700) | val,
            0x4007 => {
                self.p2.period = (self.p2.period & 0xff) | ((val & 7) << 8);
                self.p2.len = LEN_TBL[((val >> 3) & 0x1f) as usize];
                self.p2.phase = 0.0;
            }
            0x4008 => {
                self.tr.control = (val & 0x80) != 0;
                self.tr.linear = val & 0x7f;
            }
            0x400a => self.tr.period = (self.tr.period & 0x700) | val,
            0x400b => {
                self.tr.period = (self.tr.period & 0xff) | ((val & 7) << 8);
                self.tr.len = LEN_TBL[((val >> 3) & 0x1f) as usize];
            }
            0x400c => {
                self.nz.halt = (val & 0x20) != 0;
                self.nz.vol = val & 0x0f;
            }
            0x400e => {
                self.nz.period = NOISE_PER[(val & 0x0f) as usize];
                self.nz.mode = (val >> 7) & 1;
            }
            0x400f => self.nz.len = LEN_TBL[((val >> 3) & 0x1f) as usize],
            0x4015 => {
                self.status = val;
                self.p1.enabled = (val & 1) != 0;
                self.p2.enabled = (val & 2) != 0;
                self.tr.enabled = (val & 4) != 0;
                self.nz.enabled = (val & 8) != 0;
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
            _ => {}
        }
    }

    pub fn frame(&mut self) {
        if self.p1.len != 0 && !self.p1.halt {
            self.p1.len -= 1;
        }
        if self.p2.len != 0 && !self.p2.halt {
            self.p2.len -= 1;
        }
        if self.tr.len != 0 && !self.tr.control {
            self.tr.len -= 1;
        }
        if self.nz.len != 0 && !self.nz.halt {
            self.nz.len -= 1;
        }
    }

    fn pulse_out(p: &mut Pulse) -> f64 {
        if !p.enabled || p.len == 0 || p.period < 8 || p.vol == 0 {
            return 0.0;
        }
        let f = CPU_HZ / (16.0 * (p.period + 1) as f64);
        p.phase += f / APU_SR as f64;
        p.phase -= p.phase as i32 as f64;
        let lvl = if p.phase < DUTY[p.duty as usize] {
            1.0
        } else {
            -1.0
        };
        lvl * (p.vol as f64 / 15.0)
    }

    fn tri_out(&mut self) -> f64 {
        if !self.tr.enabled || self.tr.len == 0 || self.tr.linear == 0 || self.tr.period < 2 {
            return 0.0;
        }
        let f = CPU_HZ / (32.0 * (self.tr.period + 1) as f64);
        self.tr.phase += f / APU_SR as f64;
        self.tr.phase -= self.tr.phase as i32 as f64;
        let t = if self.tr.phase < 0.5 {
            self.tr.phase * 2.0
        } else {
            (1.0 - self.tr.phase) * 2.0
        };
        t * 2.0 - 1.0
    }

    fn noise_out(&mut self) -> f64 {
        if !self.nz.enabled || self.nz.len == 0 || self.nz.vol == 0 {
            return 0.0;
        }
        let f = CPU_HZ / self.nz.period as f64;
        self.nz.phase += f / APU_SR as f64;
        while self.nz.phase >= 1.0 {
            self.nz.phase -= 1.0;
            let b0 = self.nz.lfsr & 1;
            let b1 = if self.nz.mode != 0 {
                (self.nz.lfsr >> 6) & 1
            } else {
                (self.nz.lfsr >> 1) & 1
            };
            self.nz.lfsr = (self.nz.lfsr >> 1) | (((b0 ^ b1) & 1) << 14);
        }
        (if (self.nz.lfsr & 1) != 0 { -1.0 } else { 1.0 }) * (self.nz.vol as f64 / 15.0)
    }

    pub fn generate(&mut self, out: &mut [i16]) {
        for sample in out {
            let mut s = Self::pulse_out(&mut self.p1)
                + Self::pulse_out(&mut self.p2)
                + 0.8 * self.tri_out()
                + 0.6 * self.noise_out();
            s *= 0.22;
            s = s.clamp(-1.0, 1.0);
            *sample = (s * 30000.0) as i16;
        }
    }
}

pub fn wav_write(path: impl AsRef<Path>, samples: &[i16], rate: usize) -> std::io::Result<()> {
    let mut f = File::create(path)?;
    let data = samples.len() * 2;
    f.write_all(b"RIFF")?;
    write_u32(&mut f, 36 + data as u32)?;
    f.write_all(b"WAVE")?;
    f.write_all(b"fmt ")?;
    write_u32(&mut f, 16)?;
    write_u16(&mut f, 1)?;
    write_u16(&mut f, 1)?;
    write_u32(&mut f, rate as u32)?;
    write_u32(&mut f, (rate * 2) as u32)?;
    write_u16(&mut f, 2)?;
    write_u16(&mut f, 16)?;
    f.write_all(b"data")?;
    write_u32(&mut f, data as u32)?;
    for s in samples {
        f.write_all(&s.to_le_bytes())?;
    }
    Ok(())
}

fn write_u32(f: &mut File, value: u32) -> std::io::Result<()> {
    f.write_all(&value.to_le_bytes())
}

fn write_u16(f: &mut File, value: u16) -> std::io::Result<()> {
    f.write_all(&value.to_le_bytes())
}
