/* $A2EB (bank13) — world-warp / map-entry intro driver. Runs the "you arrive on
 * the map" presentation: refresh the palette buffer twice via $C540 (far-call,
 * X=pass count), build the scene for the chosen map screen ($C8F2 scene_assemble
 * with map_screen_y=$13 / map_screen_x=$02), set up scroll and the status sprite
 * ($0200=$EF), commit frames, then run the L_A378 reveal: a frame-synced wipe
 * loop that walks the counter $1E down (wrapping $22 -> .. -> $C2), deriving the
 * animation phase $1D = ($1E>>3)&1 and pushing a full PPU job ($CC8F with A=$FF)
 * every frame. On reaching $C2 (L_A395) it does a final palette pass ($C540 X=2),
 * a $C1C7 far-call, zeroes the warp/scroll state, folds player_x_tile into
 * player_x_fine, then L_AD7A / L_A7D2 / L_A7F0 and RTS.
 *
 * Far-calls use the $CCE4 farcall_return_home epilogue (restore bank shadows
 * $30/$31 from saved $32/$33, run the $hi:$lo target, then the $CD08 seed re-maps
 * banks 12/13 ($30=$0C,$31=$0D) with select_shadow $25=$07 — modelled per
 * src/ported/sub_B6A6.c). Resolved far-call targets: $C540 (x4, X passes the
 * loop count), $C8F2 (scene_assemble), $C5CB, $C76C, $C1C7. Direct (fixed-bank)
 * JSRs: $C1D8, $D08A, $C2B1, $C135, $C38B, $CC8F (queue_ppu_job_and_wait),
 * and the bank-local L_A5B5 / L_AD7A / L_A7D2 / L_A7F0.
 *
 * INSPECTION-PORT (no diff-test spec): the L_A378 loop is a frame-synced wipe —
 * $CC8F (queue_ppu_job_and_wait) blocks on the NMI and the counter only settles
 * over real frames; flat memory never advances it. Combined with the far-call-
 * driven presentation chain this is not isolation-testable. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_C1D8(Regs *r); void sub_C540(Regs *r); void sub_D08A(Regs *r);
void sub_C2B1(Regs *r); void sub_A5B5(Regs *r); void sub_C135(Regs *r);
void scene_assemble(Regs *r);   /* $C8F2 */
void sub_C38B(Regs *r); void sub_C5CB(Regs *r); void sub_C76C(Regs *r);
void sub_C1C7(Regs *r); void sub_AD7A(Regs *r); void sub_A7D2(Regs *r);
void sub_A7F0(Regs *r);
void queue_ppu_job_and_wait(Regs *r);   /* $CC8F (A = job id) */

/* $CCE4 farcall_return_home: restore banks from $32/$33, run target, then the
 * $CD08 seed re-maps banks 12/13 with select=$07 (see src/ported/sub_B6A6.c). */
static void farcall_cce4(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32); RAM8(0x31) = RAM8(0x33);
    target(r);
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07;
}

void sub_A2EB(Regs *r)
{
    RAM8(0x8F) = 0x18;
    RAM8(0x85) = 0x00;
    sub_C1D8(r);                                    /* JSR $C1D8 */

    r->x = 0x02;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);          /* far-call $C540 (X=2 passes) */
    sub_D08A(r);
    sub_C2B1(r);
    r->x = 0x03;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);          /* far-call $C540 (X=3 passes) */
    sub_A5B5(r);

    RAM8(0x8F) = 0x20;
    RAM8(0x36) = 0x3C;
    sub_C135(r);                                    /* JSR $C135 */

    RAM8(0x48) = 0x13;                              /* map_screen_y */
    RAM8(0x47) = 0x02;                              /* map_screen_x */
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);    /* far-call $C8F2 */
    sub_C38B(r);                                    /* JSR $C38B */

    RAM8(0x0200) = 0xEF;
    RAM8(0x1E) = 0x22;
    RAM8(0x7B) = 0x00;                              /* scroll_x_fine */
    RAM8(0x43) = 0x00;                              /* player_x_fine */
    RAM8(0x7C) = 0x10;                              /* scroll_x_tile */
    farcall_cce4(r, 0xCB, 0xC5, sub_C5CB);          /* far-call $C5CB */
    r->x = 0x04;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);          /* far-call $C540 (X=4 passes) */
    RAM8(0x7C) = 0x00;                              /* scroll_x_tile */
    farcall_cce4(r, 0x6C, 0xC7, sub_C76C);          /* far-call $C76C */
    RAM8(0x2D) = 0x3D;                              /* mmc3_r3_shadow */

    /* L_A378 — frame-synced reveal: walk $1E down until it hits $C2 */
    for (;;) {
        u8 x = RAM8(0x1E);                          /* LDX $1E / BNE L_A37E */
        if (x == 0) x = 0xF0;                       /* LDX #$F0 */
        if (x == 0xC2) break;                       /* CPX #$C2 / BEQ L_A395 */
        x = (u8)(x - 1);                            /* DEX */
        RAM8(0x1E) = x;                             /* STX $1E */
        RAM8(0x1D) = (u8)((x & 0x08) >> 3);         /* TXA / AND #$08 / LSR x3 */
        r->a = 0xFF;                                /* LDA #$FF */
        queue_ppu_job_and_wait(r);                  /* JSR $CC8F (blocks on NMI) */
    }                                               /* JMP L_A378 */

    /* L_A395 */
    r->x = 0x02;
    farcall_cce4(r, 0x40, 0xC5, sub_C540);          /* far-call $C540 (X=2 passes) */
    farcall_cce4(r, 0xC7, 0xC1, sub_C1C7);          /* far-call $C1C7 */

    RAM8(0x040C) = 0x00;
    RAM8(0x040D) = 0x00;
    RAM8(0x0406) = 0x00;
    RAM8(0xE9) = 0x00;
    RAM8(0x7B) = 0x00;                              /* scroll_x_fine */
    RAM8(0x7C) = 0x00;                              /* scroll_x_tile */
    RAM8(0x0405) = 0x64;
    RAM8(0x3E) = 0x08;
    RAM8(0x43) = (u8)(((u8)(RAM8(0x44) << 4)) | RAM8(0x43));  /* player_x_tile<<4 | player_x_fine */
    sub_AD7A(r);                                    /* JSR L_AD7A */
    RAM8(0x0210) = 0xEF;
    RAM8(0x0214) = 0xEF;
    sub_A7D2(r);                                    /* JSR L_A7D2 */
    sub_A7F0(r);                                    /* JSR L_A7F0 */
}
