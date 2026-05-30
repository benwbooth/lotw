/* $CA54:  saves original $0D, runs L_CA85 (mul $0C/$0D), then combines the
 * saved high byte (>>4) with $0C, building 16-bit results in $0C/$0D and $10/$11.
 *   LDA $0D / PHA / JSR L_CA85 / LDA $0D / STA $11 / PLA / LSR x4
 *   CLC / ADC $0C / STA $0C / STA $10 / BCC + / INC $0D / INC $11
 *   + CLC / LDA $0D / ADC #$05 / STA $0D
 *   CLC / LDA $10 / ADC $75 / STA $10 / LDA $11 / ADC $76 / STA $11 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA85(Regs *r);

void sub_CA54(Regs *r)
{
    u8 saved_0d = RAM8(0x0D);   /* PHA */

    sub_CA85(r);                /* mutates $0C/$0D */

    RAM8(0x11) = RAM8(0x0D);

    {
        u8 a = (u8)(saved_0d >> 4);     /* PLA / LSR x4 */
        u16 s = (u16)(a + RAM8(0x0C));  /* CLC / ADC $0C */
        RAM8(0x0C) = (u8)s;
        RAM8(0x10) = (u8)s;
        if (s & 0x100) {                /* BCC skip; carry -> INC $0D / INC $11 */
            RAM8(0x0D) = (u8)(RAM8(0x0D) + 1);
            RAM8(0x11) = (u8)(RAM8(0x11) + 1);
        }
    }

    RAM8(0x0D) = (u8)(RAM8(0x0D) + 0x05);   /* CLC / ADC #$05 */

    {
        u16 lo = (u16)(RAM8(0x10) + RAM8(0x75));   /* CLC / ADC $75 */
        u8 carry = (u8)(lo >> 8);
        RAM8(0x10) = (u8)lo;
        RAM8(0x11) = (u8)(RAM8(0x11) + RAM8(0x76) + carry);
    }
}
