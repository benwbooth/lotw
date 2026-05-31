#!/usr/bin/env python3
"""Trace the oracle on a single failing sub_D620 state dumped by bulkdiff
(DUMP_FAIL=/tmp/d620_fail.bin). Prints the sound-engine execution path (entries
into known routine addresses) and every write to $A3 plus the music pointer
$A5:$A6, so we can see which routine the port replicates wrongly.

Usage:  python3 tools/re/trace_d620.py /tmp/d620_fail.bin
"""
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
from m6502 import CPU, ROM, U, I

# Sound-engine routine entry points (JSR/JMP targets) worth logging on entry.
ENTRIES = {0xF89A, 0xF8F0, 0xF96E, 0xFA09, 0xFB1F, 0xFB8E, 0xFD6B, 0xFCF9,
           0xFC81, 0xFD11, 0xFCB4, 0xFCC4, 0xFCDF, 0xFBC5, 0xFBE2, 0xFBFF,
           0xFC02, 0xFC05, 0xF978, 0xF97F, 0xF991, 0xF997, 0xF9DD, 0xFA60,
           0xF89A, 0xFA09}


def main():
    path = sys.argv[1] if len(sys.argv) > 1 else "/tmp/d620_fail.bin"
    blob = Path(path).read_bytes()
    a, x, y = blob[0], blob[1], blob[2]
    ram = blob[3:3 + 0x800]
    rom = ROM.read_bytes()
    cpu = CPU()
    cpu.map_fixed(rom)
    cpu.map_bank(rom, 13, 0xA000)
    cpu.mem[0x0000:0x0800] = ram
    cpu.a, cpu.x, cpu.y, cpu.p, cpu.s = a, x, y, (U | I), 0xFD
    SENT = 0x0FFE
    cpu._push((SENT >> 8) & 0xFF); cpu._push(SENT & 0xFF)
    cpu.pc = 0xD620
    sc = [0x28, 0x36]
    print(f"state a={a:02X} x={x:02X} y={y:02X} | "
          f"$02={ram[0x02]:02X} $A3={ram[0xA3]:02X} $A4={ram[0xA4]:02X} "
          f"$A5={ram[0xA5]:02X} $A6={ram[0xA6]:02X} $36={ram[0x36]:02X}")
    prev_a3 = cpu.mem[0xA3]
    prev_80dc = (cpu.mem[0x80DC], cpu.mem[0x80DD])
    prev_08 = cpu.mem[0x08]
    si_seen = False
    snd_calls = 0
    events = []
    for i in range(200000):
        if cpu.pc == (SENT + 1) & 0xFFFF:
            break
        if i >= 4000:
            for ad in sc:
                cpu.mem[ad] = 0
        pc = cpu.pc
        if pc == 0xFC08:   # song_init entry — snapshot scratch + bank table cell
            si_seen = True
            zp = " ".join(f"{cpu.mem[k]:02X}" for k in range(0x10))
            events.append(("SI", f"$8E={cpu.mem[0x8E]:02X} $80DC={cpu.mem[0x80DC]:02X} "
                                 f"$80DD={cpu.mem[0x80DD]:02X} zp00-0F: {zp}"))
        cur = pc
        cpu.step()
        if not si_seen and cpu.mem[0x08] != prev_08:
            w08 = [e for e in events if e[0] == "W08"]
            # keep only the LAST 12 $08 writes before song_init (drop older to avoid spam)
            if len(w08) >= 12:
                events.remove(w08[0])
            events.append(("W08", cur, prev_08, cpu.mem[0x08]))
        prev_08 = cpu.mem[0x08]
        now = (cpu.mem[0x80DC], cpu.mem[0x80DD])
        if now != prev_80dc:
            events.append(("W80", cur, prev_80dc, now))
            prev_80dc = now
        if cpu.mem[0xA3] != prev_a3:
            events.append(("A3", cpu.pc, prev_a3, cpu.mem[0xA3], cpu.mem[0x0E], cpu.mem[0x0F]))
            prev_a3 = cpu.mem[0xA3]
    print(f"oracle: F89A(sound_tick) entered {snd_calls}x; final $A3={cpu.mem[0xA3]:02X}; "
          f"steps={i}")
    for ev in events:
        if ev[0] == "A3":
            print(f"  A3write @ {ev[1]:04X}: {ev[2]:02X} -> {ev[3]:02X}  (src ptr $0E:$0F = {ev[5]:02X}{ev[4]:02X})")
        elif ev[0] == "SI":
            print(f"  song_init entry: {ev[1]}")
        elif ev[0] == "W80":
            print(f"  $80DC/DD write @ {ev[1]:04X}: {ev[2][0]:02X},{ev[2][1]:02X} -> {ev[3][0]:02X},{ev[3][1]:02X}")
        elif ev[0] == "W08":
            print(f"  $08 write @ {ev[1]:04X}: {ev[2]:02X} -> {ev[3]:02X}")


if __name__ == "__main__":
    main()
