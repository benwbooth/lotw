/* $AC6D (bank13) — begin/continue a player jump-launch. On the first frame of a
 * jump ($4F==0) it requires the "jump armed" flag $22==0 (else plain RTS), arms
 * the sound ($8F=$1B) and loads the jump duration $4F from stat_jump. Once
 * launched it PLA/PLA-drops the caller's frame (non-local return), marks $22=1,
 * decrements the timer, derives the horizontal step $4B = -(X>>2) from the entry
 * $4F-in-X, and slides the player to a valid spot via L_ADC7 (twice, once with
 * $49 cleared). On a valid landing (carry clear) it commits the new position
 * (player_x_fine/player_y) and L_ADE4; otherwise (L_ACAF) it cancels the jump
 * ($4F=$4E=0) and still L_ADE4. Finally L_ACBB: L_ACE0, L_AD3B, L_AD7A, RTS.
 *
 *   LDX $4F / BNE L_AC7E              ; already launched -> skip arm check
 *   LDA $22 / BEQ L_AC76 / RTS        ; not armed -> bail
 * L_AC76: LDA #$1B/STA $8F / LDA stat_jump/STA $4F
 * L_AC7E:
 *   PLA / PLA                         ; drop caller frame (non-local return)
 *   LDA #$01/STA $22
 *   DEC $4F
 *   TXA / LSR / LSR / EOR #$FF / CLC / ADC #$01 / STA $4B   ; $4B = -(X>>2)
 *   JSR L_ADC7 / BCC L_ACA1
 *   LDA #$00/STA $49 / JSR L_ADC7 / BCC L_ACA1 / JMP L_ACAF
 * L_ACA1: LDA $0E/STA player_x_fine / LDA $0A/STA player_y / JSR L_ADE4 / JMP L_ACBB
 * L_ACAF: LDA #$00/STA $4F/STA $4E / JSR L_ADE4
 * L_ACBB: JSR L_ACE0 / JSR L_AD3B / JSR L_AD7A / RTS
 *
 * INSPECTION-PORT (no diff-test spec): the PLA/PLA pops the caller's return
 * address — a non-local return that the flat Regs ABI cannot model. The X
 * register (the entry value of $4F) is also carried across the PLA/PLA into the
 * $4B computation. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_ADC7(Regs *r); void sub_ADE4(Regs *r); void sub_ACE0(Regs *r);
void sub_AD3B(Regs *r); void sub_AD7A(Regs *r);

void sub_AC6D(Regs *r)
{
    u8 x = RAM8(0x4F);                  /* LDX $4F */
    if (x == 0) {                       /* BNE L_AC7E skipped */
        if (RAM8(0x22) != 0)            /* LDA $22 / BEQ L_AC76 */
            return;                     /* armed flag set -> RTS */
        /* L_AC76 */
        RAM8(0x8F) = 0x1B;
        RAM8(0x4F) = RAM8(0x5C);        /* stat_jump */
    }

    /* L_AC7E — PLA / PLA drop caller frame (non-local return; see header) */
    RAM8(0x22) = 0x01;
    RAM8(0x4F) = (u8)(RAM8(0x4F) - 1);  /* DEC $4F */
    RAM8(0x4B) = (u8)(((u8)(x >> 2) ^ 0xFF) + 1);   /* TXA/LSR/LSR/EOR#$FF/+1 */

    sub_ADC7(r);                        /* JSR L_ADC7 */
    if (r->c) {                         /* BCS: blocked, retry with $49=0 */
        RAM8(0x49) = 0x00;
        sub_ADC7(r);
    }

    if (!r->c) {                        /* L_ACA1: valid landing */
        RAM8(0x43) = RAM8(0x0E);        /* player_x_fine <- $0E */
        RAM8(0x45) = RAM8(0x0A);        /* player_y     <- $0A */
        sub_ADE4(r);
    } else {                            /* L_ACAF: cancel jump */
        RAM8(0x4F) = 0x00;
        RAM8(0x4E) = 0x00;
        sub_ADE4(r);
    }

    /* L_ACBB */
    sub_ACE0(r);
    sub_AD3B(r);
    sub_AD7A(r);
}
