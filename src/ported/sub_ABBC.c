/* $ABBC (bank13) — warp-web presentation / player-launch driver. Far-called from
 * main_loop_dispatch's warp path ($EC) inside the A2EB/ABBC/A5E6/A75D/A3E3 loop.
 *
 * It folds the player-pad nibble ($20) into the facing/momentum byte $FD (clearing
 * the low nibble unless V of $20 is set, then OR-ing in the new direction), and —
 * on the launch frame ($85==0) gated by the frame status ($26) V flag and the
 * 3-frame phase mask ($3E&$06) — arms a web/jump: it picks a sound id ($0A/$05)
 * from $1C + the per-slot offset $040C,X vs $B0, plays it via L_AE2F, seeds the
 * launch timer $4F=$0A, sound $8F=$21, $90=$02, sets the launched flag $85=1, and
 * far-... (direct) JSR $CB7F. While launched it folds $20 to direction $02 and runs
 * the per-frame step: L_AE51, then depending on $4E/$4F/$20 it either continues the
 * jump via L_AC6D, advances horizontally via L_ADC7 with step $4B, and commits the
 * new position. All exit paths converge on the shared tail (L_ACA1 commit /
 * L_ACAF cancel) -> L_ACBB (L_ACE0 / L_AD3B / L_AD7A) — the same tail owned by
 * src/ported/sub_AC6D.c, replicated here for the JMP L_ACA1 / JMP L_ACAF entries.
 *
 * INSPECTION-PORT (no diff-test spec): warp/launch presentation reached only via
 * the main-loop warp far-call chain; depends on vblank-driven phase counters ($3E),
 * the frame status flag, and a `JSR L_AC6D` whose PLA/PLA non-local return drops
 * this routine's own frame (see sub_AC6D.c) — not isolation-testable in flat
 * memory. Integration-verified.
 *
 * Non-local target: JMP $AE11 (early bail when $20&$10 set) is provided by the
 * native coroutine layer. Direct JSR $CB7F is a fixed-bank routine (always
 * mapped) — already ported as src/ported/sub_CB7F.c. */
#include "ram.h"
#include "regs.h"

void sub_AE2F(Regs *r); void sub_AE51(Regs *r); void sub_ADC7(Regs *r);
void sub_AC6D(Regs *r); void sub_ADE4(Regs *r); void sub_ACE0(Regs *r);
void sub_AD3B(Regs *r); void sub_AD7A(Regs *r); void sub_CB7F(Regs *r);
void sub_AE11(Regs *r);         /* L_AE11: native press-start gate */

/* Shared tail owned by sub_AC6D.c; replicated for the JMP L_ACA1/L_ACAF entries. */
static void tail_acbb(Regs *r)          /* L_ACBB */
{
    sub_ACE0(r);
    sub_AD3B(r);
    sub_AD7A(r);
}
static void tail_aca1(Regs *r)          /* L_ACA1: commit new position */
{
    RAM8(0x43) = RAM8(0x0E);            /* player_x_fine <- $0E */
    RAM8(0x45) = RAM8(0x0A);            /* player_y     <- $0A */
    sub_ADE4(r);
    tail_acbb(r);
}
static void tail_acaf(Regs *r)          /* L_ACAF: cancel jump */
{
    RAM8(0x4F) = 0x00;
    RAM8(0x4E) = 0x00;
    sub_ADE4(r);
    tail_acbb(r);
}

