/* $DD42: sets pointer $E5/$E6=$0490, saves $0E/$0F/$0A, builds tile pointer
 * $0C/$0D from $0F/$0A via sub_CA54, then probes up to four tiles via sub_DD97
 * (LDY #$00, #$0C, #$01, #$0D), bailing out early when DD97 returns carry set.
 * On the clean path it CLC; finally restores $0A/$0F/$0E from the stack and RTS.
 * Carry on return reflects the last DD97 (set) or the explicit CLC.
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_DD97(Regs *r);

void sub_DD42(Regs *r)
{
    u8 s_0E, s_0F, s_0A;

    RAM8(0xE5) = 0x90;
    RAM8(0xE6) = 0x04;

    s_0E = RAM8(0x0E);
    s_0F = RAM8(0x0F);
    s_0A = RAM8(0x0A);

    RAM8(0x0C) = RAM8(0x0F);
    RAM8(0x0D) = RAM8(0x0A);

    sub_CA54(r);

    r->y = 0x00;
    sub_DD97(r);
    if (r->c) goto restore;

    if (RAM8(0x0E) != 0) {
        r->y = 0x0C;
        sub_DD97(r);
        if (r->c) goto restore;
    }

    /* L_DD70 */
    {
        u8 a = RAM8(0x0A);
        if (a >= 0xB0) goto done_clc;
        if ((a & 0x0F) == 0) goto done_clc;

        r->y = 0x01;
        sub_DD97(r);
        if (r->c) goto restore;

        if (RAM8(0x0E) == 0) goto done_clc;

        r->y = 0x0D;
        sub_DD97(r);
        if (r->c) goto restore;
    }

done_clc:
    r->c = 0;

restore:
    RAM8(0x0A) = s_0A;
    RAM8(0x0F) = s_0F;
    RAM8(0x0E) = s_0E;
}
