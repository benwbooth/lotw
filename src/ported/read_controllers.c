/* $CC43 read_controllers — strobe both controllers, then shift 8 bits out of
 * JOY1 ($4016) ORed with APU_FRAME ($4017) into the button shadows $20/$21,
 * finally OR them together into $20.
 *
 * Shim builds hit the software controller shift register via REG_W/REG_R.
 * Flat host builds keep the old RAM8 behavior for non-interactive routine tests. */
#include "ram.h"
#include "regs.h"

void read_controllers(Regs *r)
{
    u8 x, a, c;

#ifdef LOTW_SHIM
    /* Lockstep co-sim: pull the next input by controller-READ count (content-aligned)
     * before strobing, so frame-timing slips don't misalign input. */
    {
        extern u8 (*nes_next_input)(void);
        extern void ppu_set_buttons(u8 b);
        if (nes_next_input) ppu_set_buttons(nes_next_input());
    }
    /* Shim build: hit the real controller shift register via REG_W/REG_R so each
     * read returns the next button bit (flat RAM8 returns the same value 8x). */
    REG_W(0x4016, 0x01);
    REG_W(0x4016, 0x00);
    for (x = 8; x != 0; x--) {
        a = (u8)(REG_R(0x4016) | REG_R(0x4017));
        c = a & 1; a >>= 1;
        RAM8(0x20) = (u8)((RAM8(0x20) << 1) | c);
        c = a & 1;
        RAM8(0x21) = (u8)((RAM8(0x21) << 1) | c);
    }
    RAM8(0x20) = RAM8(0x20) | RAM8(0x21);
    (void)r; return;
#endif
    RAM8(0x4016) = 0x01;          /* LDX #$01 / STX JOY1  — strobe on  */
    RAM8(0x4016) = 0x00;          /* DEX / STX JOY1       — strobe off */
    for (x = 8; x != 0; x--) {
        a = RAM8(0x4016) | RAM8(0x4017);   /* LDA JOY1 / ORA APU_FRAME */
        c = a & 1; a >>= 1;                /* LSR A; ROL $20 */
        RAM8(0x20) = (u8)((RAM8(0x20) << 1) | c);
        c = a & 1;                         /* LSR A; ROL $21 */
        RAM8(0x21) = (u8)((RAM8(0x21) << 1) | c);
    }
    RAM8(0x20) = RAM8(0x20) | RAM8(0x21);  /* LDA $20 / ORA $21 / STA $20 */
}
