#!/usr/bin/env python3
"""Compact, correct 6502 interpreter — the differential-testing ORACLE.

Runs an ORIGINAL routine from the ROM in isolation (no interrupts, exact game
memory map) against an injected machine state, so a C reimplementation can be
proven behaviourally identical. NES 2A03 has no decimal mode, so ADC/SBC are
always binary. Only the 151 official opcodes are implemented (the game uses no
illegals; an unknown opcode raises).

Memory model: a flat 64 KiB bytearray. The caller maps ROM banks into it
(fixed banks 14/15 at $C000-$FFFF; a chosen swappable bank at $8000 or $A000)
and seeds RAM ($0000-$07FF). run_routine() pushes a sentinel return address,
sets PC, and executes until that RTS returns.
"""
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
ROM = ROOT / "rom" / "lotw.nes"
PRG_BASE = 0x10
BANK = 0x2000

C, Z, I, D, B, U, V, N = 1, 2, 4, 8, 16, 32, 64, 128


class CPU:
    def __init__(self):
        self.mem = bytearray(0x10000)
        self.a = self.x = self.y = 0
        self.s = 0xFD
        self.p = U | I
        self.pc = 0
        self.cycles = 0
        self.ppu_status = None      # opt-in: PPUSTATUS read value (else flat memory)

    # ---- ROM mapping ----
    def map_fixed(self, rom: bytes):
        # banks 14+15 -> $C000-$FFFF
        self.mem[0xC000:0x10000] = rom[PRG_BASE + 14 * BANK: PRG_BASE + 16 * BANK]

    def map_bank(self, rom: bytes, bank: int, origin: int):
        self.mem[origin:origin + BANK] = rom[PRG_BASE + bank * BANK: PRG_BASE + (bank + 1) * BANK]

    # ---- helpers ----
    def _set_zn(self, v):
        self.p = (self.p & ~(Z | N)) | (Z if v == 0 else 0) | (v & N)

    def _push(self, v):
        self.mem[0x100 + self.s] = v & 0xFF
        self.s = (self.s - 1) & 0xFF

    def _pop(self):
        self.s = (self.s + 1) & 0xFF
        return self.mem[0x100 + self.s]

    def _rd(self, a):
        a &= 0xFFFF
        # Opt-in PPUSTATUS model (only when a routine's spec sets it) so
        # sprite-0/vblank polls terminate; otherwise pure flat memory.
        if self.ppu_status is not None and 0x2002 <= a < 0x4000 and (a & 7) == 2:
            return self.ppu_status
        return self.mem[a]

    def _wr(self, a, v):
        self.mem[a & 0xFFFF] = v & 0xFF

    def _rdw(self, a):
        return self._rd(a) | (self._rd((a + 1) & 0xFFFF) << 8)

    # operand address by mode
    def _imm(self):
        a = self.pc; self.pc += 1; return a

    def _zp(self):
        a = self._rd(self.pc); self.pc += 1; return a

    def _zpx(self):
        a = (self._rd(self.pc) + self.x) & 0xFF; self.pc += 1; return a

    def _zpy(self):
        a = (self._rd(self.pc) + self.y) & 0xFF; self.pc += 1; return a

    def _abs(self):
        a = self._rdw(self.pc); self.pc += 2; return a

    def _abx(self):
        a = (self._rdw(self.pc) + self.x) & 0xFFFF; self.pc += 2; return a

    def _aby(self):
        a = (self._rdw(self.pc) + self.y) & 0xFFFF; self.pc += 2; return a

    def _izx(self):
        z = (self._rd(self.pc) + self.x) & 0xFF; self.pc += 1
        return self._rd(z) | (self._rd((z + 1) & 0xFF) << 8)

    def _izy(self):
        z = self._rd(self.pc); self.pc += 1
        base = self._rd(z) | (self._rd((z + 1) & 0xFF) << 8)
        return (base + self.y) & 0xFFFF

    def _branch(self, cond):
        off = self._rd(self.pc); self.pc += 1
        if cond:
            self.pc = (self.pc + ((off ^ 0x80) - 0x80)) & 0xFFFF

    def _adc(self, m):
        a = self.a
        r = a + m + (1 if self.p & C else 0)
        self.p &= ~(C | V | Z | N)
        if r > 0xFF: self.p |= C
        if (~(a ^ m) & (a ^ r) & 0x80): self.p |= V
        self.a = r & 0xFF
        self._set_zn(self.a)

    def _sbc(self, m):
        self._adc(m ^ 0xFF)

    def _cmp(self, reg, m):
        r = (reg - m) & 0x1FF
        self.p &= ~(C | Z | N)
        if reg >= m: self.p |= C
        self._set_zn(r & 0xFF)

    def step(self):
        op = self._rd(self.pc); self.pc += 1
        self.cycles += 1
        f = OPS.get(op)
        if f is None:
            raise ValueError(f"unimplemented opcode ${op:02X} at ${(self.pc-1)&0xFFFF:04X}")
        f(self)

    def run_routine(self, pc, a=0, x=0, y=0, p=(U | I), s=0xFD, max_steps=200000,
                    vram_sync=False, sync_clear=None):
        """Push a sentinel return address; run the routine until it RTSes back.
        sync_clear (opt-in): zero-page addresses the NMI would clear/decrement;
        once past a soft step limit they are zeroed so a frame-sync wait-loop on
        them terminates. vram_sync=True is shorthand for sync_clear=[0x28]."""
        sc = sync_clear if sync_clear is not None else ([0x28] if vram_sync else [])
        self.a, self.x, self.y, self.p, self.s = a & 0xFF, x & 0xFF, y & 0xFF, p, s & 0xFF
        SENT = 0x0FFE  # sentinel return-1 (RTS will jump to SENT+1)
        self._push((SENT >> 8) & 0xFF)
        self._push(SENT & 0xFF)
        self.pc = pc
        for i in range(max_steps):
            if self.pc == (SENT + 1) & 0xFFFF:
                return
            if sc and i >= 4000:
                for addr in sc:
                    self.mem[addr] = 0   # NMI consumed/decremented this sync var
            self.step()
        raise RuntimeError("routine did not return within step budget")


