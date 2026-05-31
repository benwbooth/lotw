/* $A6E0 — draw the 3 boss-part metasprites. Initializes the OAM byte offset
 * $0F = $88 and the sprite_tables index $0E = $10, then loops 3 times: draw one
 * part (A703), advance $0F by 8 (next OAM group) and $0E by $10 (next part).
 * The PHA/PLA pair just preserves the loop counter across the JSR. RTS. */
#include "ram.h"
#include "regs.h"

void sub_A703(Regs *r);

void sub_A6E0(Regs *r)
{
    u8 count;

    RAM8(0x0F) = 0x88;                  /* LDA #$88 / STA $0F */
    RAM8(0x0E) = 0x10;                  /* LDA #$10 / STA $0E */
    count = 0x03;                       /* LDA #$03 */
    do {                                /* L_A6EA */
        sub_A703(r);                    /* JSR L_A703 */
        RAM8(0x0F) = (u8)(RAM8(0x0F) + 0x08);   /* CLC / ADC #$08 / STA $0F */
        RAM8(0x0E) = (u8)(RAM8(0x0E) + 0x10);   /* CLC / ADC #$10 / STA $0E */
        count = (u8)(count - 0x01);     /* SEC / SBC #$01 */
    } while (count != 0);               /* BNE L_A6EA */
}
