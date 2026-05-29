/* $CA85: 16-bit multiply ($0C) * 12 -> ($0C low : $0D high).
 *   LDA #$00 / STA $0D           ; $0D = 0
 *   ASL $0C / ROL $0D            ; $0C:$0D = 2c
 *   ASL $0C / ROL $0D            ; $0C:$0D = 4c
 *   LDX $0D / LDY $0C            ; Y:X = 4c (low:high)
 *   ASL $0C / ROL $0D            ; $0C:$0D = 8c
 *   TYA / CLC / ADC $0C / STA $0C; low  = 4c_lo + 8c_lo
 *   TXA / ADC $0D / STA $0D     ; high = 4c_hi + 8c_hi + carry
 * Result $0C:$0D = 4c + 8c = 12c (16-bit). Input: $0C. */
#include "ram.h"
#include "regs.h"

void sub_CA85(Regs *r)
{
    u16 four = (u16)(RAM8(0x0C) << 2);     /* 4c (16-bit) */
    u16 eight = (u16)(RAM8(0x0C) << 3);    /* 8c (16-bit) */
    u16 result = (u16)(four + eight);

    u8 x = (u8)(four >> 8);                /* LDX $0D after ROL -> high of 4c */
    u8 y = (u8)four;                       /* LDY $0C -> low of 4c */

    RAM8(0x0C) = (u8)result;
    RAM8(0x0D) = (u8)(result >> 8);
    r->x = x;
    r->y = y;
    r->a = (u8)(result >> 8);             /* A holds final high byte after STA $0D */
}
