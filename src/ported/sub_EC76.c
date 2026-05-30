/* $EC76:
 *   LDA $F5 / ORA $F7 / BEQ L_EC82 / LDA $F3 / CMP #$20 / BCC L_EC85
 *   L_EC82: JSR L_EE53
 *   L_EC85: LDY #$09 / LDA ($E7),Y / TAY / LDA $F4 / JSR L_CD70 / JSR L_F11B
 *           BCC L_EC9F / JSR L_F2DA / BCC L_EC9F / JSR L_EF11 / JMP L_ECA2
 *   L_EC9F: JSR L_EF04
 *   L_ECA2: JSR L_F01E / JMP L_EFF0 (RTS)
 */
#include "ram.h"
#include "regs.h"

void sub_EE53(Regs *);
void sub_CD70(Regs *);
void sub_F11B(Regs *);
void sub_F2DA(Regs *);
void sub_EF11(Regs *);
void sub_EF04(Regs *);
void sub_F01E(Regs *);

void sub_EC76(Regs *r)
{
    /* call EE53 unless ($F5|$F7)!=0 and $F3 < $20 */
    int skip = ((RAM8(0xF5) | RAM8(0xF7)) != 0) && (RAM8(0xF3) < 0x20);
    if (!skip)
        sub_EE53(r);                     /* L_EC82 */

    /* L_EC85 */
    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        r->y = RAM8((u16)(ptr + 0x09));
        r->a = RAM8(0xF4);
        sub_CD70(r);
    }

    sub_F11B(r);
    if (r->c) {                          /* BCC skips this whole block */
        sub_F2DA(r);
        if (r->c) {
            sub_EF11(r);
            sub_F01E(r);                 /* L_ECA2 */
            return;
        }
    }

    /* L_EC9F */
    sub_EF04(r);
    sub_F01E(r);                         /* L_ECA2 */
    /* JMP L_EFF0 -> RTS */
}
