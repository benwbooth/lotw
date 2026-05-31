/* $B13D — resume saved game / load-game transition screen. Bumps the game-phase
 * counter $92, resets state (B29B/C461/C38B/B2EE), programs MMC3 CHR banks
 * ($20/$22 into r0/r1), enables PPU bits ($24|=$18), clears the screen via a $FF
 * VRAM fill ($CC8F queue), inits the song ($FC08, $8E=$0A), zeroes scroll/temps,
 * runs B2CC, and sets up the $B79C source pointer for the B25D/B1EA/B215 reveal
 * loop. After the reveal it waits on $D4 (frame edge), the $36 timer ($3C frames),
 * clears the four reveal flags ($94/$A4/$B4/$C4), and runs a 10-iteration palette
 * pump (fill $0180.. with $30, C569 + frame commits C135/B2CC). It then falls into
 * an INTENTIONAL infinite spin (L_B1E7: JMP L_B1E7) — the routine never returns;
 * control resumes only through the NMI / next game phase.
 *
 * INSPECTION-PORT (no diff-test spec): multiple read-controllers / frame-timer wait
 * loops and a terminal infinite spin; not isolation-testable in flat memory (the
 * $D4/$36 frame edges are advanced by the NMI). Integration-verified.
 * Far-calls handled: none via $CCE4 here — C461/C38B/etc. are plain JSRs in-bank.
 * Primitives: $CC8F=queue_ppu_job_and_wait, $FC08=song_init. */
#include "ram.h"
#include "regs.h"

void sub_B29B(Regs *r); void sub_C461(Regs *r); void sub_C38B(Regs *r);
void sub_B2EE(Regs *r); void sub_B2CC(Regs *r); void sub_B25D(Regs *r);
void sub_B1EA(Regs *r); void sub_B215(Regs *r); void sub_C569(Regs *r);
void sub_C135(Regs *r);
void queue_ppu_job_and_wait(Regs *r);   /* $CC8F (A = job id) */
void song_init(Regs *r);                /* $FC08 */

void sub_B13D(Regs *r)
{
    RAM8(0x92) = (u8)(RAM8(0x92) + 1);          /* INC $92 */
    sub_B29B(r);                                /* JSR L_B29B */
    sub_C461(r);                                /* JSR $C461 */
    sub_C38B(r);                                /* JSR $C38B */
    sub_B2EE(r);                                /* JSR L_B2EE */
    RAM8(0x2A) = 0x20;                          /* mmc3_r0_shadow */
    RAM8(0x2B) = 0x22;                          /* mmc3_r1_shadow */
    RAM8(0x24) = (u8)(RAM8(0x24) | 0x18);       /* LDA $24 / ORA #$18 / STA $24 */

    r->a = 0xFF;
    queue_ppu_job_and_wait(r);                  /* LDA #$FF / JSR $CC8F */

    RAM8(0x8E) = 0x0A;
    song_init(r);                               /* JSR $FC08 */

    RAM8(0x1C) = 0x00;                          /* a:$001C */
    RAM8(0x1D) = 0x00;                          /* a:$001D */
    RAM8(0x0A) = 0x00;                          /* a:$000A */
    RAM8(0x7B) = 0x00;                          /* scroll_x_fine */
    RAM8(0x7C) = 0x00;                          /* scroll_x_tile */
    sub_B2CC(r);                                /* JSR L_B2CC */

    RAM8(0x18) = 0x40;                          /* vram_src_lo */
    RAM8(0x19) = 0x01;                          /* vram_src_hi */
    RAM8(0x1A) = 0x20;                          /* vram_len */
    RAM8(0x0C) = 0x9C;                          /* src ptr lo -> $B79C */
    RAM8(0x0D) = 0xB7;                          /* src ptr hi */

    do {                                        /* L_B18B */
        sub_B25D(r);                            /* JSR L_B25D */
        sub_B1EA(r);                            /* JSR L_B1EA */
        if (r->c) break;                        /* BCS L_B19B */
        sub_B25D(r);                            /* JSR L_B25D */
        sub_B215(r);                            /* JSR L_B215 */
    } while (!r->c);                            /* BCC L_B18B */

    /* L_B19B */
    RAM8(0x8F) = 0x20;
    while (RAM8(0xD4) == 0) { }                 /* L_B19F: wait for $D4 != 0 */
    while (RAM8(0xD4) != 0) { }                 /* L_B1A3: wait for $D4 == 0 */
    RAM8(0x36) = 0x3C;
    while (RAM8(0x36) != 0) { }                 /* L_B1AB: 60-frame timer wait */

    RAM8(0x94) = 0x00;                          /* clear reveal flags */
    RAM8(0xA4) = 0x00;
    RAM8(0xB4) = 0x00;
    RAM8(0xC4) = 0x00;
    RAM8(0x8F) = 0x18;

    {                                           /* LDX #$0A: 10-iteration pump */
        u8 cnt = 0x0A;
        do {                                    /* L_B1BF */
            int x;
            for (x = 0x1F; x >= 0; x--)         /* L_B1C5: fill $0180.. with $30 */
                RAM8((u16)(0x0180 + x)) = 0x30;
            sub_C569(r);                        /* JSR $C569 */
            RAM8(0x36) = 0x01;
            sub_C135(r);                        /* JSR $C135 */
            sub_B2CC(r);                        /* JSR L_B2CC */
            sub_C569(r);                        /* JSR $C569 */
            RAM8(0x36) = 0x02;
            sub_C135(r);                        /* JSR $C135 */
            cnt = (u8)(cnt - 1);                /* DEX */
        } while (cnt != 0);                     /* BNE L_B1BF */
    }

    /* L_B1E7: JMP L_B1E7 — intentional terminal spin; routine never returns.
     * Control continues only via the NMI / next phase, so we just return here. */
}
