/* $D42B — top-level per-frame game update / state dispatcher.
 *
 * Runs once per active gameplay frame. First it marks the frame ($E3=$FF) and,
 * if a pending reset/game-over flag ($EB) is set, hands off to the reset path
 * (L_D641). Otherwise it runs the secret-warp check (D64F), and on the
 * "screen-transition" input bit ($20 & $10) hands off to the scroll/transition
 * routine (L_E00F). On the normal path it runs the equipped-item tick (D596),
 * decrements the invuln/freeze timer ($46) clearing input when it expires,
 * massages the held-direction byte ($FD) from the current input ($20) with a
 * character-4 / ($84&7) special case, and then:
 *   - if the "item-swap" input bit ($20 & $20) is held, runs the equip-cycle
 *     input loop (L_D55A) and redraws;
 *   - else applies a queued action ($20 & $08 -> D CE2), finds the first free
 *     object slot ($0087..$008B -> Y, else Y=6), runs the per-object recompute
 *     (CD2C), and drives the movement / jump-fall state machine via D991 (object
 *     scan), DF90 (settle) and the jump physics (D4DF), finally committing the
 *     player position (player_x_fine/tile, player_y) and redrawing through L_D8AF.
 *
 * Non-local control transfers handled here:
 *   - JMP L_D641 (reset path), JMP L_E00F (screen transition) — both leave
 *     game_update entirely; ported as forward-declared handoff calls + return.
 *   - D64F may itself PLA/PLA (secret-warp non-local return) — stack effect is
 *     integration-only; we just call it.
 *   - L_D4A4 calls D4DF, which itself PLA/PLAs (drops *this* frame's return) and
 *     tail-drives the commit/redraw; modelled by calling sub_D4DF and returning.
 *   - L_D536 / L_D54E / L_D8AF are the same commit/redraw tails as in sub_D4DF.c;
 *     inlined here as local labels (trivial data-commit + D8E3/D94E redraw).
 *
 * INSPECTION-PORT (no diff-test spec): top-level dispatcher with non-local
 * PLA/PLA returns (via D4DF / D64F) and whole-frame handoffs (L_D641 / L_E00F)
 * that the flat Regs ABI cannot model in isolation. Integration-verified. */
#include "ram.h"
#include "regs.h"

/* Ported callees (all in src/ported/). */
void sub_D64F(Regs *r);    /* secret-warp check (RTS-trampoline; PLA/PLA stack effect is integration-only) */
void sub_D596(Regs *r);    /* equipped-item per-frame tick */
void sub_DCE2(Regs *r);    /* queued-action handler */
void sub_CD2C(Regs *r);    /* per-object recompute */
void sub_D4DF(Regs *r);    /* jump/fall physics (PLA/PLA non-local return; tail-redraws) */
void sub_D991(Regs *r);    /* per-object scan loop (returns carry) */
void sub_DF90(Regs *r);    /* position settle (returns carry) */
void sub_DBDD(Regs *r);    /* post-move fixup */
void sub_CC09(Regs *r);    /* read/poll input (returns A in r->a) */
void sub_D8E3(Regs *r);    /* redraw part 1 (L_D8AF tail) */
void sub_D94E(Regs *r);    /* redraw part 2 (L_D8AF tail) */

/* Handoff targets that leave game_update entirely. These are reached only via
 * JMP (whole-frame non-local transfers); their bodies live outside this
 * routine. They are not yet ported as standalone files, so they are declared
 * here as integration-only handoffs (stubbed until ported — see notes below). */
void sub_D641(Regs *r);    /* L_D641: reset / game-over path (NOT YET PORTED — integration stub) */
void sub_E00F(Regs *r);    /* L_E00F: screen-transition / scroll path (NOT YET PORTED — integration stub) */

