#!/usr/bin/env python3
"""Minimal but complete 6502 disassembler for the LotW PRG ROM (MMC3, 16x8KB).

Usage:
  dis6502.py <start_hex> [end_hex]      # disassemble the fixed-bank window ($C000/$E000)
  dis6502.py bank <n> [base_hex]        # disassemble 8KB bank n at base (default $8000)
The ROM path defaults to rom/lotw.nes (run from repo root) or $LOTW_ROM.
"""
import sys, os

ROM = os.environ.get("LOTW_ROM", "rom/lotw.nes")
data = open(ROM, "rb").read()
PRG = data[16:16 + 128 * 1024]  # 16-byte iNES header, then 128KB PRG

# opcode -> (mnemonic, mode). modes: imp,acc,imm,zp,zpx,zpy,abs,abx,aby,ind,izx,izy,rel
T = {}
def d(o, m, mode): T[o] = (m, mode)
# load/store
for o,mode in [(0xA9,'imm'),(0xA5,'zp'),(0xB5,'zpx'),(0xAD,'abs'),(0xBD,'abx'),(0xB9,'aby'),(0xA1,'izx'),(0xB1,'izy')]: d(o,'LDA',mode)
for o,mode in [(0xA2,'imm'),(0xA6,'zp'),(0xB6,'zpy'),(0xAE,'abs'),(0xBE,'aby')]: d(o,'LDX',mode)
for o,mode in [(0xA0,'imm'),(0xA4,'zp'),(0xB4,'zpx'),(0xAC,'abs'),(0xBC,'abx')]: d(o,'LDY',mode)
for o,mode in [(0x85,'zp'),(0x95,'zpx'),(0x8D,'abs'),(0x9D,'abx'),(0x99,'aby'),(0x81,'izx'),(0x91,'izy')]: d(o,'STA',mode)
for o,mode in [(0x86,'zp'),(0x96,'zpy'),(0x8E,'abs')]: d(o,'STX',mode)
for o,mode in [(0x84,'zp'),(0x94,'zpx'),(0x8C,'abs')]: d(o,'STY',mode)
# arithmetic / logic
for o,mode in [(0x69,'imm'),(0x65,'zp'),(0x75,'zpx'),(0x6D,'abs'),(0x7D,'abx'),(0x79,'aby'),(0x61,'izx'),(0x71,'izy')]: d(o,'ADC',mode)
for o,mode in [(0xE9,'imm'),(0xE5,'zp'),(0xF5,'zpx'),(0xED,'abs'),(0xFD,'abx'),(0xF9,'aby'),(0xE1,'izx'),(0xF1,'izy')]: d(o,'SBC',mode)
for o,mode in [(0x29,'imm'),(0x25,'zp'),(0x35,'zpx'),(0x2D,'abs'),(0x3D,'abx'),(0x39,'aby'),(0x21,'izx'),(0x31,'izy')]: d(o,'AND',mode)
for o,mode in [(0x09,'imm'),(0x05,'zp'),(0x15,'zpx'),(0x0D,'abs'),(0x1D,'abx'),(0x19,'aby'),(0x01,'izx'),(0x11,'izy')]: d(o,'ORA',mode)
for o,mode in [(0x49,'imm'),(0x45,'zp'),(0x55,'zpx'),(0x4D,'abs'),(0x5D,'abx'),(0x59,'aby'),(0x41,'izx'),(0x51,'izy')]: d(o,'EOR',mode)
for o,mode in [(0xC9,'imm'),(0xC5,'zp'),(0xD5,'zpx'),(0xCD,'abs'),(0xDD,'abx'),(0xD9,'aby'),(0xC1,'izx'),(0xD1,'izy')]: d(o,'CMP',mode)
for o,mode in [(0xE0,'imm'),(0xE4,'zp'),(0xEC,'abs')]: d(o,'CPX',mode)
for o,mode in [(0xC0,'imm'),(0xC4,'zp'),(0xCC,'abs')]: d(o,'CPY',mode)
for o,mode in [(0x24,'zp'),(0x2C,'abs')]: d(o,'BIT',mode)
# shifts
for o,mode in [(0x0A,'acc'),(0x06,'zp'),(0x16,'zpx'),(0x0E,'abs'),(0x1E,'abx')]: d(o,'ASL',mode)
for o,mode in [(0x4A,'acc'),(0x46,'zp'),(0x56,'zpx'),(0x4E,'abs'),(0x5E,'abx')]: d(o,'LSR',mode)
for o,mode in [(0x2A,'acc'),(0x26,'zp'),(0x36,'zpx'),(0x2E,'abs'),(0x3E,'abx')]: d(o,'ROL',mode)
for o,mode in [(0x6A,'acc'),(0x66,'zp'),(0x76,'zpx'),(0x6E,'abs'),(0x7E,'abx')]: d(o,'ROR',mode)
# inc/dec
for o,mode in [(0xE6,'zp'),(0xF6,'zpx'),(0xEE,'abs'),(0xFE,'abx')]: d(o,'INC',mode)
for o,mode in [(0xC6,'zp'),(0xD6,'zpx'),(0xCE,'abs'),(0xDE,'abx')]: d(o,'DEC',mode)
# branches (relative)
for o,m in [(0x10,'BPL'),(0x30,'BMI'),(0x50,'BVC'),(0x70,'BVS'),(0x90,'BCC'),(0xB0,'BCS'),(0xD0,'BNE'),(0xF0,'BEQ')]: d(o,m,'rel')
# jumps
d(0x4C,'JMP','abs'); d(0x6C,'JMP','ind'); d(0x20,'JSR','abs')
# implied
for o,m in [(0x60,'RTS'),(0x40,'RTI'),(0x00,'BRK'),(0xEA,'NOP'),(0x18,'CLC'),(0x38,'SEC'),
            (0x58,'CLI'),(0x78,'SEI'),(0xB8,'CLV'),(0xD8,'CLD'),(0xF8,'SED'),(0xAA,'TAX'),
            (0x8A,'TXA'),(0xCA,'DEX'),(0xE8,'INX'),(0xA8,'TAY'),(0x98,'TYA'),(0x88,'DEY'),
            (0xC8,'INY'),(0x9A,'TXS'),(0xBA,'TSX'),(0x48,'PHA'),(0x68,'PLA'),(0x08,'PHP'),(0x28,'PLP')]:
    d(o,m,'imp')

