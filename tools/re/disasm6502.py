#!/usr/bin/env python3
"""Byte-exact 6502 disassembler for the matching ca65 build.

Given a bank's bytes, its CPU origin, and a set of entry CPU addresses (verified
anchors + coverage-confirmed instruction starts), do recursive-descent decode and
emit ca65 source that re-assembles to the SAME bytes. Unproven bytes stay `.byte`.

Byte-exactness rules:
- absolute modes with operand < $0100 get the `a:` force-absolute prefix.
- in-window control-flow targets that land on an instruction boundary become
  local labels (L_xxxx); everything else is a literal `$nnnn` / `$nn`.
- only the 151 official opcodes decode; anything else terminates the run and the
  bytes fall back to data (coverage should never seed an illegal opcode).
"""
from __future__ import annotations

# opcode -> (mnemonic, mode)
IMP, ACC, IMM, ZP, ZPX, ZPY, IZX, IZY, ABS, ABX, ABY, IND, REL = range(13)
MODE_LEN = {IMP: 1, ACC: 1, IMM: 2, ZP: 2, ZPX: 2, ZPY: 2, IZX: 2, IZY: 2,
            ABS: 3, ABX: 3, ABY: 3, IND: 3, REL: 2}

OPS: dict[int, tuple[str, int]] = {}
def _d(table):
    for op, mn, md in table:
        OPS[op] = (mn, md)
