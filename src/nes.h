/* Portable NES machine model for the Legacy of the Wizard C port.
 *
 * The same game-logic .c files compile two ways:
 *   - HOST (gcc/clang, -DLOTW_HOST): RAM is a plain 2 KiB array, hardware
 *     registers are captured into a mock log. Used for differential testing
 *     against the original 6502 (the m6502.py oracle).
 *   - NES (cc65): RAM/registers are real addresses; links into a ROM.
 *
 * RAM access goes through RAM8()/RAM16() so both builds share one source.
 */
#ifndef LOTW_NES_H
#define LOTW_NES_H

typedef unsigned char u8;
typedef unsigned short u16;

#ifdef LOTW_HOST
extern u8 NES_RAM[0x800];                 /* 2 KiB internal RAM ($0000-$07FF) */
#define RAM8(a)  (NES_RAM[(a) & 0x7FF])
/* Hardware-register writes are captured (mocked) on the host. */
void nes_reg_write(u16 addr, u8 val);
#define REG_W(a, v) nes_reg_write((a), (u8)(v))
#else
#define RAM8(a)  (*(volatile u8 *)(a))
#define REG_W(a, v) (*(volatile u8 *)(a) = (u8)(v))
#endif

#endif /* LOTW_NES_H */
