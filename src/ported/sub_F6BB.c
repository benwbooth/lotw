/* $F6BB: projectile/object step. Restores its 16-byte work buffer ($00ED..) via
 * sub_E98F, decrements life counter $EE, and while alive moves it (EFF1), tests
 * out-of-bounds (CF08) and sprite collision (CDB2), resolving hits / despawn.
 * Finishes by writing back position ($F9/$FA/$FB) and committing via E99A.
 * Pure RAM effects (plus PPU/APU side writes inside callees).
 *
 *   JSR L_E98F
 *   DEC $EE / BEQ L_F735
 *   JSR L_EFF1 / JSR L_CF08 / BCS L_F722        ; off-screen -> kill
 *   JSR L_CDB2 / BCC L_F729                      ; no collision -> just store pos
 *   LDA $2D / CMP #$30 / BCC L_F6ED              ; (mmc3_r3_shadow)
 *   LDA $08 / CMP #$04 / BCC L_F6ED
 *   LDX $09 / LDA #$80 STA $0401,X / $EE=1 / $8F=$0C / JMP L_F71F   ; despawn target
 * L_F6ED:
 *   LDY $0401,X / DEY / BNE L_F729               ; target hp != 1 -> ignore
 *   LDX $09 / LDA $EE / LDY #$FE / AND #$01 / BEQ L_F6FF / LDY #$02
 * L_F6FF:
 *   TYA / STA $040F,X
 *   LDA $0405,X / SEC / SBC $F8 / STA $0405,X / BCS L_F71B
 *   LDA #$80 STA $0401,X / LDA #$00 STA $0405,X / JMP L_F71F
 * L_F71B: LDA #$06 / STA $8F
 * L_F71F: JMP L_F729
 * L_F722: LDA #$00 / STA $EE / JMP L_F735
 * L_F729: $F9=$0E / $FA=$0F / $FB=$0A
 * L_F735: LDA $EE / BEQ L_F73C / JSR L_F773
 * L_F73C: JSR L_E99A / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r);
void sub_EFF1(Regs *r);
void sub_CF08(Regs *r);
void sub_CDB2(Regs *r);
void sub_F773(Regs *r);
void sub_E99A(Regs *r);

static void store_pos(void)        /* L_F729 */
{
    RAM8(0xF9) = RAM8(0x0E);
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xFB) = RAM8(0x0A);
}

static void finish(Regs *r)        /* L_F735 */
{
    if (RAM8(0xEE) != 0)
        sub_F773(r);
    sub_E99A(r);
}

void sub_F6BB(Regs *r)
{
    sub_E98F(r);

    RAM8(0xEE) = (u8)(RAM8(0xEE) - 1);   /* DEC $EE */
    if (RAM8(0xEE) == 0) { finish(r); return; }

    sub_EFF1(r);

    sub_CF08(r);
    if (r->c) {                          /* L_F722 */
        RAM8(0xEE) = 0x00;
        finish(r);
        return;
    }

    sub_CDB2(r);
    if (!r->c) {                         /* BCC L_F729 (no collision) */
        store_pos();
        finish(r);
        return;
    }

    /* collision: $08=slot, $09=table offset */
    if (RAM8(0x2D) >= 0x30 && RAM8(0x08) >= 0x04) {   /* $2D = mmc3_r3_shadow */
        u8 x = RAM8(0x09);
        RAM8((u16)(0x0401 + x)) = 0x80;
        RAM8(0xEE) = 0x01;
        RAM8(0x8F) = 0x0C;
        store_pos();                     /* L_F71F -> L_F729 */
        finish(r);
        return;
    }

    /* L_F6ED */
    {
        u8 x = RAM8(0x09);
        if ((u8)(RAM8((u16)(0x0401 + x)) - 1) != 0) {   /* LDY $0401,X / DEY / BNE L_F729 */
            store_pos();
            finish(r);
            return;
        }
        /* target hp == 1 */
        x = RAM8(0x09);                  /* LDX $09 */
        {
            u8 yv = (RAM8(0xEE) & 0x01) ? 0x02 : 0xFE;   /* LDA $EE / AND #$01 */
            RAM8((u16)(0x040F + x)) = yv;                /* TYA / STA $040F,X */
        }
        {
            u8 cur = RAM8((u16)(0x0405 + x));
            u8 sub = RAM8(0xF8);
            RAM8((u16)(0x0405 + x)) = (u8)(cur - sub);   /* SBC $F8 */
            if (cur >= sub) {            /* BCS L_F71B */
                RAM8(0x8F) = 0x06;
            } else {
                RAM8((u16)(0x0401 + x)) = 0x80;
                RAM8((u16)(0x0405 + x)) = 0x00;
            }
        }
        store_pos();                     /* L_F71F -> L_F729 */
        finish(r);
    }
}
