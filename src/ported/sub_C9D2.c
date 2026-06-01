/* $C9D2 — set up the map row's CHR bank ($30 mmc3_r6_shadow) and tile pointers.
 *   A = map_screen_y>>1; if A != mmc3_r6_shadow then store it and queue job $FF.
 *   t = ((map_screen_y&1)<<2 | map_screen_x) << 2;
 *   $76 = t + $80; $78 = $76 + 3; $77 = 0; $75 = 0.
 * Net RAM: $30 (maybe), $76, $78, $77=0, $75=0, $28=0 (queue). */
#include "ram.h"
#include "regs.h"

void queue_ppu_job_and_wait(Regs *r);

void sub_C9D2(Regs *r)
{
    u8 bank = (u8)(RAM8(0x48) >> 1);     /* LDA map_screen_y / LSR A */
    u8 t, lo;

    if (bank != RAM8(0x30)) {            /* CMP mmc3_r6_shadow / BEQ */
        RAM8(0x30) = bank;               /* STA mmc3_r6_shadow */
        r->a = 0xFF;                     /* LDA #$FF */
        queue_ppu_job_and_wait(r);       /* JSR $CC8F */
    }

    t = (u8)(((RAM8(0x48) & 0x01) << 2)); /* (y&1)<<2 */
    t = (u8)((t | RAM8(0x47)) << 2);      /* ORA map_screen_x / ASL ASL */
    lo = (u8)(t + 0x80);                  /* CLC / ADC #$80 */
    RAM8(0x76) = lo;                      /* STA $76 */
    RAM8(0x78) = (u8)(lo + 0x03);         /* CLC / ADC #$03 / STA $78 */
    RAM8(0x77) = 0x00;                    /* STA $77 */
    RAM8(0x75) = 0x00;                    /* STA $75 */

    /* The final carry-affecting op is "CLC / ADC #$03"; the STA $77/$75 that follow
     * don't touch it. text_attr_build's first "ADC #$A0" (forming the tile-table
     * pointer $7A) consumes this carry, so model it. */
    r->c = ((lo + 0x03) > 0xFF) ? 1 : 0;
}
