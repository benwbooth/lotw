/* $E9A5 — enemy/boss spawn tick. Decrements timer $F3; while the timer is in
 * range it picks a candidate spawn position (random via rng_update, or from the
 * record at ($E7)), validates it with CE7C + F23A, and on success commits the
 * spawn (writes $F9/$FA/$FB and a batch of state vars). Tail-jumps into the
 * L_EA30 fragment (inlined) which sets up $EE/$ED/$EF or toggles $EF.
 *
 *   DEC $F3 / LDX $F3 / CPX #$3C / BCS ret
 *   LDY 2 / ($E7),2 / INY / ORA ($E7),3 / BNE useRec
 *     LDA #$0C / rng_update / ASL*4 / STA $0A ; LDA #$40 / rng_update / STA $0F
 *     JMP cont
 *   useRec: ($E7),3 -> $0A ; ($E7),2 -> $0F
 *   cont: 0 -> $0E,$0B ; JSR CE7C / BCS ret ; JSR F23A / BCC commit ; ret: RTS
 *   commit: $0E->$F9,$0F->$FA,$0A->$FB ; 0->$F1,$F0,$F4,$FC
 *           ($E7),4->boss_life ; ($E7),5->$F8
 *           A = 1<<cur_character ; AND $41 ; BNE skip ; ASL $F8 ; BCS->$F8=$FF
 *   skip: $7F->$EE,$F9->$ED,$01->$EF ; LDA $F3 ; (inlined L_EA30)
 */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);
void sub_CE7C(Regs *r);
void sub_F23A(Regs *r);

void sub_E9A5(Regs *r)
{
    u8 x, t;
    u16 e7;

    RAM8(0xF3) = (u8)(RAM8(0xF3) - 1);     /* DEC $F3 */
    x = RAM8(0xF3);                        /* LDX $F3 */
    if (x >= 0x3C) { return; }             /* CPX #$3C / BCS L_E9E6 */

    e7 = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));

    /* LDY 2 / LDA ($E7),Y / INY / ORA ($E7),Y / BNE L_E9CB */
    if ((RAM8((u16)(e7 + 2)) | RAM8((u16)(e7 + 3))) == 0) {
        r->a = 0x0C;
        rng_update(r);
        RAM8(0x0A) = (u8)(r->a << 4);      /* ASL A x4 */
        r->a = 0x40;
        rng_update(r);
        RAM8(0x0F) = r->a;
    } else {
        /* L_E9CB */
        RAM8(0x0A) = RAM8((u16)(e7 + 3));
        RAM8(0x0F) = RAM8((u16)(e7 + 2));
    }

    /* L_E9D6 */
    RAM8(0x0E) = 0x00;
    RAM8(0x0B) = 0x00;

    sub_CE7C(r);
    if (r->c) { return; }                  /* BCS L_E9E6 */

    sub_F23A(r);
    if (r->c) { return; }                  /* BCC L_E9E7 (continue); else ret */

    /* L_E9E7: commit */
    RAM8(0xF9) = RAM8(0x0E);
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xFB) = RAM8(0x0A);
    RAM8(0xF1) = 0x00;
    RAM8(0xF0) = 0x00;
    RAM8(0xF4) = 0x00;
    RAM8(0xFC) = 0x00;
    RAM8(0xF2) = RAM8((u16)(e7 + 4));      /* boss_life = $00F2 */
    RAM8(0xF8) = RAM8((u16)(e7 + 5));

    /* LDX cur_character / LDA #0 / SEC / ROL A; DEX; BPL -> A = 1<<cur_character */
    {
        u8 a = 0x00;
        u8 c = 1;
        u8 xi = RAM8(0x40);                /* LDX cur_character */
        do {                               /* ROL A / DEX / BPL (8-bit N of X) */
            u8 nc = (u8)((a >> 7) & 1);
            a = (u8)((a << 1) | c);
            c = nc;
            xi = (u8)(xi - 1);
        } while ((xi & 0x80) == 0);
        a = (u8)(a & RAM8(0x41));           /* AND $41 */
        if (a == 0) {                       /* BNE L_EA1D skips this */
            /* ASL $F8 / BCC L_EA1D / LDA #$FF / STA $F8 */
            u8 f8 = RAM8(0xF8);
            u8 carry = (u8)((f8 >> 7) & 1);
            RAM8(0xF8) = (u8)(f8 << 1);
            if (carry)
                RAM8(0xF8) = 0xFF;
        }
    }

    /* L_EA1D */
    RAM8(0xEE) = 0x7F;
    RAM8(0xED) = 0xF9;
    RAM8(0xEF) = 0x01;

    /* LDA $F3 / JMP L_EA30 (inlined) */
    t = RAM8(0xF3);
    if (t == 0) {                           /* BNE L_EA42 not taken */
        RAM8(0xEE) = 0x01;
        RAM8(0xED) = RAM8((u16)(e7 + 0));
        RAM8(0xEF) = RAM8((u16)(e7 + 1));
    } else {
        /* L_EA42 */
        if ((RAM8(0xF3) & 0x03) == 0) {     /* BNE L_EA4E not taken */
            RAM8(0xEF) = (u8)(RAM8(0xEF) ^ 0x40);
        }
    }
}
