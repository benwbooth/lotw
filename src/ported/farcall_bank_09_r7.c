/* $C833 farcall_bank_09_r7
 * Switches MMC3 r7 to bank $09, runs sub_CA54 + metasprite_build, then restores
 * the saved r7 bank shadow. The PHA/PLA just preserves the old r7 shadow across
 * the calls (not a frame pop), modeled here with a local.
 *
 *   LDA $31 / PHA                  save old r7 shadow
 *   LDA #$07 / STA $25 / STA $8000 select r7
 *   LDA #$09 / STA $31 / STA $8001 r7 = $09
 *   LDA #$00 / STA $0D
 *   JSR L_CA54 (sub_CA54)
 *   JSR metasprite_build
 *   LDA #$07 / STA $25 / STA $8000 select r7
 *   PLA / STA $31 / STA $8001      restore old r7 shadow
 *   RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void metasprite_build(Regs *r);

void farcall_bank_09_r7(Regs *r)
{
    u8 saved_r7 = RAM8(0x31);           /* PHA: old r7 shadow */

    RAM8(0x25) = 0x07;                  /* select r7 */
    REG_W(0x8000, 0x07);
    RAM8(0x31) = 0x09;                  /* r7 = bank $09 */
    REG_W(0x8001, 0x09);

    RAM8(0x0D) = 0x00;

    r->a = 0x00;                        /* LDA #$00 before JSR */
    sub_CA54(r);
    metasprite_build(r);

    RAM8(0x25) = 0x07;                  /* select r7 */
    REG_W(0x8000, 0x07);
    RAM8(0x31) = saved_r7;              /* PLA: restore old r7 shadow */
    REG_W(0x8001, saved_r7);

    r->a = saved_r7;
}
