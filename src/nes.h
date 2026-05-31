/* Portable NES machine model for the Legacy of the Wizard C port.
 *
 * Compiles two ways from one source:
 *   - HOST (gcc/clang, -DLOTW_HOST): the whole 6502 address space is a 64 KiB
 *     array (RAM + ROM banks mapped in by the test harness), so pointer
 *     dereferences match the m6502.py oracle. Hardware-register writes are
 *     captured (mocked).
 *   - NES (cc65): RAM/ROM/registers are real addresses; links into a ROM.
 *
 * All memory access goes through RAM8()/REG_W() so both builds share one source.
 */
#ifndef LOTW_NES_H
#define LOTW_NES_H

typedef unsigned char u8;
typedef unsigned short u16;

#ifdef LOTW_HOST
extern u8 NES_MEM[0x10000];               /* full 6502 address space on host */
#define RAM8(a)  (NES_MEM[(a) & 0xFFFF])
void nes_reg_write(u16 addr, u8 val);     /* hardware-register write hook */
u8   nes_reg_read(u16 addr);              /* hardware-register read hook  */
#define REG_W(a, v) nes_reg_write((u16)(a), (u8)(v))
#define REG_R(a)    nes_reg_read((u16)(a))
#else
#define RAM8(a)  (*(volatile u8 *)(a))
#define REG_W(a, v) (*(volatile u8 *)(a) = (u8)(v))
#define REG_R(a)    (*(volatile u8 *)(a))
#endif

#endif /* LOTW_NES_H */