SIZE={'imp':1,'acc':1,'imm':2,'zp':2,'zpx':2,'zpy':2,'rel':2,'izx':2,'izy':2,'abs':3,'abx':3,'aby':3,'ind':3}

def fmt(mn, mode, b, addr):
    if mode in('imp',): return mn
    if mode=='acc': return f'{mn} A'
    if mode=='imm': return f'{mn} #${b[1]:02X}'
    if mode=='zp':  return f'{mn} ${b[1]:02X}'
    if mode=='zpx': return f'{mn} ${b[1]:02X},X'
    if mode=='zpy': return f'{mn} ${b[1]:02X},Y'
    if mode=='izx': return f'{mn} (${b[1]:02X},X)'
    if mode=='izy': return f'{mn} (${b[1]:02X}),Y'
    if mode=='rel':
        off=b[1]-256 if b[1]>127 else b[1]; return f'{mn} ${addr+2+off:04X}'
    w=b[1]|(b[2]<<8)
    if mode=='abs': return f'{mn} ${w:04X}'
    if mode=='abx': return f'{mn} ${w:04X},X'
    if mode=='aby': return f'{mn} ${w:04X},Y'
    if mode=='ind': return f'{mn} (${w:04X})'

def disasm(mem, base, start, end):
    out=[]; a=start
    while a<end:
        op=mem[a-base]
        if op in T:
            mn,mode=T[op]; n=SIZE[mode]
            b=mem[a-base:a-base+n]
            if len(b)<n: out.append(f'{a:04X}: .byte ${op:02X}'); a+=1; continue
            out.append(f'{a:04X}: {fmt(mn,mode,b,a)}'); a+=n
        else:
            out.append(f'{a:04X}: .byte ${op:02X}'); a+=1
    return '\n'.join(out)

if __name__=='__main__':
    if sys.argv[1]=='bank':
        n=int(sys.argv[2]); base=int(sys.argv[3],16) if len(sys.argv)>3 else 0x8000
        blk=PRG[n*0x2000:(n+1)*0x2000]
        print(disasm(blk, base, base, base+len(blk)))
    else:
        start=int(sys.argv[1],16); end=int(sys.argv[2],16) if len(sys.argv)>2 else start+64
        # fixed-bank window: $C000-$DFFF = bank 14, $E000-$FFFF = bank 15
        if start>=0xE000: bank=15; base=0xE000
        elif start>=0xC000: bank=14; base=0xC000
        else: print("for $8000-$BFFF use: bank <n> <base>"); sys.exit(1)
        blk=PRG[bank*0x2000:(bank+1)*0x2000]
        print(disasm(blk, base, start, min(end, base+0x2000)))