_d([
    (0x69,"ADC",IMM),(0x65,"ADC",ZP),(0x75,"ADC",ZPX),(0x6D,"ADC",ABS),(0x7D,"ADC",ABX),(0x79,"ADC",ABY),(0x61,"ADC",IZX),(0x71,"ADC",IZY),
    (0x29,"AND",IMM),(0x25,"AND",ZP),(0x35,"AND",ZPX),(0x2D,"AND",ABS),(0x3D,"AND",ABX),(0x39,"AND",ABY),(0x21,"AND",IZX),(0x31,"AND",IZY),
    (0x0A,"ASL",ACC),(0x06,"ASL",ZP),(0x16,"ASL",ZPX),(0x0E,"ASL",ABS),(0x1E,"ASL",ABX),
    (0x90,"BCC",REL),(0xB0,"BCS",REL),(0xF0,"BEQ",REL),(0x30,"BMI",REL),(0xD0,"BNE",REL),(0x10,"BPL",REL),(0x50,"BVC",REL),(0x70,"BVS",REL),
    (0x24,"BIT",ZP),(0x2C,"BIT",ABS),
    (0x00,"BRK",IMP),
    (0x18,"CLC",IMP),(0xD8,"CLD",IMP),(0x58,"CLI",IMP),(0xB8,"CLV",IMP),
    (0xC9,"CMP",IMM),(0xC5,"CMP",ZP),(0xD5,"CMP",ZPX),(0xCD,"CMP",ABS),(0xDD,"CMP",ABX),(0xD9,"CMP",ABY),(0xC1,"CMP",IZX),(0xD1,"CMP",IZY),
    (0xE0,"CPX",IMM),(0xE4,"CPX",ZP),(0xEC,"CPX",ABS),
    (0xC0,"CPY",IMM),(0xC4,"CPY",ZP),(0xCC,"CPY",ABS),
    (0xC6,"DEC",ZP),(0xD6,"DEC",ZPX),(0xCE,"DEC",ABS),(0xDE,"DEC",ABX),
    (0xCA,"DEX",IMP),(0x88,"DEY",IMP),
    (0x49,"EOR",IMM),(0x45,"EOR",ZP),(0x55,"EOR",ZPX),(0x4D,"EOR",ABS),(0x5D,"EOR",ABX),(0x59,"EOR",ABY),(0x41,"EOR",IZX),(0x51,"EOR",IZY),
    (0xE6,"INC",ZP),(0xF6,"INC",ZPX),(0xEE,"INC",ABS),(0xFE,"INC",ABX),
    (0xE8,"INX",IMP),(0xC8,"INY",IMP),
    (0x4C,"JMP",ABS),(0x6C,"JMP",IND),(0x20,"JSR",ABS),
    (0xA9,"LDA",IMM),(0xA5,"LDA",ZP),(0xB5,"LDA",ZPX),(0xAD,"LDA",ABS),(0xBD,"LDA",ABX),(0xB9,"LDA",ABY),(0xA1,"LDA",IZX),(0xB1,"LDA",IZY),
    (0xA2,"LDX",IMM),(0xA6,"LDX",ZP),(0xB6,"LDX",ZPY),(0xAE,"LDX",ABS),(0xBE,"LDX",ABY),
    (0xA0,"LDY",IMM),(0xA4,"LDY",ZP),(0xB4,"LDY",ZPX),(0xAC,"LDY",ABS),(0xBC,"LDY",ABX),
    (0x4A,"LSR",ACC),(0x46,"LSR",ZP),(0x56,"LSR",ZPX),(0x4E,"LSR",ABS),(0x5E,"LSR",ABX),
    (0xEA,"NOP",IMP),
    (0x09,"ORA",IMM),(0x05,"ORA",ZP),(0x15,"ORA",ZPX),(0x0D,"ORA",ABS),(0x1D,"ORA",ABX),(0x19,"ORA",ABY),(0x01,"ORA",IZX),(0x11,"ORA",IZY),
    (0x48,"PHA",IMP),(0x08,"PHP",IMP),(0x68,"PLA",IMP),(0x28,"PLP",IMP),
    (0x2A,"ROL",ACC),(0x26,"ROL",ZP),(0x36,"ROL",ZPX),(0x2E,"ROL",ABS),(0x3E,"ROL",ABX),
    (0x6A,"ROR",ACC),(0x66,"ROR",ZP),(0x76,"ROR",ZPX),(0x6E,"ROR",ABS),(0x7E,"ROR",ABX),
    (0x40,"RTI",IMP),(0x60,"RTS",IMP),
    (0xE9,"SBC",IMM),(0xE5,"SBC",ZP),(0xF5,"SBC",ZPX),(0xED,"SBC",ABS),(0xFD,"SBC",ABX),(0xF9,"SBC",ABY),(0xE1,"SBC",IZX),(0xF1,"SBC",IZY),
    (0x38,"SEC",IMP),(0xF8,"SED",IMP),(0x78,"SEI",IMP),
    (0x85,"STA",ZP),(0x95,"STA",ZPX),(0x8D,"STA",ABS),(0x9D,"STA",ABX),(0x99,"STA",ABY),(0x81,"STA",IZX),(0x91,"STA",IZY),
    (0x86,"STX",ZP),(0x96,"STX",ZPY),(0x8E,"STX",ABS),
    (0x84,"STY",ZP),(0x94,"STY",ZPX),(0x8C,"STY",ABS),
    (0xAA,"TAX",IMP),(0xA8,"TAY",IMP),(0xBA,"TSX",IMP),(0x8A,"TXA",IMP),(0x9A,"TXS",IMP),(0x98,"TYA",IMP),
])

# runs end after these
TERMINATORS = {0x4C, 0x6C, 0x60, 0x40, 0x00}  # JMP abs/ind, RTS, RTI, BRK
BRANCHES = {0x90, 0xB0, 0xF0, 0x30, 0xD0, 0x10, 0x50, 0x70}  # conditional rel


