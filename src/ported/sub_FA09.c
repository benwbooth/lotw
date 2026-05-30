/* $FA09  (music channel tick)
 *   L_FA09: LDA $B4 / BMI L_FA10 / JMP L_FA54
 *   L_FA10: DEC $B3 / BEQ L_FA15 / RTS
 *   L_FA15: LDY #$00 / LDA ($B5),Y / BEQ L_FA27 / PHP / CMP #$FF / BNE L_FA2D
 *           PLP / JSR L_FB8E / JMP L_FA15
 *   L_FA27: JSR L_FCF9 / JMP L_FA54
 *   L_FA2D: JSR L_FD6B / AND #$7F / STA $B3 / PLP / BMI L_FA54
 *           JSR L_FC81 / LDA $27 / ORA #$04 / STA $27
 *           LDA $BA / STA TRI_LINEAR / LDA $04 / STA TRI_LO
 *           LDA $05 / AND #$07 / ORA #$F8 / STA TRI_HI / RTS
 *   L_FA54: LDA #$00 / STA TRI_LINEAR / LDA $27 / AND #$FB / STA $27 / RTS
 *
 * The PHP/PLP carries the N flag (bit7) of the just-loaded command byte across
 * the FD6B/AND/STA at FA2D, where BMI tests it.
 * TRI_LINEAR/TRI_LO/TRI_HI are APU regs ($4008/$400A/$400B) -> REG_W (host-ignored).
 */
#include "ram.h"
#include "regs.h"

#define TRI_LINEAR 0x4008
#define TRI_LO     0x400A
#define TRI_HI     0x400B

void sub_FB8E(Regs *);
void sub_FCF9(Regs *);
void inc16_95(Regs *);   /* $FD6B */
void sub_FC81(Regs *);

static void fa54(Regs *r)
{
    r->a = 0x00;
    REG_W(TRI_LINEAR, 0x00);
    RAM8(0x27) = RAM8(0x27) & 0xFB;
    r->a = RAM8(0x27);
}

void sub_FA09(Regs *r)
{
    if ((RAM8(0xB4) & 0x80) == 0) {      /* BMI L_FA10 else JMP L_FA54 */
        fa54(r);
        return;
    }

    /* L_FA10 */
    if ((u8)(RAM8(0xB3) - 1) != 0) {     /* DEC $B3 / BEQ L_FA15 */
        RAM8(0xB3) = (u8)(RAM8(0xB3) - 1);
        return;                          /* RTS */
    }
    RAM8(0xB3) = (u8)(RAM8(0xB3) - 1);   /* now 0 */

    /* L_FA15 loop */
    for (;;) {
        u16 ptr = (u16)(RAM8(0xB5) | (RAM8(0xB6) << 8));
        u8 cmd = RAM8(ptr);              /* LDA ($B5),Y, Y=0 */
        if (cmd == 0) {                  /* BEQ L_FA27 */
            sub_FCF9(r);
            fa54(r);
            return;
        }
        if (cmd != 0xFF) {               /* CMP #$FF / BNE L_FA2D */
            /* L_FA2D: A = cmd here; saved N flag = bit7 of cmd (PHP/PLP) */
            u8 saved_n = (u8)(cmd & 0x80);
            r->a = cmd;
            inc16_95(r);                 /* $FD6B: X=$02; inc16 ($95+X); A preserved */
            r->a = (u8)(cmd & 0x7F);     /* AND #$7F */
            RAM8(0xB3) = r->a;
            if (saved_n) {               /* BMI L_FA54 */
                fa54(r);
                return;
            }
            sub_FC81(r);
            RAM8(0x27) = RAM8(0x27) | 0x04;
            REG_W(TRI_LINEAR, RAM8(0xBA));
            REG_W(TRI_LO, RAM8(0x04));
            r->a = (u8)((RAM8(0x05) & 0x07) | 0xF8);
            REG_W(TRI_HI, r->a);
            return;                      /* RTS */
        }
        /* cmd == $FF: PLP / JSR L_FB8E / loop */
        sub_FB8E(r);
    }
}
