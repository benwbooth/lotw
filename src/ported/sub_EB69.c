/* $EB69:
 *   LDA $F5 / ORA $F7 / BNE L_EB72 / JSR L_EE9A
 *   L_EB72: LDY #$09 / LDA ($E7),Y / TAY / LDA $F4 / JSR L_CD70 / JSR L_F11B
 *           BCC L_EB87 / JSR L_EF11 / JMP L_EB8A
 *   L_EB87: JSR L_EF04
 *   L_EB8A: JSR L_F01E / JMP L_EFF0 (RTS)
 */
#include "ram.h"
#include "regs.h"

void sub_EE9A(Regs *);
void sub_CD70(Regs *);
void sub_F11B(Regs *);
void sub_EF11(Regs *);
void sub_EF04(Regs *);
void sub_F01E(Regs *);

void sub_EB69(Regs *r)
{
    if ((RAM8(0xF5) | RAM8(0xF7)) == 0)
        sub_EE9A(r);                     /* no register inputs */

    /* L_EB72 */
    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        u8 v = RAM8((u16)(ptr + 0x09));
        r->y = v;
        r->a = RAM8(0xF4);
        sub_CD70(r);                     /* A=$F4, Y=stream byte */
    }

    sub_F11B(r);
    if (r->c)
        sub_EF11(r);
    else
        sub_EF04(r);

    sub_F01E(r);
    /* JMP L_EFF0 -> RTS */
}