void sub_ABBC(Regs *r)
{
    /* $ABBC */
    r->a = RAM8(0x20);
    if (r->a & 0x10) {                  /* AND #$10 / BEQ L_ABC5 / JMP L_AE11 */
        sub_AE11(r);                    /* non-local bail (warp cancel) */
        return;
    }
    /* L_ABC5 */
    if (!(RAM8(0x20) & 0x40)) {         /* BIT $20 / BVS L_ABCF */
        RAM8(0xFD) = (u8)(RAM8(0xFD) & 0x0F);
    }
    /* L_ABCF */
    r->a = (u8)(RAM8(0x20) & 0x0F);
    if (r->a != 0) {                    /* BEQ L_ABDF */
        RAM8(0x08) = r->a;
        RAM8(0xFD) = (u8)((RAM8(0xFD) & 0xF0) | RAM8(0x08));
    }

    /* L_ABDF */
    if (RAM8(0x85) == 0) {              /* BNE L_AC13 */
        /* launch-frame gate */
        if ((RAM8(0x26) & 0x40) == 0)   /* BIT frame status / BVC L_AC2A */
            goto L_AC2A;
        r->x = (u8)(RAM8(0x3E) + 1);    /* LDX $3E / INX */
        if (((r->x) & 0x06) != 0)       /* TXA / AND #$06 / BNE L_AC2A */
            goto L_AC2A;
        {                               /* sound id from ($1C + $040C,X) vs $B0 */
            u8 sum = (u8)(RAM8(0x1C) + RAM8((u16)(0x040C + r->x)));  /* CLC/ADC */
            r->a = (sum < 0xB0) ? 0x0A : 0x05;      /* CMP #$B0; LDA #$0A/BCC/#$05 */
        }
        sub_AE2F(r);                    /* JSR L_AE2F */
        RAM8(0x4F) = 0x0A;
        RAM8(0x8F) = 0x21;
        RAM8(0x90) = 0x02;
        RAM8(0x85) = 0x01;
        sub_CB7F(r);                    /* JSR $CB7F (fixed bank) */
    }

    /* L_AC13 */
    if (RAM8(0x4F) == 0 && RAM8(0x4E) == 0) {   /* both zero -> clear $85 */
        RAM8(0x85) = 0x00;
        goto L_AC2A;
    }
    /* L_AC22 */
    RAM8(0x20) = (u8)((RAM8(0x20) & 0xF0) | 0x02);

L_AC2A:
    sub_AE51(r);                        /* JSR L_AE51 */
    if (RAM8(0x4E) != 0) {              /* BNE L_AC52 */
        /* L_AC52 */
        r->a = (u8)(RAM8(0x4E) >> 2);   /* LSR / LSR (on A=$4E) */
        r->a = (u8)(r->a + 1);          /* CLC / ADC #$01 */
        RAM8(0x4B) = r->a;
        sub_ADC7(r);                    /* JSR L_ADC7 */
        if (!r->c) {                    /* BCS L_AC61 ; else fall to JMP L_ACA1 */
            tail_aca1(r);
            return;
        }
        /* L_AC61 */
        RAM8(0x49) = 0x00;
        sub_ADC7(r);
        if (!r->c) { tail_aca1(r); return; }    /* BCC L_ACA1 */
        tail_acaf(r);                   /* JMP L_ACAF */
        return;
    }

    /* $4E == 0 */
    if (RAM8(0x4F) != 0) {              /* BNE L_AC39 */
        goto L_AC39;
    }
    if (!(RAM8(0x20) & 0x80)) {         /* LDA $20 / BPL L_AC41 */
        /* L_AC41 */
        RAM8(0x22) = 0x00;
        r->a = 0x00;                    /* (A already 0 via STA path) */
        goto L_AC45;
    }
L_AC39:
    sub_AC6D(r);                        /* JSR L_AC6D — PLA/PLA may not return (see hdr) */
    r->a = 0x00;                        /* LDA #$00 (only on early-RTS path) */
L_AC45:
    RAM8(0x4F) = r->a;                  /* STA $4F (A==0 here) */
    sub_ADC7(r);                        /* JSR L_ADC7 */
    if (r->c) {                         /* BCC L_AC4F ; else JMP L_ACAF */
        tail_acaf(r);
        return;
    }
    /* L_AC4F */
    tail_aca1(r);                       /* JMP L_ACA1 */
}