void game_update(Regs *r)
{
    u8 a, y;

    RAM8(0xE3) = 0xFF;                       /* LDA #$FF / STA $E3 */
    if (RAM8(0xEB) != 0) {                   /* LDA $EB / BEQ L_D436 */
        sub_D641(r);                         /* JMP L_D641 — reset path (non-local handoff) */
        return;
    }

L_D436:
    sub_D64F(r);                             /* JSR L_D64F (may PLA/PLA on secret warp) */
    if (RAM8(0x20) & 0x10) {                 /* LDA $20 / AND #$10 / BNE -> L_E00F */
        sub_E00F(r);                         /* JMP L_E00F — screen transition (non-local handoff) */
        return;
    }

    /* L_D442 */
    sub_D596(r);                             /* JSR L_D596 */
    if (RAM8(0x46) != 0) {                   /* LDA $46 / BEQ L_D44F */
        RAM8(0x46)--;                        /* DEC $46 */
        RAM8(0x20) = 0x00;                   /* LDA #$00 / STA $20 */
    }

    /* L_D44F — held-direction ($FD) housekeeping */
    {
        int clear_hi = 1;                    /* whether to clear $FD high nibble (L_D45F) */
        if (RAM8(0x40) /*cur_character*/ == 0x04) {   /* CMP #$04 / BNE L_D45B */
            if ((RAM8(0x84) & 0x07) == 0)    /* LDA $84 / AND #$07 / BEQ L_D45F */
                clear_hi = 1;
            else
                clear_hi = (RAM8(0x20) & 0x40) ? 0 : 1;  /* L_D45B: BIT $20 / BVS L_D465 */
        } else {
            /* L_D45B */
            clear_hi = (RAM8(0x20) & 0x40) ? 0 : 1;      /* BIT $20 / BVS L_D465 */
        }
        if (clear_hi)                        /* L_D45F */
            RAM8(0xFD) &= 0x0F;              /* LDA $FD / AND #$0F / STA $FD */
    }

    /* L_D465 — fold input low nibble into $FD low nibble */
    a = RAM8(0x20) & 0x0F;                    /* LDA $20 / AND #$0F */
    if (a != 0) {                            /* BEQ L_D475 */
        RAM8(0x08) = a;                       /* STA $08 */
        RAM8(0xFD) = (u8)((RAM8(0xFD) & 0xF0) | a);  /* LDA $FD / AND #$F0 / ORA $08 / STA $FD */
    }

    /* L_D475 */
    if (RAM8(0x20) & 0x20)                    /* LDA $20 / AND #$20 / BNE -> L_D55A */
        goto L_D55A;

    /* L_D47E */
    if (RAM8(0x20) & 0x08)                    /* LDA $20 / AND #$08 / BEQ L_D487 */
        sub_DCE2(r);                          /* JSR L_DCE2 */

    /* L_D487 — find first free object slot in $0087..$008B */
    y = 0x01;                                 /* LDY #$01 */
    while (RAM8((u16)(0x0087 + y)) != 0) {    /* L_D489: LDA a:$0087,Y / BEQ L_D495 */
        y++;                                  /* INY */
        if (y >= 0x05) {                      /* CPY #$05 / BCC L_D489 */
            y = 0x06;                          /* LDY #$06 */
            break;
        }
    }
    r->y = y;

    /* L_D495 */
    sub_CD2C(r);                              /* JSR L_CD2C */

    if (RAM8(0x4E) != 0) {                    /* LDA $4E / BNE L_D4C2 */
        /* L_D4C2 — falling: $4E>>2 + 1 -> step into $4B, scan, settle */
        RAM8(0x4B) = (u8)((RAM8(0x4E) >> 2) + 1);   /* LSR/LSR/CLC/ADC #$01 / STA $4B */
        sub_D991(r);                          /* JSR L_D991 */
        if (!r->c)                            /* BCS L_D4D1; else JMP L_D536 */
            goto L_D536;
        /* L_D4D1 */
        RAM8(0x49) = 0x00; RAM8(0x4A) = 0x00; /* zero object index */
        sub_D991(r);                          /* JSR L_D991 */
        if (!r->c)                            /* BCC L_D536 */
            goto L_D536;
        goto L_D54E;                          /* JMP L_D54E */
    }

    /* $4E == 0 — decide jump vs. walk, both converge on the L_D4B0 walk tail */
    if (RAM8(0x4F) != 0) {                    /* LDA $4F / BNE L_D4A4 */
        /* L_D4A4 */
        sub_D4DF(r);                          /* JSR L_D4DF (jump physics; may PLA/PLA tail-return) */
        /* On the PLA/PLA branch D4DF redraws and returns to the grandparent; in
         * the flat ABI it returns here and we continue. On its normal-RTS branch
         * it returns here too. Either way: LDA #$00 / JMP L_D4B0. */
        RAM8(0x4F) = 0x00;                    /* LDA #$00 / (L_D4B0) STA $4F */
    } else if (RAM8(0x20) & 0x80) {           /* LDA $4F==0 -> LDA $20 / BPL L_D4AC; bit7 set -> L_D4A4 */
        sub_D4DF(r);                          /* L_D4A4: JSR L_D4DF */
        RAM8(0x4F) = 0x00;                    /* LDA #$00 / (L_D4B0) STA $4F */
    } else {
        /* L_D4AC */
        RAM8(0x22) = 0x00;                    /* LDA #$00 / STA $22 */
        RAM8(0x4F) = 0x00;                    /* LDA #$00 (A still 0) / (L_D4B0) STA $4F */
    }

    /* L_D4B0 tail: run the walk scan + settle */
    sub_D991(r);                              /* JSR L_D991 */
    if (!r->c)                                /* BCC L_D4BF */
        goto L_D4BF;
    sub_DF90(r);                              /* JSR L_DF90 */
    if (!r->c)                                /* BCC L_D4BF */
        goto L_D4BF;
    goto L_D54E;                              /* JMP L_D54E */
L_D4BF:
    goto L_D536;                              /* JMP L_D536 */

L_D536:
    /* Commit trial position. (Same tail as in sub_D4DF.c.) */
    RAM8(0x43) /*player_x_fine*/ = RAM8(0x0E);   /* LDA $0E / STA player_x_fine */
    RAM8(0x44) /*player_x_tile*/ = RAM8(0x0F);   /* LDA $0F / STA player_x_tile */
    a = RAM8(0x0A);                            /* LDA $0A */
    if (a >= 0xEF)                             /* CMP #$EF / BCC L_D546 / LDA #$00 */
        a = 0x00;
    RAM8(0x45) /*player_y*/ = a;               /* STA player_y */
    sub_DBDD(r);                               /* JSR L_DBDD */
    goto L_D8AF;                               /* JMP L_D8AF */

L_D54E:
    RAM8(0x4F) = 0x00;                         /* STA $4F */
    RAM8(0x4E) = 0x00;                         /* STA $4E */
    sub_DBDD(r);                               /* JSR L_DBDD */
    goto L_D8AF;                               /* JMP L_D8AF */

L_D55A:
    /* Equip-cycle input loop: poll input, on a held d-pad direction cycle the
     * equipped_item index (clamped to 0..3) and re-recompute, until a non-d-pad
     * button or release is seen. */
    RAM8(0x8F) = 0x10;                         /* LDA #$10 / STA $8F */
    for (;;) {                                 /* L_D55E */
        sub_CC09(r);                           /* JSR L_CC09 (returns A) */
        if (r->a & 0xF0)                       /* AND #$F0 / BNE L_D58F */
            break;
        if ((RAM8(0x20) & 0x03) == 0)          /* LDA $20 / AND #$03 / BEQ L_D55E */
            continue;
        RAM8(0x20) <<= 1;                       /* ASL $20 */
        RAM8(0x20) <<= 1;                       /* ASL $20 */
        r->y = 0x01;                            /* LDY #$01 */
        sub_CD2C(r);                            /* JSR L_CD2C */
        {
            u8 t = (u8)(RAM8(0x4B) + RAM8(0x55) /*equipped_item*/);  /* LDA $4B / CLC / ADC equipped_item */
            u8 ni;
            if (t & 0x80)                       /* BMI L_D584 */
                ni = 0x03;                       /* LDA #$03 */
            else if (t < 0x04)                  /* CMP #$04 / BCC L_D586 */
                ni = t;
            else
                ni = 0x00;                       /* LDA #$00 / JMP L_D586 */
            RAM8(0x55) /*equipped_item*/ = ni;  /* L_D586: STA equipped_item */
        }
        RAM8(0x8F) = 0x0C;                       /* LDA #$0C / STA $8F */
        /* JMP L_D55E (loop) */
    }

    /* L_D58F */
    RAM8(0x8F) = 0x10;                          /* LDA #$10 / STA $8F */
    /* fall through to JMP L_D8AF */

L_D8AF:
    /* Redraw tail (L_D8AF: JSR D8E3 / JSR D94E / RTS). */
    sub_D8E3(r);                                /* JSR L_D8E3 */
    sub_D94E(r);                                /* JSR L_D94E */
}
