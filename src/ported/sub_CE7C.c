/* $CE7C:
 *   LDA #$00 / STA $EA
 *   JSR L_CEB6 / BCC CE8F        ; vertical proximity; if C=0 return (C=0)
 *   JSR L_CE90 / BCC CE8F        ; horizontal proximity; if C=0 return (C=0)
 *   LDA #$01 / STA $EA / SEC     ; both -> $EA=1, C=1
 * CE8F: RTS
 * Output: $EA (RAM) and carry.
 */
#include "ram.h"
#include "regs.h"

void sub_CEB6(Regs *r);
void sub_CE90(Regs *r);

void sub_CE7C(Regs *r)
{
    RAM8(0xEA) = 0x00;

    sub_CEB6(r);
    if (r->c == 0) return;        /* BCC CE8F */

    sub_CE90(r);
    if (r->c == 0) return;        /* BCC CE8F */

    RAM8(0xEA) = 0x01;
    r->c = 1;                     /* SEC */
}
