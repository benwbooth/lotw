/* $EB90:
 *   LDA $F5 / ORA $F7 / BNE L_EB99
 *     JSR L_EE8D
 * L_EB99:
 *   LDA $F0 / BEQ L_EBA5
 *     JSR L_EEBB / BCC L_EBC6 / JMP L_EBCF
 * L_EBA5:
 *   LDY #$09 / LDA ($E7),Y / TAY / LDA $F4 / JSR L_CD70
 *   JSR L_F0E1 / BCS L_EBCC
 *   LDY #$01 / JSR L_F233 / BCC L_EBCC
 *   LDA $0E / BEQ L_EBC6
 *   LDY #$0D / JSR L_F233 / BCC L_EBCC
 * L_EBC6:
 *   JSR L_EF04 / JMP L_EBCF
 * L_EBCC:
 *   JSR L_EF11
 * L_EBCF:
 *   JSR L_F179 / JSR L_F01E / JMP L_EFF0 (RTS)
 *
 * Per-frame movement/placement step for one entity. JMP L_EFF0 is a bare RTS.
 */
#include "ram.h"
#include "regs.h"

void sub_EE8D(Regs *r);
void sub_EEBB(Regs *r);
void sub_CD70(Regs *r);
void sub_F0E1(Regs *r);
void sub_F233(Regs *r);
void sub_EF04(Regs *r);
void sub_EF11(Regs *r);
void sub_F179(Regs *r);
void sub_F01E(Regs *r);

void sub_EB90(Regs *r)
{
    int reached_EBC6 = 0;   /* fall into EF04 path */
    int reached_EBCC = 0;   /* EF11 path */
    int done = 0;           /* jumped straight to L_EBCF */

    if ((RAM8(0xF5) | RAM8(0xF7)) == 0)     /* BNE L_EB99 not taken */
        sub_EE8D(r);

    /* L_EB99 */
    if (RAM8(0xF0) != 0) {                   /* BEQ L_EBA5 not taken */
        sub_EEBB(r);
        if (r->c == 0)                       /* BCC L_EBC6 */
            reached_EBC6 = 1;
        else
            done = 1;                        /* JMP L_EBCF */
    } else {
        /* L_EBA5 */
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        r->y = RAM8((u16)(ptr + 9));         /* LDY #$09 / LDA ($E7),Y / TAY */
        r->a = RAM8(0xF4);                   /* LDA $F4 */
        sub_CD70(r);                         /* A=$F4, Y=struct[9] */

        sub_F0E1(r);
        if (r->c) {                          /* BCS L_EBCC */
            reached_EBCC = 1;
        } else {
            r->y = 0x01;
            sub_F233(r);
            if (r->c == 0) {                 /* BCC L_EBCC */
                reached_EBCC = 1;
            } else if (RAM8(0x0E) == 0) {    /* LDA $0E / BEQ L_EBC6 */
                reached_EBC6 = 1;
            } else {
                r->y = 0x0D;
                sub_F233(r);
                if (r->c == 0)               /* BCC L_EBCC */
                    reached_EBCC = 1;
                else
                    reached_EBC6 = 1;        /* fall into L_EBC6 */
            }
        }
    }

    if (!done) {
        if (reached_EBCC)
            sub_EF11(r);                     /* L_EBCC */
        else if (reached_EBC6)
            sub_EF04(r);                     /* L_EBC6 (then JMP L_EBCF) */
    }

    /* L_EBCF */
    sub_F179(r);
    sub_F01E(r);
    /* JMP L_EFF0 -> RTS */
}
