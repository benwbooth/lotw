pub const FLAG_C: u8 = 0x01;
pub const FLAG_Z: u8 = 0x02;
pub const FLAG_I: u8 = 0x04;
pub const FLAG_D: u8 = 0x08;
pub const FLAG_B: u8 = 0x10;
pub const FLAG_U: u8 = 0x20;
pub const FLAG_V: u8 = 0x40;
pub const FLAG_N: u8 = 0x80;

pub trait Bus {
    fn read(&mut self, addr: u16) -> u8;
    fn write(&mut self, addr: u16, value: u8);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StepResult {
    Ok,
    Stopped,
    UnsupportedOpcode(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cpu6502 {
    pub a: u8,
    pub x: u8,
    pub y: u8,
    pub p: u8,
    pub s: u8,
    pub pc: u16,
    pub cycles: u64,
    pub stopped: bool,
}

impl Default for Cpu6502 {
    fn default() -> Self {
        Self {
            a: 0,
            x: 0,
            y: 0,
            p: FLAG_U | FLAG_I,
            s: 0xfd,
            pc: 0,
            cycles: 0,
            stopped: false,
        }
    }
}

impl Cpu6502 {
    pub fn reset_to_vector<B: Bus>(&mut self, bus: &mut B) {
        *self = Self::default();
        self.pc = read16(bus, 0xfffc);
    }

    pub fn step<B: Bus>(&mut self, bus: &mut B) -> StepResult {
        if self.stopped {
            return StepResult::Stopped;
        }

        let opcode = fetch8(self, bus);
        match opcode {
            0x00 => {
                self.stopped = true;
                self.p |= FLAG_B;
                self.cycles += 7;
                StepResult::Stopped
            }
            0x05 => {
                let zp = fetch8(self, bus) as u16;
                self.a |= read8(bus, zp);
                set_nz(self, self.a);
                self.cycles += 3;
                StepResult::Ok
            }
            0x06 => {
                let zp = fetch8(self, bus) as u16;
                let value = read8(bus, zp);
                let result = value << 1;
                set_carry(self, value & 0x80 != 0);
                write8(bus, zp, result);
                set_nz(self, result);
                self.cycles += 5;
                StepResult::Ok
            }
            0x08 => {
                push8(self, bus, self.p | FLAG_B | FLAG_U);
                self.cycles += 3;
                StepResult::Ok
            }
            0x09 => {
                self.a |= fetch8(self, bus);
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x0a => {
                set_carry(self, self.a & 0x80 != 0);
                self.a <<= 1;
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x0d => {
                let addr = fetch16(self, bus);
                self.a |= read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x10 => {
                branch_if(self, bus, self.p & FLAG_N == 0);
                StepResult::Ok
            }
            0x11 => {
                let addr = fetch_ind_y(self, bus);
                self.a |= read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 5;
                StepResult::Ok
            }
            0x18 => {
                set_carry(self, false);
                self.cycles += 2;
                StepResult::Ok
            }
            0x1d => {
                let addr = fetch_abs_x_read(self, bus);
                self.a |= read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x20 => {
                let addr = fetch16(self, bus);
                push8(self, bus, ((self.pc.wrapping_sub(1)) >> 8) as u8);
                push8(self, bus, self.pc.wrapping_sub(1) as u8);
                self.pc = addr;
                self.cycles += 6;
                StepResult::Ok
            }
            0x24 => {
                let zp = fetch8(self, bus) as u16;
                bit(self, read8(bus, zp));
                self.cycles += 3;
                StepResult::Ok
            }
            0x25 => {
                let zp = fetch8(self, bus) as u16;
                self.a &= read8(bus, zp);
                set_nz(self, self.a);
                self.cycles += 3;
                StepResult::Ok
            }
            0x26 => {
                let zp = fetch8(self, bus) as u16;
                let value = read8(bus, zp);
                let result = (value << 1) | u8::from(self.p & FLAG_C != 0);
                set_carry(self, value & 0x80 != 0);
                write8(bus, zp, result);
                set_nz(self, result);
                self.cycles += 5;
                StepResult::Ok
            }
            0x28 => {
                self.p = (pull8(self, bus) & !FLAG_B) | FLAG_U;
                self.cycles += 4;
                StepResult::Ok
            }
            0x29 => {
                self.a &= fetch8(self, bus);
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x2a => {
                let value = self.a;
                self.a = (value << 1) | u8::from(self.p & FLAG_C != 0);
                set_carry(self, value & 0x80 != 0);
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x2c => {
                let addr = fetch16(self, bus);
                bit(self, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            0x30 => {
                branch_if(self, bus, self.p & FLAG_N != 0);
                StepResult::Ok
            }
            0x38 => {
                set_carry(self, true);
                self.cycles += 2;
                StepResult::Ok
            }
            0x3d => {
                let addr = fetch_abs_x_read(self, bus);
                self.a &= read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x3e => {
                let addr = fetch_abs_x(self, bus);
                let value = read8(bus, addr);
                let result = (value << 1) | u8::from(self.p & FLAG_C != 0);
                set_carry(self, value & 0x80 != 0);
                write8(bus, addr, result);
                set_nz(self, result);
                self.cycles += 7;
                StepResult::Ok
            }
            0x46 => {
                let zp = fetch8(self, bus) as u16;
                let value = read8(bus, zp);
                let result = value >> 1;
                set_carry(self, value & 0x01 != 0);
                write8(bus, zp, result);
                set_nz(self, result);
                self.cycles += 5;
                StepResult::Ok
            }
            0x48 => {
                push8(self, bus, self.a);
                self.cycles += 3;
                StepResult::Ok
            }
            0x49 => {
                self.a ^= fetch8(self, bus);
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x4a => {
                set_carry(self, self.a & 0x01 != 0);
                self.a >>= 1;
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x4c => {
                self.pc = fetch16(self, bus);
                self.cycles += 3;
                StepResult::Ok
            }
            0x50 => {
                branch_if(self, bus, self.p & FLAG_V == 0);
                StepResult::Ok
            }
            0x58 => {
                self.p &= !FLAG_I;
                self.cycles += 2;
                StepResult::Ok
            }
            0x5d => {
                let addr = fetch_abs_x_read(self, bus);
                self.a ^= read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x5e => {
                let addr = fetch_abs_x(self, bus);
                let value = read8(bus, addr);
                let result = value >> 1;
                set_carry(self, value & 0x01 != 0);
                write8(bus, addr, result);
                set_nz(self, result);
                self.cycles += 7;
                StepResult::Ok
            }
            0x60 => {
                let lo = read8(bus, 0x0100 | self.s.wrapping_add(1) as u16);
                let hi = read8(bus, 0x0100 | self.s.wrapping_add(2) as u16);
                self.s = self.s.wrapping_add(2);
                self.pc = u16::from_le_bytes([lo, hi]).wrapping_add(1);
                self.cycles += 6;
                StepResult::Ok
            }
            0x65 => {
                let zp = fetch8(self, bus) as u16;
                adc8(self, read8(bus, zp));
                self.cycles += 3;
                StepResult::Ok
            }
            0x66 => {
                let zp = fetch8(self, bus) as u16;
                let value = read8(bus, zp);
                let result = (value >> 1) | if self.p & FLAG_C != 0 { 0x80 } else { 0 };
                set_carry(self, value & 0x01 != 0);
                write8(bus, zp, result);
                set_nz(self, result);
                self.cycles += 5;
                StepResult::Ok
            }
            0x68 => {
                self.a = pull8(self, bus);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x69 => {
                let value = fetch8(self, bus);
                adc8(self, value);
                self.cycles += 2;
                StepResult::Ok
            }
            0x6a => {
                let value = self.a;
                self.a = (value >> 1) | if self.p & FLAG_C != 0 { 0x80 } else { 0 };
                set_carry(self, value & 0x01 != 0);
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x6c => {
                let ptr = fetch16(self, bus);
                self.pc = read16_indirect_bug(bus, ptr);
                self.cycles += 5;
                StepResult::Ok
            }
            0x70 => {
                branch_if(self, bus, self.p & FLAG_V != 0);
                StepResult::Ok
            }
            0x75 => {
                let addr = fetch_zp_x(self, bus) as u16;
                adc8(self, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            0x78 => {
                self.p |= FLAG_I;
                self.cycles += 2;
                StepResult::Ok
            }
            0x7d => {
                let addr = fetch_abs_x_read(self, bus);
                adc8(self, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            0x84 => {
                let addr = fetch8(self, bus) as u16;
                write8(bus, addr, self.y);
                self.cycles += 3;
                StepResult::Ok
            }
            0x85 => {
                let addr = fetch8(self, bus) as u16;
                write8(bus, addr, self.a);
                self.cycles += 3;
                StepResult::Ok
            }
            0x86 => {
                let addr = fetch8(self, bus) as u16;
                write8(bus, addr, self.x);
                self.cycles += 3;
                StepResult::Ok
            }
            0x88 => {
                self.y = self.y.wrapping_sub(1);
                set_nz(self, self.y);
                self.cycles += 2;
                StepResult::Ok
            }
            0x8a => {
                self.a = self.x;
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x8c => {
                let addr = fetch16(self, bus);
                write8(bus, addr, self.y);
                self.cycles += 4;
                StepResult::Ok
            }
            0x8d => {
                let addr = fetch16(self, bus);
                write8(bus, addr, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x8e => {
                let addr = fetch16(self, bus);
                write8(bus, addr, self.x);
                self.cycles += 4;
                StepResult::Ok
            }
            0x90 => {
                branch_if(self, bus, self.p & FLAG_C == 0);
                StepResult::Ok
            }
            0x91 => {
                let addr = fetch_ind_y(self, bus);
                write8(bus, addr, self.a);
                self.cycles += 6;
                StepResult::Ok
            }
            0x94 => {
                let addr = fetch_zp_x(self, bus) as u16;
                write8(bus, addr, self.y);
                self.cycles += 4;
                StepResult::Ok
            }
            0x95 => {
                let addr = fetch_zp_x(self, bus) as u16;
                write8(bus, addr, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0x96 => {
                let addr = fetch_zp_y(self, bus) as u16;
                write8(bus, addr, self.x);
                self.cycles += 4;
                StepResult::Ok
            }
            0x98 => {
                self.a = self.y;
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0x99 => {
                let addr = fetch_abs_y(self, bus);
                write8(bus, addr, self.a);
                self.cycles += 5;
                StepResult::Ok
            }
            0x9a => {
                self.s = self.x;
                self.cycles += 2;
                StepResult::Ok
            }
            0x9d => {
                let addr = fetch_abs_x(self, bus);
                write8(bus, addr, self.a);
                self.cycles += 5;
                StepResult::Ok
            }
            0xa0 => {
                self.y = fetch8(self, bus);
                set_nz(self, self.y);
                self.cycles += 2;
                StepResult::Ok
            }
            0xa1 => {
                let addr = fetch_ind_x(self, bus);
                self.a = read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 6;
                StepResult::Ok
            }
            0xa2 => {
                self.x = fetch8(self, bus);
                set_nz(self, self.x);
                self.cycles += 2;
                StepResult::Ok
            }
            0xa4 => {
                let zp = fetch8(self, bus) as u16;
                self.y = read8(bus, zp);
                set_nz(self, self.y);
                self.cycles += 3;
                StepResult::Ok
            }
            0xa5 => {
                let zp = fetch8(self, bus) as u16;
                self.a = read8(bus, zp);
                set_nz(self, self.a);
                self.cycles += 3;
                StepResult::Ok
            }
            0xa6 => {
                let zp = fetch8(self, bus) as u16;
                self.x = read8(bus, zp);
                set_nz(self, self.x);
                self.cycles += 3;
                StepResult::Ok
            }
            0xa8 => {
                self.y = self.a;
                set_nz(self, self.y);
                self.cycles += 2;
                StepResult::Ok
            }
            0xa9 => {
                self.a = fetch8(self, bus);
                set_nz(self, self.a);
                self.cycles += 2;
                StepResult::Ok
            }
            0xaa => {
                self.x = self.a;
                set_nz(self, self.x);
                self.cycles += 2;
                StepResult::Ok
            }
            0xad => {
                let addr = fetch16(self, bus);
                self.a = read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0xae => {
                let addr = fetch16(self, bus);
                self.x = read8(bus, addr);
                set_nz(self, self.x);
                self.cycles += 4;
                StepResult::Ok
            }
            0xb0 => {
                branch_if(self, bus, self.p & FLAG_C != 0);
                StepResult::Ok
            }
            0xb1 => {
                let addr = fetch_ind_y(self, bus);
                self.a = read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 5;
                StepResult::Ok
            }
            0xb4 => {
                let addr = fetch_zp_x(self, bus) as u16;
                self.y = read8(bus, addr);
                set_nz(self, self.y);
                self.cycles += 4;
                StepResult::Ok
            }
            0xb5 => {
                let addr = fetch_zp_x(self, bus) as u16;
                self.a = read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0xb6 => {
                let addr = fetch_zp_y(self, bus) as u16;
                self.x = read8(bus, addr);
                set_nz(self, self.x);
                self.cycles += 4;
                StepResult::Ok
            }
            0xb9 => {
                let addr = fetch_abs_y_read(self, bus);
                self.a = read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0xbc => {
                let addr = fetch_abs_x_read(self, bus);
                self.y = read8(bus, addr);
                set_nz(self, self.y);
                self.cycles += 4;
                StepResult::Ok
            }
            0xbd => {
                let addr = fetch_abs_x_read(self, bus);
                self.a = read8(bus, addr);
                set_nz(self, self.a);
                self.cycles += 4;
                StepResult::Ok
            }
            0xc0 => {
                let value = fetch8(self, bus);
                compare8(self, self.y, value);
                self.cycles += 2;
                StepResult::Ok
            }
            0xc4 => {
                let zp = fetch8(self, bus) as u16;
                compare8(self, self.y, read8(bus, zp));
                self.cycles += 3;
                StepResult::Ok
            }
            0xc5 => {
                let zp = fetch8(self, bus) as u16;
                compare8(self, self.a, read8(bus, zp));
                self.cycles += 3;
                StepResult::Ok
            }
            0xc6 => {
                let zp = fetch8(self, bus) as u16;
                let value = read8(bus, zp).wrapping_sub(1);
                write8(bus, zp, value);
                set_nz(self, value);
                self.cycles += 5;
                StepResult::Ok
            }
            0xc8 => {
                self.y = self.y.wrapping_add(1);
                set_nz(self, self.y);
                self.cycles += 2;
                StepResult::Ok
            }
            0xc9 => {
                let value = fetch8(self, bus);
                compare8(self, self.a, value);
                self.cycles += 2;
                StepResult::Ok
            }
            0xca => {
                self.x = self.x.wrapping_sub(1);
                set_nz(self, self.x);
                self.cycles += 2;
                StepResult::Ok
            }
            0xcd => {
                let addr = fetch16(self, bus);
                compare8(self, self.a, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            0xce => {
                let addr = fetch16(self, bus);
                let value = read8(bus, addr).wrapping_sub(1);
                write8(bus, addr, value);
                set_nz(self, value);
                self.cycles += 6;
                StepResult::Ok
            }
            0xd0 => {
                branch_if(self, bus, self.p & FLAG_Z == 0);
                StepResult::Ok
            }
            0xd5 => {
                let addr = fetch_zp_x(self, bus) as u16;
                compare8(self, self.a, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            0xd6 => {
                let addr = fetch_zp_x(self, bus) as u16;
                let value = read8(bus, addr).wrapping_sub(1);
                write8(bus, addr, value);
                set_nz(self, value);
                self.cycles += 6;
                StepResult::Ok
            }
            0xd8 => {
                self.p &= !FLAG_D;
                self.cycles += 2;
                StepResult::Ok
            }
            0xe0 => {
                let value = fetch8(self, bus);
                compare8(self, self.x, value);
                self.cycles += 2;
                StepResult::Ok
            }
            0xe5 => {
                let zp = fetch8(self, bus) as u16;
                sbc8(self, read8(bus, zp));
                self.cycles += 3;
                StepResult::Ok
            }
            0xe6 => {
                let zp = fetch8(self, bus) as u16;
                let value = read8(bus, zp).wrapping_add(1);
                write8(bus, zp, value);
                set_nz(self, value);
                self.cycles += 5;
                StepResult::Ok
            }
            0xe8 => {
                self.x = self.x.wrapping_add(1);
                set_nz(self, self.x);
                self.cycles += 2;
                StepResult::Ok
            }
            0xe9 => {
                let value = fetch8(self, bus);
                sbc8(self, value);
                self.cycles += 2;
                StepResult::Ok
            }
            0xea => {
                self.cycles += 2;
                StepResult::Ok
            }
            0xed => {
                let addr = fetch16(self, bus);
                sbc8(self, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            0xf0 => {
                branch_if(self, bus, self.p & FLAG_Z != 0);
                StepResult::Ok
            }
            0xf6 => {
                let addr = fetch_zp_x(self, bus) as u16;
                let value = read8(bus, addr).wrapping_add(1);
                write8(bus, addr, value);
                set_nz(self, value);
                self.cycles += 6;
                StepResult::Ok
            }
            0xf8 => {
                self.p |= FLAG_D;
                self.cycles += 2;
                StepResult::Ok
            }
            0xfd => {
                let addr = fetch_abs_x_read(self, bus);
                sbc8(self, read8(bus, addr));
                self.cycles += 4;
                StepResult::Ok
            }
            unsupported => {
                self.pc = self.pc.wrapping_sub(1);
                StepResult::UnsupportedOpcode(unsupported)
            }
        }
    }
}

fn read8<B: Bus>(bus: &mut B, addr: u16) -> u8 {
    bus.read(addr)
}

fn write8<B: Bus>(bus: &mut B, addr: u16, value: u8) {
    bus.write(addr, value);
}

fn read16<B: Bus>(bus: &mut B, addr: u16) -> u16 {
    let lo = read8(bus, addr);
    let hi = read8(bus, addr.wrapping_add(1));
    u16::from_le_bytes([lo, hi])
}

fn read16_zp<B: Bus>(bus: &mut B, addr: u8) -> u16 {
    let lo = read8(bus, addr as u16);
    let hi = read8(bus, addr.wrapping_add(1) as u16);
    u16::from_le_bytes([lo, hi])
}

fn read16_indirect_bug<B: Bus>(bus: &mut B, addr: u16) -> u16 {
    let hi_addr = (addr & 0xff00) | (addr.wrapping_add(1) & 0x00ff);
    let lo = read8(bus, addr);
    let hi = read8(bus, hi_addr);
    u16::from_le_bytes([lo, hi])
}

fn fetch8<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u8 {
    let value = read8(bus, cpu.pc);
    cpu.pc = cpu.pc.wrapping_add(1);
    value
}

fn fetch16<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    let lo = fetch8(cpu, bus);
    let hi = fetch8(cpu, bus);
    u16::from_le_bytes([lo, hi])
}

fn fetch_zp_x<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u8 {
    fetch8(cpu, bus).wrapping_add(cpu.x)
}

fn fetch_zp_y<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u8 {
    fetch8(cpu, bus).wrapping_add(cpu.y)
}

fn fetch_abs_x<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    fetch16(cpu, bus).wrapping_add(cpu.x as u16)
}

fn fetch_abs_y<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    fetch16(cpu, bus).wrapping_add(cpu.y as u16)
}

fn fetch_abs_x_read<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    let base = fetch16(cpu, bus);
    let addr = base.wrapping_add(cpu.x as u16);
    if base & 0xff00 != addr & 0xff00 {
        cpu.cycles += 1;
    }
    addr
}

fn fetch_abs_y_read<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    let base = fetch16(cpu, bus);
    let addr = base.wrapping_add(cpu.y as u16);
    if base & 0xff00 != addr & 0xff00 {
        cpu.cycles += 1;
    }
    addr
}

fn fetch_ind_x<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    let zp = fetch8(cpu, bus).wrapping_add(cpu.x);
    read16_zp(bus, zp)
}

fn fetch_ind_y<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u16 {
    let zp = fetch8(cpu, bus);
    read16_zp(bus, zp).wrapping_add(cpu.y as u16)
}

fn set_nz(cpu: &mut Cpu6502, value: u8) {
    if value == 0 {
        cpu.p |= FLAG_Z;
    } else {
        cpu.p &= !FLAG_Z;
    }

    if value & 0x80 != 0 {
        cpu.p |= FLAG_N;
    } else {
        cpu.p &= !FLAG_N;
    }
}

fn set_carry(cpu: &mut Cpu6502, enabled: bool) {
    if enabled {
        cpu.p |= FLAG_C;
    } else {
        cpu.p &= !FLAG_C;
    }
}

fn set_overflow(cpu: &mut Cpu6502, enabled: bool) {
    if enabled {
        cpu.p |= FLAG_V;
    } else {
        cpu.p &= !FLAG_V;
    }
}

fn adc8(cpu: &mut Cpu6502, value: u8) {
    let carry = u16::from(cpu.p & FLAG_C != 0);
    let sum = cpu.a as u16 + value as u16 + carry;
    let result = sum as u8;
    set_carry(cpu, sum > 0xff);
    set_overflow(cpu, ((cpu.a ^ result) & (value ^ result) & 0x80) != 0);
    cpu.a = result;
    set_nz(cpu, cpu.a);
}

fn sbc8(cpu: &mut Cpu6502, value: u8) {
    adc8(cpu, value ^ 0xff);
}

fn compare8(cpu: &mut Cpu6502, lhs: u8, rhs: u8) {
    let result = lhs.wrapping_sub(rhs);
    set_carry(cpu, lhs >= rhs);
    set_nz(cpu, result);
}

fn bit(cpu: &mut Cpu6502, value: u8) {
    if cpu.a & value == 0 {
        cpu.p |= FLAG_Z;
    } else {
        cpu.p &= !FLAG_Z;
    }
    cpu.p = (cpu.p & !(FLAG_N | FLAG_V)) | (value & (FLAG_N | FLAG_V));
}

fn branch_if<B: Bus>(cpu: &mut Cpu6502, bus: &mut B, condition: bool) {
    let offset = fetch8(cpu, bus) as i8;
    cpu.cycles += 2;
    if condition {
        let old_pc = cpu.pc;
        cpu.pc = cpu.pc.wrapping_add_signed(offset as i16);
        cpu.cycles += 1;
        if old_pc & 0xff00 != cpu.pc & 0xff00 {
            cpu.cycles += 1;
        }
    }
}

fn push8<B: Bus>(cpu: &mut Cpu6502, bus: &mut B, value: u8) {
    write8(bus, 0x0100 | cpu.s as u16, value);
    cpu.s = cpu.s.wrapping_sub(1);
}

fn pull8<B: Bus>(cpu: &mut Cpu6502, bus: &mut B) -> u8 {
    cpu.s = cpu.s.wrapping_add(1);
    read8(bus, 0x0100 | cpu.s as u16)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Mem([u8; 65536]);

    impl Bus for Mem {
        fn read(&mut self, addr: u16) -> u8 {
            self.0[addr as usize]
        }

        fn write(&mut self, addr: u16, value: u8) {
            self.0[addr as usize] = value;
        }
    }

    #[test]
    fn branch_cycles_match_6502_page_crossing_rules() {
        let mut mem = Mem([0; 65536]);
        mem.0[0x80fe] = 0xd0;
        mem.0[0x80ff] = 0x80;
        let mut cpu = Cpu6502 {
            pc: 0x80fe,
            p: FLAG_U,
            ..Cpu6502::default()
        };

        assert_eq!(cpu.step(&mut mem), StepResult::Ok);
        assert_eq!(cpu.pc, 0x8080);
        assert_eq!(cpu.cycles, 4);
    }

    #[test]
    fn jsr_and_rts_round_trip_stack_address() {
        let mut mem = Mem([0; 65536]);
        mem.0[0x8000] = 0x20;
        mem.0[0x8001] = 0x00;
        mem.0[0x8002] = 0x90;
        mem.0[0x9000] = 0x60;
        let mut cpu = Cpu6502 {
            pc: 0x8000,
            ..Cpu6502::default()
        };

        assert_eq!(cpu.step(&mut mem), StepResult::Ok);
        assert_eq!(cpu.pc, 0x9000);
        assert_eq!(cpu.s, 0xfb);
        assert_eq!(cpu.step(&mut mem), StepResult::Ok);
        assert_eq!(cpu.pc, 0x8003);
        assert_eq!(cpu.s, 0xfd);
    }
}