class BankDisasm:
    def __init__(self, data: bytes, origin: int, name: str):
        self.data = data
        self.origin = origin
        self.end = origin + len(data)
        self.name = name
        self.starts: set[int] = set()    # cpu addrs that begin an instruction
        self.lengths: dict[int, int] = {}
        self.labels: set[int] = set()    # cpu addrs needing a label

    def _in_window(self, addr: int) -> bool:
        return self.origin <= addr < self.end

    def _byte(self, addr: int) -> int:
        return self.data[addr - self.origin]

    def _word(self, addr: int) -> int:
        return self._byte(addr) | (self._byte(addr + 1) << 8)

    def trace(self, entries: set[int], force_labels: set[int] | None = None):
        for a in (force_labels or ()):
            if self._in_window(a):
                self.labels.add(a)  # named anchors get labels for navigability
        work = [a for a in entries if self._in_window(a)]
        seen = set()
        while work:
            pc = work.pop()
            while True:
                if pc in seen or not self._in_window(pc):
                    break
                op = self._byte(pc)
                info = OPS.get(op)
                if info is None:
                    break  # illegal -> leave as data
                mn, md = info
                ln = MODE_LEN[md]
                if pc + ln > self.end:
                    break  # would run past bank end
                seen.add(pc)
                self.starts.add(pc)
                self.lengths[pc] = ln
                # collect control-flow targets within window
                if md == REL:
                    tgt = (pc + 2 + ((self._byte(pc + 1) ^ 0x80) - 0x80)) & 0xFFFF
                    if self._in_window(tgt):
                        self.labels.add(tgt)
                        work.append(tgt)
                elif op == 0x20 or op == 0x4C:  # JSR / JMP abs
                    tgt = self._word(pc + 1)
                    if self._in_window(tgt):
                        self.labels.add(tgt)
                        work.append(tgt)
                if op in TERMINATORS:
                    break
                pc += ln

    # ---- emit ----
    def _label(self, addr: int) -> str:
        return f"L_{addr:04X}"

    def _operand(self, pc: int, mn: str, md: int) -> str:
        d = self.data
        o = pc - self.origin
        if md in (IMP,):
            return ""
        if md == ACC:
            return "A"
        if md == IMM:
            return f"#${d[o+1]:02X}"
        if md == ZP:
            return f"${d[o+1]:02X}"
        if md == ZPX:
            return f"${d[o+1]:02X},X"
        if md == ZPY:
            return f"${d[o+1]:02X},Y"
        if md == IZX:
            return f"(${d[o+1]:02X},X)"
        if md == IZY:
            return f"(${d[o+1]:02X}),Y"
        if md == REL:
            tgt = (pc + 2 + ((d[o+1] ^ 0x80) - 0x80)) & 0xFFFF
            return self._label(tgt) if tgt in self.starts else f"${tgt:04X}"
        # absolute family
        addr = d[o+1] | (d[o+2] << 8)
        if md == IND:
            return f"(${addr:04X})"
        # pick label or literal; force a: when operand < $0100
        if addr in self.starts and self._in_window(addr):
            base = self._label(addr)
        elif addr < 0x100:
            base = f"a:${addr:04X}"
        else:
            base = f"${addr:04X}"
        if md == ABX:
            return base + ",X"
        if md == ABY:
            return base + ",Y"
        return base  # ABS

    def render(self) -> str:
        out = [f'; PRG bank {self.name} — CPU origin ${self.origin:04X}',
               f'.segment "CODE{self.name}"']
        o = 0
        n = len(self.data)
        pending: list[int] = []

        def flush_data():
            nonlocal pending
            for i in range(0, len(pending), 16):
                chunk = pending[i:i+16]
                out.append("    .byte " + ",".join(f"${b:02X}" for b in chunk))
            pending = []

        while o < n:
            addr = self.origin + o
            if addr in self.starts:
                flush_data()
                if addr in self.labels:
                    out.append(f"{self._label(addr)}:")
                mn, md = OPS[self.data[o]]
                ln = self.lengths[addr]
                operand = self._operand(addr, mn, md)
                txt = f"    {mn}"
                if operand:
                    txt += f" {operand}"
                out.append(txt)
                o += ln
            else:
                pending.append(self.data[o])
                o += 1
        flush_data()
        return "\n".join(out) + "\n"


def disassemble_bank(data: bytes, origin: int, name: str, entries: set[int],
                     force_labels: set[int] | None = None) -> dict:
    bd = BankDisasm(data, origin, name)
    bd.trace(entries, force_labels)
    code_bytes = sum(bd.lengths.values())
    return {"text": bd.render(), "code_bytes": code_bytes,
            "instructions": len(bd.starts), "labels": len(bd.labels)}
