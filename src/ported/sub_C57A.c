/* $C57A — wait for pending VRAM job (sub_CC97), then blit a 160-byte block from
 * ROM $FECB to nametable $2320 and clear 16 bytes at $23F0, all via PPUDATA.
 * Saves/restores ppuctrl_shadow ($23) and PPUMASK shadow ($24); sets $29=1.
 * Net RAM: $28=0 (sub_CC97), $29=1; $23/$24 restored unchanged. */
#include "ram.h"
#include "regs.h"

void sub_CC97(Regs *r);

void sub_C57A(Regs *r)
{
    u8 saved_ctrl, saved_mask;
    int i;

    sub_CC97(r);                          /* JSR L_CC97 -> $28 = 0 */

    saved_ctrl = RAM8(0x23);              /* LDA ppuctrl_shadow / PHA */
    REG_W(0x2000, saved_ctrl & 0x7B);     /* AND #$7B / STA PPUCTRL */
    RAM8(0x29) = 0x00;                    /* STA $29 */

    saved_mask = RAM8(0x24);              /* LDA $24 / PHA */
    REG_W(0x2001, saved_mask & 0xE7);     /* AND #$E7 / STA PPUMASK */

    REG_W(0x2006, 0x23);                  /* PPUADDR = $2320 */
    REG_W(0x2006, 0x20);
    for (i = 0; i < 0xA0; ++i)            /* LDY #$A0 loop */
        REG_W(0x2007, RAM8((u16)(0xFECB + i)));

    REG_W(0x2006, 0x23);                  /* PPUADDR = $23F0 */
    REG_W(0x2006, 0xF0);
    for (i = 0; i < 0x10; ++i)            /* LDY #$10 loop, clear */
        REG_W(0x2007, 0x00);

    RAM8(0x29) += 1;                      /* INC $29 -> 1 */

    RAM8(0x24) = saved_mask;              /* PLA / STA $24 (restored) */
    RAM8(0x23) = saved_ctrl;              /* PLA / STA ppuctrl_shadow */
    REG_W(0x2000, saved_ctrl);            /* STA PPUCTRL */

    r->a = saved_ctrl;
    r->y = 0x00;
}
