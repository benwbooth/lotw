/* $C9FB:
 *   LDY #$E0
 * - LDA ($77),Y / STA $00A0,Y / INY / BMI -        ; copy 32 bytes E0..FF -> A0..BF
 *   LDA $40 / CMP #$06 / BCS done                  ; cur_character < 6 only
 *   ASL A / ASL A / CLC / ADC #$03 / TAX           ; X = char*4 + 3
 *   LDY #$03
 * - LDA $FFC5,X / STA $0190,Y / DEX / DEY / BPL -  ; copy 4 bytes from ROM table
 * done RTS
 */
#include "ram.h"
#include "regs.h"

void sub_C9FB(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));
    u8 a, x;
    int y;

    /* Y = $E0..$FF (BMI loops while Y has high bit set; wraps to 0 -> exit) */
    for (y = 0xE0; y <= 0xFF; y++)
        RAM8((u16)(0x00A0 + (u8)y)) = RAM8((u16)(ptr + (u8)y));

    a = cur_character;
    if (a >= 0x06) {                 /* BCS -> done */
        r->a = a;
        r->c = 1;
        return;
    }
    a = (u8)((a << 2) + 0x03);        /* ASL,ASL,CLC,ADC #$03 */
    x = a;
    for (y = 0x03; y >= 0; y--) {     /* DEY/BPL */
        RAM8((u16)(0x0190 + y)) = RAM8((u16)(0xFFC5 + x));
        x--;
    }
    r->a = a;
    r->x = x;
    r->y = (u8)0xFF;
    r->c = 0;
}
