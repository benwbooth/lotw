/* $E6B7 sub_E6B7 — build OAM-style sprite entries for the 3 carried items
 * ($51..$53) into the display buffer at $0240+. Three iterations (X=2..0),
 * Y stepping $10,$08,$00; each item occupies two 4-byte sprites.
 *
 *   $08 = $58  (running Y screen position)
 *   for X = 2 down to 0, Y = $10 then -8 each pass:
 *     item = carried_item0[X]              ($51+X)
 *     if item < 0 (bit7 set):
 *         attr/tile A = $EF
 *         ($0241+Y, $0245+Y left untouched)
 *     else:
 *         t = item*4 + $A1
 *         $0241+Y = t ;  $0245+Y = t+2
 *         A = $BB
 *     $0240+Y = A ;  $0244+Y = A          (Y screen coords / first byte)
 *     $0243+Y = $08 ;  $0247+Y = $08+8    (X screen coords)
 *     $08 = ($08+8) - $28                 (advance, net -$20)
 *     $0242+Y = 1 ;  $0246+Y = 1          (attributes)
 *     Y -= 8
 */
#include "ram.h"
#include "regs.h"

void sub_E6B7(Regs *r)
{
    int x;
    u8 y = 0x10;
    u8 a;

    RAM8(0x08) = 0x58;                          /* LDA #$58 / STA $08 */

    for (x = 2; x >= 0; --x) {                  /* LDX #$02 ... DEX / BPL */
        u8 item = RAM8((u16)(0x0051 + x));      /* LDA carried_item0,X */
        if (item & 0x80) {                      /* BMI L_E6D6 */
            a = 0xEF;
        } else {
            u8 t = (u8)(((u8)(item << 2)) + 0xA1);  /* ASL/ASL/ADC #$A1 */
            RAM8((u16)(0x0241 + y)) = t;
            RAM8((u16)(0x0245 + y)) = (u8)(t + 0x02);
            a = 0xBB;
        }
        RAM8((u16)(0x0240 + y)) = a;            /* STA $0240,Y / $0244,Y */
        RAM8((u16)(0x0244 + y)) = a;

        RAM8((u16)(0x0243 + y)) = RAM8(0x08);   /* LDA $08 / STA $0243,Y */
        RAM8((u16)(0x0247 + y)) = (u8)(RAM8(0x08) + 0x08);  /* ADC #$08 / STA $0247,Y */
        RAM8(0x08) = (u8)((u8)(RAM8(0x08) + 0x08) - 0x28);  /* SBC #$28 / STA $08 */

        RAM8((u16)(0x0242 + y)) = 0x01;         /* LDA #$01 / STA $0242,Y / $0246,Y */
        RAM8((u16)(0x0246 + y)) = 0x01;

        y = (u8)(y - 0x08);                     /* TYA / SBC #$08 / TAY */
    }
    r->x = 0xFF;
    r->y = y;
}