# ---- opcode implementations ----
def _ld(reg):
    def f(c, am): setattr(c, reg, c._rd(am(c))); c._set_zn(getattr(c, reg))
    return f


OPS = {}


def _op(code, fn):
    OPS[code] = fn


def build():
    A = CPU
    # loads
    for code, mode in [(0xA9, '_imm'), (0xA5, '_zp'), (0xB5, '_zpx'), (0xAD, '_abs'),
                       (0xBD, '_abx'), (0xB9, '_aby'), (0xA1, '_izx'), (0xB1, '_izy')]:
        _op(code, (lambda m: (lambda c: (setattr(c, 'a', c._rd(getattr(c, m)())), c._set_zn(c.a))))(mode))
    for code, mode in [(0xA2, '_imm'), (0xA6, '_zp'), (0xB6, '_zpy'), (0xAE, '_abs'), (0xBE, '_aby')]:
        _op(code, (lambda m: (lambda c: (setattr(c, 'x', c._rd(getattr(c, m)())), c._set_zn(c.x))))(mode))
    for code, mode in [(0xA0, '_imm'), (0xA4, '_zp'), (0xB4, '_zpx'), (0xAC, '_abs'), (0xBC, '_abx')]:
        _op(code, (lambda m: (lambda c: (setattr(c, 'y', c._rd(getattr(c, m)())), c._set_zn(c.y))))(mode))
    # stores
    for code, mode in [(0x85, '_zp'), (0x95, '_zpx'), (0x8D, '_abs'), (0x9D, '_abx'),
                       (0x99, '_aby'), (0x81, '_izx'), (0x91, '_izy')]:
        _op(code, (lambda m: (lambda c: c._wr(getattr(c, m)(), c.a)))(mode))
    for code, mode in [(0x86, '_zp'), (0x96, '_zpy'), (0x8E, '_abs')]:
        _op(code, (lambda m: (lambda c: c._wr(getattr(c, m)(), c.x)))(mode))
    for code, mode in [(0x84, '_zp'), (0x94, '_zpx'), (0x8C, '_abs')]:
        _op(code, (lambda m: (lambda c: c._wr(getattr(c, m)(), c.y)))(mode))
    # transfers
    _op(0xAA, lambda c: (setattr(c, 'x', c.a), c._set_zn(c.x)))
    _op(0xA8, lambda c: (setattr(c, 'y', c.a), c._set_zn(c.y)))
    _op(0x8A, lambda c: (setattr(c, 'a', c.x), c._set_zn(c.a)))
    _op(0x98, lambda c: (setattr(c, 'a', c.y), c._set_zn(c.a)))
    _op(0xBA, lambda c: (setattr(c, 'x', c.s), c._set_zn(c.x)))
    _op(0x9A, lambda c: setattr(c, 's', c.x))
    # stack
    _op(0x48, lambda c: c._push(c.a))
    _op(0x68, lambda c: (setattr(c, 'a', c._pop()), c._set_zn(c.a)))
    _op(0x08, lambda c: c._push(c.p | B | U))
    _op(0x28, lambda c: setattr(c, 'p', (c._pop() & ~B) | U))
    # logic
    for code, mode in [(0x29, '_imm'), (0x25, '_zp'), (0x35, '_zpx'), (0x2D, '_abs'),
                       (0x3D, '_abx'), (0x39, '_aby'), (0x21, '_izx'), (0x31, '_izy')]:
        _op(code, (lambda m: (lambda c: (setattr(c, 'a', c.a & c._rd(getattr(c, m)())), c._set_zn(c.a))))(mode))
    for code, mode in [(0x09, '_imm'), (0x05, '_zp'), (0x15, '_zpx'), (0x0D, '_abs'),
                       (0x1D, '_abx'), (0x19, '_aby'), (0x01, '_izx'), (0x11, '_izy')]:
        _op(code, (lambda m: (lambda c: (setattr(c, 'a', c.a | c._rd(getattr(c, m)())), c._set_zn(c.a))))(mode))
    for code, mode in [(0x49, '_imm'), (0x45, '_zp'), (0x55, '_zpx'), (0x4D, '_abs'),
                       (0x5D, '_abx'), (0x59, '_aby'), (0x41, '_izx'), (0x51, '_izy')]:
        _op(code, (lambda m: (lambda c: (setattr(c, 'a', c.a ^ c._rd(getattr(c, m)())), c._set_zn(c.a))))(mode))
    # bit
    for code, mode in [(0x24, '_zp'), (0x2C, '_abs')]:
        def _bit(c, m=mode):
            v = c._rd(getattr(c, m)())
            c.p = (c.p & ~(Z | N | V)) | (Z if (c.a & v) == 0 else 0) | (v & (N | V))
        _op(code, _bit)
    # arithmetic
    for code, mode in [(0x69, '_imm'), (0x65, '_zp'), (0x75, '_zpx'), (0x6D, '_abs'),
                       (0x7D, '_abx'), (0x79, '_aby'), (0x61, '_izx'), (0x71, '_izy')]:
        _op(code, (lambda m: (lambda c: c._adc(c._rd(getattr(c, m)()))))(mode))
    for code, mode in [(0xE9, '_imm'), (0xE5, '_zp'), (0xF5, '_zpx'), (0xED, '_abs'),
                       (0xFD, '_abx'), (0xF9, '_aby'), (0xE1, '_izx'), (0xF1, '_izy')]:
        _op(code, (lambda m: (lambda c: c._sbc(c._rd(getattr(c, m)()))))(mode))
    for code, mode in [(0xC9, '_imm'), (0xC5, '_zp'), (0xD5, '_zpx'), (0xCD, '_abs'),
                       (0xDD, '_abx'), (0xD9, '_aby'), (0xC1, '_izx'), (0xD1, '_izy')]:
        _op(code, (lambda m: (lambda c: c._cmp(c.a, c._rd(getattr(c, m)()))))(mode))
    for code, mode in [(0xE0, '_imm'), (0xE4, '_zp'), (0xEC, '_abs')]:
        _op(code, (lambda m: (lambda c: c._cmp(c.x, c._rd(getattr(c, m)()))))(mode))
    for code, mode in [(0xC0, '_imm'), (0xC4, '_zp'), (0xCC, '_abs')]:
        _op(code, (lambda m: (lambda c: c._cmp(c.y, c._rd(getattr(c, m)()))))(mode))
    # inc/dec
    for code, mode in [(0xE6, '_zp'), (0xF6, '_zpx'), (0xEE, '_abs'), (0xFE, '_abx')]:
        def _inc(c, m=mode):
            a = getattr(c, m)(); v = (c._rd(a) + 1) & 0xFF; c._wr(a, v); c._set_zn(v)
        _op(code, _inc)
    for code, mode in [(0xC6, '_zp'), (0xD6, '_zpx'), (0xCE, '_abs'), (0xDE, '_abx')]:
        def _dec(c, m=mode):
            a = getattr(c, m)(); v = (c._rd(a) - 1) & 0xFF; c._wr(a, v); c._set_zn(v)
        _op(code, _dec)
    _op(0xE8, lambda c: (setattr(c, 'x', (c.x + 1) & 0xFF), c._set_zn(c.x)))
    _op(0xC8, lambda c: (setattr(c, 'y', (c.y + 1) & 0xFF), c._set_zn(c.y)))
    _op(0xCA, lambda c: (setattr(c, 'x', (c.x - 1) & 0xFF), c._set_zn(c.x)))
    _op(0x88, lambda c: (setattr(c, 'y', (c.y - 1) & 0xFF), c._set_zn(c.y)))
    # shifts (accumulator + memory)
    def _asl_v(c, v):
        c.p = (c.p & ~C) | (C if v & 0x80 else 0); v = (v << 1) & 0xFF; c._set_zn(v); return v
    def _lsr_v(c, v):
        c.p = (c.p & ~C) | (C if v & 1 else 0); v >>= 1; c._set_zn(v); return v
    def _rol_v(c, v):
        cin = 1 if c.p & C else 0; c.p = (c.p & ~C) | (C if v & 0x80 else 0)
        v = ((v << 1) | cin) & 0xFF; c._set_zn(v); return v
    def _ror_v(c, v):
        cin = 0x80 if c.p & C else 0; c.p = (c.p & ~C) | (C if v & 1 else 0)
        v = (v >> 1) | cin; c._set_zn(v); return v
    for code, mode, fn in [(0x06, '_zp', _asl_v), (0x16, '_zpx', _asl_v), (0x0E, '_abs', _asl_v), (0x1E, '_abx', _asl_v),
                           (0x46, '_zp', _lsr_v), (0x56, '_zpx', _lsr_v), (0x4E, '_abs', _lsr_v), (0x5E, '_abx', _lsr_v),
                           (0x26, '_zp', _rol_v), (0x36, '_zpx', _rol_v), (0x2E, '_abs', _rol_v), (0x3E, '_abx', _rol_v),
                           (0x66, '_zp', _ror_v), (0x76, '_zpx', _ror_v), (0x6E, '_abs', _ror_v), (0x7E, '_abx', _ror_v)]:
        def _rmw(c, m=mode, fn=fn):
            a = getattr(c, m)(); c._wr(a, fn(c, c._rd(a)))
        _op(code, _rmw)
    _op(0x0A, lambda c: setattr(c, 'a', _asl_v(c, c.a)))
    _op(0x4A, lambda c: setattr(c, 'a', _lsr_v(c, c.a)))
    _op(0x2A, lambda c: setattr(c, 'a', _rol_v(c, c.a)))
    _op(0x6A, lambda c: setattr(c, 'a', _ror_v(c, c.a)))
    # branches
    _op(0x90, lambda c: c._branch(not c.p & C))
    _op(0xB0, lambda c: c._branch(c.p & C))
    _op(0xD0, lambda c: c._branch(not c.p & Z))
    _op(0xF0, lambda c: c._branch(c.p & Z))
    _op(0x10, lambda c: c._branch(not c.p & N))
    _op(0x30, lambda c: c._branch(c.p & N))
    _op(0x50, lambda c: c._branch(not c.p & V))
    _op(0x70, lambda c: c._branch(c.p & V))
    # jumps / calls
    _op(0x4C, lambda c: setattr(c, 'pc', c._rdw(c.pc)))
    def _jmp_ind(c):
        a = c._rdw(c.pc)
        # 6502 indirect JMP page-wrap bug
        lo = c._rd(a); hi = c._rd((a & 0xFF00) | ((a + 1) & 0xFF))
        c.pc = lo | (hi << 8)
    _op(0x6C, _jmp_ind)
    def _jsr(c):
        a = c._rdw(c.pc); ret = (c.pc + 1) & 0xFFFF
        c._push((ret >> 8) & 0xFF); c._push(ret & 0xFF); c.pc = a
    _op(0x20, _jsr)
    def _rts(c):
        lo = c._pop(); hi = c._pop(); c.pc = ((lo | (hi << 8)) + 1) & 0xFFFF
    _op(0x60, _rts)
    def _rti(c):
        c.p = (c._pop() & ~B) | U; lo = c._pop(); hi = c._pop(); c.pc = lo | (hi << 8)
    _op(0x40, _rti)
    # flags / nop
    _op(0x18, lambda c: setattr(c, 'p', c.p & ~C))
    _op(0x38, lambda c: setattr(c, 'p', c.p | C))
    _op(0x58, lambda c: setattr(c, 'p', c.p & ~I))
    _op(0x78, lambda c: setattr(c, 'p', c.p | I))
    _op(0xB8, lambda c: setattr(c, 'p', c.p & ~V))
    _op(0xD8, lambda c: setattr(c, 'p', c.p & ~D))
    _op(0xF8, lambda c: setattr(c, 'p', c.p | D))
    _op(0xEA, lambda c: None)
    _op(0x00, lambda c: (_ for _ in ()).throw(RuntimeError("BRK")))


build()
assert len(OPS) >= 151, f"only {len(OPS)} opcodes built"
