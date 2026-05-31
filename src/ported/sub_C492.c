/* $C492:  $09=$40; loop { $36=5; C9FB; X=4,Y=$1C,C520; C135; $09-=$10; BPL }; C569
 * Fade/animate loop: decrements palette buffer brightness via C520 each pass,
 * 5 passes ($09 = $40,$30,$20,$10,$00, then $F0 exits), then commits via C569. */
#include "ram.h"
#include "regs.h"

void sub_C9FB(Regs *r);
void sub_C520(Regs *r);
void sub_C135(Regs *r);
void sub_C569(Regs *r);

void sub_C492(Regs *r)
{
    u8 v = 0x40;
    RAM8(0x09) = v;
    do {
        RAM8(0x36) = 0x05;
        sub_C9FB(r);
        r->x = 0x04;
        r->y = 0x1C;
        sub_C520(r);
        sub_C135(r);
        v = (u8)(RAM8(0x09) - 0x10);
        RAM8(0x09) = v;
    } while ((v & 0x80) == 0);   /* BPL: loop while bit7 clear */
    sub_C569(r);
}
