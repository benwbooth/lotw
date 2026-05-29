/* $EF04:  LDA $0E/STA $F9, LDA $0F/STA $FA, LDA $0A/STA $FB, RTS
 * Copies three zero-page bytes into a destination triple. */
#include "ram.h"
#include "regs.h"

void sub_EF04(Regs *r)
{
    RAM8(0xF9) = RAM8(0x0E);
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xFB) = RAM8(0x0A);
    r->a = RAM8(0x0A);      /* A holds last loaded value at RTS */
}
