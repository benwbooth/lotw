/* $DCA8:  LDA ($0C),Y / AND #$3F / CMP #$30 / BNE L_DCCA(CLC,RTS)
 *   if (byte&$3F)==$30:
 *     if $4F==0: $4F=$0A
 *     if $85==0: JSR L_E7CE; $8F=$0A; $85=$01
 *     SEC; RTS
 *   else CLC; RTS
 * Reads tile byte via pointer $0C/$0D + Y; if it's a specific tile ($30),
 * sets up a respawn/death sequence. Carry = matched. */
#include "ram.h"
#include "regs.h"

void sub_E7CE(Regs *r);

void sub_DCA8(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    u8 v = RAM8((u16)(ptr + r->y)) & 0x3F;

    if (v != 0x30) {
        r->c = 0;            /* CLC */
        return;
    }
    if (RAM8(0x4F) == 0)
        RAM8(0x4F) = 0x0A;
    if (RAM8(0x85) == 0) {
        sub_E7CE(r);
        RAM8(0x8F) = 0x0A;
        RAM8(0x85) = 0x01;
    }
    r->c = 1;                /* SEC */
}
