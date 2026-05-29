/* $FCF9:
 *   LDX $02
 *   LDA $97,X / STA $95,X
 *   LDA $98,X / STA $96,X / BEQ L_FD0A   ; Z reflects $98,X value just loaded
 *   LDA #$01 / STA $93,X / RTS
 * L_FD0A:
 *   LDA $94,X / AND #$40 / STA $94,X / RTS
 * X = $02; copies word $97/$98 -> $95/$96 (zp,X wrap). If high byte ($98,X)
 * nonzero set $93,X=1, else clear all but bit6 of $94,X.
 */
#include "ram.h"
#include "regs.h"

void sub_FCF9(Regs *r)
{
    u8 x = RAM8(0x02);
    u8 hi;
    RAM8((0x95 + x) & 0xFF) = RAM8((0x97 + x) & 0xFF);
    hi = RAM8((0x98 + x) & 0xFF);
    RAM8((0x96 + x) & 0xFF) = hi;
    if (hi != 0) {                       /* BEQ not taken */
        RAM8((0x93 + x) & 0xFF) = 0x01;
    } else {
        RAM8((0x94 + x) & 0xFF) &= 0x40;
    }
    r->x = x;
}
