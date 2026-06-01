/* $C06D main_loop_dispatch — the top-level per-frame game loop. main_init falls
 * through into it and it never returns (JMP main_loop_dispatch tail = loop).
 *
 * Each iteration:
 *   - If the player is dead (health==0): clear $85, run L_C1D8, then far-call
 *     $B307 (the death/respawn arbiter). On return X selects: X==0 -> keep looping
 *     (gameover screen still up); X==1 -> JMP L_C04F (respawn at the default start
 *     position); X>=2 -> JMP L_C026 (full soft re-init). L_C04F / L_C026 are
 *     re-entry points inside main_init, modelled here as a boot re-entry
 *     (main_init) — see note below.
 *   - If alive: $36=1 (request one frame), snapshot scroll_x_tile into $7E,
 *     read_controllers, game_update. Then, unless the warp flag $EC is set, run the
 *     per-frame chain L_F628 / L_E87C / L_F782 / L_C15D (carry preserved across
 *     L_C1D8 / L_C2B1 via PHP/PLP); if carry clear and scroll_x_tile changed, bump
 *     the column-redraw flag $3D; finally L_C135 (frame commit) and loop.
 *   - Warp path (L_C0C9, $EC set): far-call $A2EB (warp intro), then loop far-calls
 *     $ABBC / $A5E6 / $A75D / $A3E3 each frame (via read_controllers at L_C0D4)
 *     until health!=0, then fold player_x_fine into player_x_tile/player_x_fine,
 *     set the status sprite $0200=$EF, clear $85, L_C1D8, and far-call $B307 again
 *     to choose respawn (X==1 -> L_C04F) or re-init (X>=2 -> L_C026).
 *
 * Far-calls (via farcall_bank_0C0D $CC9C): save bank shadows $30/$31 -> $32/$33,
 * map banks 0C/0D ($25=$07), run the $hi:$lo target, restore shadows, $25=$06 —
 * modelled per src/ported/sub_F349.c. Targets: $B307 (sub_B307), and the warp
 * chain $A2EB, $ABBC, $A5E6, $A75D, $A3E3. All other JSRs ($C1D8, $F628, $E87C,
 * $F782, $C15D, $C2B1, $C135, read_controllers, game_update) are direct fixed-bank
 * calls.
 *
 * INSPECTION-PORT (no diff-test spec): an infinite frame loop whose progress is
 * gated by the NMI ($36 counter) and live controller input; flat memory never
 * advances it. Integration-verified.
 *
 * NOTE: L_C04F (respawn) and L_C026 (re-init) are labels inside main_init reached
 * here only by non-local JMP. The flat Regs ABI has no goto-into-another-function,
 * so both are modelled as a re-entry into main_init(r) (which contains them) and
 * documented; integration jumps directly. */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r); void sub_C135(Regs *r); void sub_C15D(Regs *r);
void sub_C2B1(Regs *r); void sub_F628(Regs *r); void sub_E87C(Regs *r);
void sub_F782(Regs *r);
void sub_B307(Regs *r);                 /* far-call $B307 (death/respawn arbiter) */
void sub_A2EB(Regs *r); void sub_ABBC(Regs *r); void sub_A5E6(Regs *r);
void sub_A75D(Regs *r); void sub_A3E3(Regs *r);
void read_controllers(Regs *r);        /* $CC43 */
void game_update(Regs *r);             /* $D42B */
void main_init(Regs *r);               /* contains L_C026 / L_C04F re-entry points */

/* farcall_bank_0C0D ($CC9C) side-effects (plain dispatcher), per sub_F349.c. */
static void farcall_0C0D(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    u8 old6 = RAM8(0x30), old7 = RAM8(0x31);
    RAM8(0x32) = old6; RAM8(0x33) = old7;
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
    target(r);                                  /* JMP ($000E) */
    RAM8(0x31) = old7; RAM8(0x30) = old6; RAM8(0x25) = 0x06; NES_PRG_SYNC();
}

void main_loop_dispatch(Regs *r)
{
    for (;;) {                                  /* JMP main_loop_dispatch */
        if (RAM8(0x58) == 0) {                  /* LDA health / BNE L_C093 — dead */
            RAM8(0x85) = 0x00;
            sub_C1D8(r);
            farcall_0C0D(r, 0x07, 0xB3, sub_B307);      /* far-call $B307 */
            if (r->x == 0)                      /* CPX #$00 / BNE L_C08A */
                continue;                       /* JMP main_loop_dispatch */
            r->x = (u8)(r->x - 1);              /* L_C08A: DEX */
            if (r->x == 0) {                    /* BNE L_C090 */
                main_init(r);                   /* JMP L_C04F (respawn, in main_init) */
                return;
            }
            main_init(r);                       /* JMP L_C026 (re-init, in main_init) */
            return;
        }

        /* L_C093 — alive */
        RAM8(0x36) = 0x01;
        RAM8(0x7E) = RAM8(0x7C);                /* $7E <- scroll_x_tile */
        read_controllers(r);
        game_update(r);

        if (RAM8(0xEC) != 0) {                  /* LDA $EC / BNE L_C0C9 — warp */
            /* L_C0C9 */
            farcall_0C0D(r, 0xEB, 0xA2, sub_A2EB);      /* far-call $A2EB */
            do {                                /* L_C0D4 */
                read_controllers(r);
                farcall_0C0D(r, 0xBC, 0xAB, sub_ABBC);  /* far-call $ABBC */
                farcall_0C0D(r, 0xE6, 0xA5, sub_A5E6);  /* far-call $A5E6 */
                farcall_0C0D(r, 0x5D, 0xA7, sub_A75D);  /* far-call $A75D */
                farcall_0C0D(r, 0xE3, 0xA3, sub_A3E3);  /* far-call $A3E3 */
            } while (RAM8(0x58) == 0);          /* LDA health / BNE L_C0D4 */

            /* reposition player from player_x_fine */
            RAM8(0x44) = (u8)(RAM8(0x43) >> 4); /* player_x_tile = fine>>4 */
            RAM8(0x43) = (u8)(RAM8(0x43) & 0x0F);   /* player_x_fine &= $0F */
            RAM8(0x0200) = 0xEF;                /* status sprite */
            RAM8(0x85) = 0x00;
            sub_C1D8(r);
            farcall_0C0D(r, 0x07, 0xB3, sub_B307);      /* far-call $B307 */
            r->x = (u8)(r->x - 1);              /* DEX */
            if (r->x == 0) {                    /* BNE L_C132 */
                main_init(r);                   /* JMP L_C04F (respawn) */
                return;
            }
            main_init(r);                       /* JMP L_C026 (re-init) */
            return;
        }

        /* L_C093 continued — normal per-frame chain */
        sub_F628(r);
        sub_E87C(r);
        sub_F782(r);
        sub_C15D(r);                            /* sets carry */
        {
            u8 saved_c = r->c;                  /* PHP */
            sub_C1D8(r);
            sub_C2B1(r);
            r->c = saved_c;                     /* PLP */
        }
        if (!r->c) {                            /* BCS L_C0C3 */
            if (RAM8(0x7E) != RAM8(0x7C))       /* CMP scroll_x_tile / BEQ L_C0C3 */
                RAM8(0x3D)++;                   /* INC $3D */
        }
        /* L_C0C3 */
        sub_C135(r);                            /* JSR L_C135 (frame commit) */
        /* JMP main_loop_dispatch */
    }
}
