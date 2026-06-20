/* Portable machine model for the Legacy of the Wizard C port.
 *
 * HOST builds keep the game's 64 KiB memory map in a plain array. Hardware
 * register writes go through REG_W()/REG_R() so the software PPU/APU/controller
 * shims can observe them.
 */
#ifndef LOTW_NES_H
#define LOTW_NES_H

typedef unsigned char u8;
typedef unsigned short u16;

#ifdef __cplusplus
extern "C" {
#endif

#ifdef LOTW_HOST
extern u8 NES_MEM[0x10000];               /* full game address space on host */
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

/* Far-call PRG-bank sync. The far-call helpers change the $30/$31 bank shadows.
 * In the software shim, NES_MEM must be re-mapped from the shadows so far-called
 * code reads its own bank's data — see nes_prg_map_shadow(). */
#ifdef LOTW_SHIM
void nes_prg_map_shadow(void);
#define NES_PRG_SYNC() nes_prg_map_shadow()
#else
#define NES_PRG_SYNC() ((void)0)
#endif

#ifdef __cplusplus
}
#endif

#endif /* LOTW_NES_H */
