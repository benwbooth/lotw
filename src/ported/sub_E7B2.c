/* $E7B2:
 *   LDX #$37
 * L_E7B4:
 *   LDA $FF6F,X / STA $0280,X / DEX / BPL L_E7B4   ; copy 56 ROM bytes -> $0280
 *   LDA #$34 / STA mmc3_r2_shadow ($2C)
 *   LDA #$35 / STA mmc3_r3_shadow ($2D)
 *   LDA #$36 / STA mmc3_r4_shadow ($2E)
 *   LDA #$37 / STA mmc3_r5_shadow ($2F)
 *   RTS
 * Initialises the OAM/sprite-DMA buffer at $0280 from ROM table $FF6F and seeds
 * the four MMC3 bank-select shadow regs to $34..$37. No register inputs. */
#include "ram.h"
#include "regs.h"

void sub_E7B2(Regs *r)
{
    int x;
    for (x = 0x37; x >= 0; x--)            /* DEX/BPL: X = $37 down to 0 */
        RAM8((u16)(0x0280 + x)) = RAM8((u16)(0xFF6F + x));

    RAM8(0x2C) = 0x34;                     /* mmc3_r2_shadow */
    RAM8(0x2D) = 0x35;                     /* mmc3_r3_shadow */
    RAM8(0x2E) = 0x36;                     /* mmc3_r4_shadow */
    RAM8(0x2F) = 0x37;                     /* mmc3_r5_shadow */

    r->x = 0xFF;                           /* loop exits with X = $FF */
    r->a = 0x37;                           /* last LDA #$37 */
}
