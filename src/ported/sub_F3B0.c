/* $F3B0: per-frame entity movement/direction dispatcher.
 *
 *   LDA $F4 / AND #$0F / STA $F4
 *   LDA $F5 / ORA $F7 / BNE L_F3E8
 *     LDA $F4 / AND #$03 / BNE L_F3C6
 *       LDA #$01 / STA $F4
 *   L_F3C6:
 *     LDX $F3 / LDA #$00 / STA $F3 / DEX / BNE L_F3DC
 *       LDA $F4 / AND #$03 / BEQ L_F3EE
 *       EOR #$03 / STA $F4 / JMP L_F3F5
 *   L_F3DC:
 *     JSR L_EE19 / LDA #$80 / ORA $F4 / STA $F4 / JMP L_F3F5
 *   L_F3E8:
 *     LDA $F3 / CMP #$32 / BCC L_F3F5
 *   L_F3EE:
 *     LDA #$00 / STA $F3 / JSR L_EE19
 *   L_F3F5:
 *     LDA $F4 / LDY #$02 / JSR L_CD70
 *     LDA $F0 / BNE L_F41C
 *     LDA $F1 / BNE L_F408
 *     LDA $F4 / BPL L_F40D
 *   L_F408:
 *     JSR L_F4E3 / BCC L_F421
 *   L_F40D:
 *     LDA #$00 / STA $F1 / JSR L_F506 / BCC L_F421
 *     JSR L_EF11 / JMP L_F424
 *   L_F41C:
 *     JSR L_F4C3 / BCS L_F424
 *   L_F421:
 *     JSR L_EF04
 *   L_F424:
 *     JSR L_F1E4 / JSR L_F53B / JSR L_F552 / JMP L_EFF0 (RTS)
 */
#include "ram.h"
#include "regs.h"

void sub_EE19(Regs *r);
void sub_CD70(Regs *r);
void sub_F4E3(Regs *r);
void sub_F506(Regs *r);
void sub_EF11(Regs *r);
void sub_F4C3(Regs *r);
void sub_EF04(Regs *r);
void sub_F1E4(Regs *r);
void sub_F53B(Regs *r);
void sub_F552(Regs *r);

void sub_F3B0(Regs *r)
{
    u8 a;

    RAM8(0xF4) = RAM8(0xF4) & 0x0F;

    if ((RAM8(0xF5) | RAM8(0xF7)) == 0) {     /* BNE L_F3E8 not taken */
        if ((RAM8(0xF4) & 0x03) == 0)          /* BNE L_F3C6 not taken */
            RAM8(0xF4) = 0x01;

        /* L_F3C6 */
        {
            u8 x = RAM8(0xF3);
            RAM8(0xF3) = 0x00;
            x = (u8)(x - 1);                   /* DEX */
            if (x == 0) {                      /* BNE L_F3DC not taken */
                a = RAM8(0xF4) & 0x03;
                if (a != 0) {                  /* BEQ L_F3EE not taken */
                    RAM8(0xF4) = (u8)(a ^ 0x03);
                    goto F3F5;
                }
                goto F3EE;
            }
            /* L_F3DC */
            sub_EE19(r);
            RAM8(0xF4) = (u8)(0x80 | RAM8(0xF4));
            goto F3F5;
        }
    } else {
        /* L_F3E8 */
        if (RAM8(0xF3) < 0x32)                 /* BCC L_F3F5 */
            goto F3F5;
        /* fall to L_F3EE */
    }

F3EE:
    RAM8(0xF3) = 0x00;
    sub_EE19(r);

F3F5:
    r->a = RAM8(0xF4);
    r->y = 0x02;
    sub_CD70(r);

    if (RAM8(0xF0) != 0) {                     /* BNE L_F41C */
        sub_F4C3(r);
        if (r->c)                              /* BCS L_F424 */
            goto F424;
        goto F421;
    }

    /* F0 == 0 */
    if (RAM8(0xF1) != 0)                        /* BNE L_F408 */
        goto F408;
    if (!(RAM8(0xF4) & 0x80))                   /* BPL L_F40D */
        goto F40D;

F408:
    sub_F4E3(r);
    if (!r->c)                                  /* BCC L_F421 */
        goto F421;

F40D:
    RAM8(0xF1) = 0x00;
    sub_F506(r);
    if (!r->c)                                  /* BCC L_F421 */
        goto F421;
    sub_EF11(r);
    goto F424;

F421:
    sub_EF04(r);

F424:
    sub_F1E4(r);
    sub_F53B(r);
    sub_F552(r);
    /* JMP L_EFF0 -> RTS */
}
