/* $ED9F:
 *   DEC $F1 / BEQ L_EDEB / LDA $F4 / BNE L_EDAD / JSR L_EE53 / JMP L_EDD0
 *   L_EDAD: LDA $F3 / CMP #$08 / BCC L_EDD0 / LDA $F4 / STA $08 / JSR L_EE53
 *           LDA $F4 / EOR $08 / LDY #$00 / LDX #$04
 *   L_EDC2: LSR A / BCC L_EDC6 / INY
 *   L_EDC6: DEX / BNE L_EDC2 / DEY / BEQ L_EDD0 / LDA $08 / STA $F4
 *   L_EDD0: LDY #$09 / LDA ($E7),Y / TAY / LDA $F4 / JSR L_CD70 / JSR L_F11B
 *           BCC L_EDE2 / JMP L_EDEB
 *   L_EDE2: JSR L_EF04 / JSR L_F01E / JMP L_EFF0 (RTS)
 *   L_EDEB: LDA #$00 / STA $EE / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_EE53(Regs *);
void sub_CD70(Regs *);
void sub_F11B(Regs *);
void sub_EF04(Regs *);
void sub_F01E(Regs *);

void sub_ED9F(Regs *r)
{
    u8 dec = (u8)(RAM8(0xF1) - 1);
    RAM8(0xF1) = dec;
    if (dec == 0)                        /* BEQ L_EDEB */
        goto edeb;

    if (RAM8(0xF4) == 0) {               /* BNE L_EDAD else fall through */
        sub_EE53(r);
        /* JMP L_EDD0 */
    } else {
        /* L_EDAD */
        if (RAM8(0xF3) >= 0x08) {        /* BCC L_EDD0 skips */
            u8 a, x, y;
            RAM8(0x08) = RAM8(0xF4);
            sub_EE53(r);
            a = (u8)(RAM8(0xF4) ^ RAM8(0x08));
            y = 0x00;
            x = 0x04;
            do {                          /* L_EDC2 */
                u8 c = a & 1;
                a >>= 1;
                if (c) y++;
            } while (--x != 0);
            y--;                          /* DEY */
            if (y != 0)                   /* BEQ L_EDD0 skips this */
                RAM8(0xF4) = RAM8(0x08);
        }
    }

    /* L_EDD0 */
    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        r->y = RAM8((u16)(ptr + 0x09));
        r->a = RAM8(0xF4);
        sub_CD70(r);
    }
    sub_F11B(r);
    if (r->c)                            /* BCC L_EDE2 else JMP L_EDEB */
        goto edeb;

    /* L_EDE2 */
    sub_EF04(r);
    sub_F01E(r);
    return;                              /* JMP L_EFF0 -> RTS */

edeb:
    r->a = 0x00;
    RAM8(0xEE) = 0x00;
}
