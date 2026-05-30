/* $C909 text_attr_build — reads a structured record through pointer $77/$78 and
 * populates a set of zero-page / $04xx fields plus MMC3 shadows. Calls sub_CA1E
 * (returns A, plus carry from its final ASL — recomputed here) and sub_D02E.
 *
 *   LDY #$00 / LDA ($77),Y / ADC #$A0 / STA $7A  (ADC uses caller's carry)
 *   ... reads bytes [0..6] into $7A,$2D,$70,$71,$74,$2A,$2B
 *   LDY #$07 / JSR L_CA1E / LDA #$00 / BCC + / LDA ($77),Y  ($04A1 = A or 0)
 *   ... conditional record [7..10] block, then $8E gating + JSR L_D02E,
 *   then bytes [$10..$14] into $80-$83,$41.
 */
#include "ram.h"
#include "regs.h"

void sub_CA1E(Regs *r);
void sub_D02E(Regs *r);

void text_attr_build(Regs *r)
{
    u16 p = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));
    u8 carry_in = r->c;
    u8 b;

    /* Y=0: ($77),0 + #$A0 + C -> $7A ; $79 = 0 */
    b = RAM8(p);
    RAM8(0x7A) = (u8)(b + 0xA0 + carry_in);
    RAM8(0x79) = 0;
    /* Y=1 */
    RAM8(0x2D) = RAM8((u16)(p + 1));   /* mmc3_r3_shadow */
    /* Y=2 */
    RAM8(0x70) = RAM8((u16)(p + 2));
    /* Y=3 */
    RAM8(0x71) = RAM8((u16)(p + 3));
    /* Y=4 */
    RAM8(0x74) = RAM8((u16)(p + 4));
    /* Y=5: ORA #$00 -> just store */
    RAM8(0x2A) = RAM8((u16)(p + 5));   /* mmc3_r0_shadow */
    /* Y=6 */
    RAM8(0x2B) = RAM8((u16)(p + 6));   /* mmc3_r1_shadow */

    /* LDY #$07 ; JSR L_CA1E ; carry = bit shifted out by CA1E's final ASL */
    {
        u8 ms_y = RAM8(0x48);
        u8 ms_x = RAM8(0x47);
        u8 idx = (u8)(((ms_y << 2) & 0x04) | ms_x);
        u8 a = RAM8((u16)(0x0300 + idx));
        u8 cnt = (u8)((ms_y >> 1) + 1);
        /* carry out of cnt left-shifts = bit (8-cnt) ... track per shift */
        u8 c = 0;
        do {
            c = (u8)((a >> 7) & 1);
            a = (u8)(a << 1);
        } while (--cnt != 0);
        r->a = a;       /* CA1E result (oracle runs real CA1E) */
        r->c = c;
    }

    {
        u8 y = 0x07;
        u8 a;
        /* LDA #$00 / BCC L_C942 / LDA ($77),Y */
        if (r->c)
            a = RAM8((u16)(p + y));
        else
            a = 0;
        RAM8(0x04A1) = a;
        if (a != 0) {                  /* BEQ L_C973 (skip if zero) */
            RAM8(0x04A2) = 0x01;
            y++;                       /* Y=8 */
            RAM8(0x04AD) = RAM8((u16)(p + y));
            RAM8(0x04AC) = 0x00;
            y++;                       /* Y=9 */
            RAM8(0x04AE) = RAM8((u16)(p + y));
            y++;                       /* Y=$0A */
            b = RAM8((u16)(p + y));
            if (b == 0x17) {
                RAM8(0x04A1) = 0x19;
                RAM8(0x04A0) = 0xDD;
            } else {
                RAM8(0x04A0) = 0xE9;
            }
        }
    }

    /* L_C973: LDX $8E / CPX #$05 / BCS L_C986 */
    {
        u8 x = RAM8(0x8E);
        u8 do_d02e = 1;
        if (x < 0x05) {
            /* LDA #$00 / SEC / ROL A; DEX; BPL (x+1 iterations) -> A = 1<<x */
            u8 a = 0x00;
            u8 c = 1;        /* SEC */
            int i = (int)x;
            do {
                u8 nc = (u8)((a >> 7) & 1);
                a = (u8)((a << 1) | c);
                c = nc;
                --i;
            } while (i >= 0);
            a = (u8)(a & RAM8((u16)(p + 0x15)));
            if (a != 0)
                do_d02e = 0;           /* BNE L_C98D : skip D02E */
        }
        if (do_d02e) {
            /* L_C986: LDY #$0B / LDA ($77),Y / JSR L_D02E */
            r->a = RAM8((u16)(p + 0x0B));
            sub_D02E(r);
        }
    }

    /* L_C98D */
    RAM8(0x80) = RAM8((u16)(p + 0x10));
    RAM8(0x81) = RAM8((u16)(p + 0x11));
    RAM8(0x82) = RAM8((u16)(p + 0x12));
    RAM8(0x83) = RAM8((u16)(p + 0x13));
    RAM8(0x41) = RAM8((u16)(p + 0x14));
}
