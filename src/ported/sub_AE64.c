/* $AE64 — title-screen / game-init / character-select driver (far-called from
 * main_init at boot, bank 13). It programs the PPU (ppuctrl_shadow=$A0 + PPUCTRL,
 * PPUMASK off then on=$1E), maps MMC3 banks, clears the BG palette buffer
 * ($0180-$019F = $0F), loads the title graphics/palette via a chain of far-calls,
 * runs the title-screen input loop (cursor animation on the player-1 pad until
 * START or the SELECT-style $21==$83 chord), then on START falls through to the
 * character-select / world-entry sequence (L_AF1D): pick a random map screen and
 * spawn position via rng_update, load the chosen character's stats from the
 * $FFA7 table, seed health/magic, build the scene, and run the select/intro
 * frame loop (L_B021). On finishing it tail-jumps back to $AE64 to restart the
 * title; the SELECT/START exit paths route to L_B0B1 (clean teardown -> RTS) or
 * tail-jump to L_B13D (resume-saved-game, a separate routine).
 *
 * Far-calls use the $CCE4 farcall_return_home epilogue: restore bank shadows
 * $30/$31 from the saved $32/$33, run the $hi:$lo target, then the $CD08 seed
 * re-maps banks 12/13 ($30=$0C,$31=$0D) with select_shadow $25=$07
 * (modelled per src/ported/sub_B6A6.c). The far-call targets resolved here are
 * $C569,$C135,$C8F2(scene_assemble),$C38B,$C5CB,$C492,$D42B(game_update),
 * $F628,$E87C,$F782,$C15D.
 *
 * INSPECTION-PORT (no diff-test spec): PPU/MMC3 register setup, vblank waits on
 * the NMI-decremented $36 counter, read_controllers ($CC43) input loops, and a
 * far-call-driven title/select sequence — not isolation-testable in flat memory
 * (read_controllers yields no input and $36 never decrements without the NMI).
 * Integration-verified. */
#include "ram.h"
#include "regs.h"
#ifdef LOTW_SHIM
#include "ppu.h"         /* nes_vblank_wait — drive the title/select frames */
#endif

/* JSR / far-call targets */
void sub_B631(Regs *r); void sub_D08A(Regs *r); void sub_C375(Regs *r);
void sub_B102(Regs *r); void sub_B648(Regs *r); void sub_B6A6(Regs *r);
void sub_C461(Regs *r); void sub_B10E(Regs *r); void sub_CA54(Regs *r);
void sub_C57A(Regs *r); void sub_CAB6(Regs *r); void sub_CACC(Regs *r);
void sub_CAF8(Regs *r); void sub_CAE2(Regs *r); void sub_C1C7(Regs *r);
void sub_D07C(Regs *r); void sub_C1D8(Regs *r); void sub_C234(Regs *r);
void sub_C2B1(Regs *r); void sub_B11A(Regs *r); void sub_B0E4(Regs *r);
void sub_C569(Regs *r); void sub_C135(Regs *r); void sub_C38B(Regs *r);
void sub_C5CB(Regs *r); void sub_C492(Regs *r); void sub_F628(Regs *r);
void sub_E87C(Regs *r); void sub_F782(Regs *r); void sub_C15D(Regs *r);
void scene_assemble(Regs *r);   /* $C8F2 */
void game_update(Regs *r);      /* $D42B */
void read_controllers(Regs *r); /* $CC43 (result -> $20, A) */
void rng_update(Regs *r);       /* $CC64 (result -> A) */
void song_init(Regs *r);        /* $FC08 */

/* These two are reached only by non-local JMP out of $AE64. */
void sub_B0B1(Regs *r);         /* L_B0B1: teardown -> RTS */
void sub_B13D(Regs *r);         /* L_B13D: resume-saved-game */

/* $CCE4 farcall_return_home: restore banks from $32/$33, run target, then the
 * $CD08 seed re-maps banks 12/13 with select=$07 (see src/ported/sub_B6A6.c). */
static void farcall_cce4(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32); RAM8(0x31) = RAM8(0x33); RAM8(0x25) = 0x06; NES_PRG_SYNC();
    target(r);
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
}

/* JSR $CC43 leaves the combined pad in A (== $20). */
static u8 read_pad(Regs *r) { read_controllers(r); r->a = RAM8(0x20); return r->a; }

void sub_AE64(Regs *r)
{
restart:                                            /* L_AE64 */
    sub_B631(r);
    RAM8(0x2C) = 0x37;                              /* mmc3_r2_shadow */
    RAM8(0x29) = 0x00;
    RAM8(0x23) = 0xA0;                              /* ppuctrl_shadow */
    REG_W(0x2000, 0xA0);                            /* PPUCTRL */
    RAM8(0x24) = 0x00;
    REG_W(0x2001, 0x00);                            /* PPUMASK */
    RAM8(0x1C) = 0x00;
    RAM8(0x1D) = 0x00;
    RAM8(0x1E) = 0xE8;
    {                                               /* fill $0180..$019F with $0F */
        int x;
        for (x = 0x1F; x >= 0; x--) RAM8((u16)(0x0180 + x)) = 0x0F;
    }
    farcall_cce4(r, 0x69, 0xC5, sub_C569);          /* far-call $C569 */
    sub_D08A(r);
    sub_C375(r);
    sub_B102(r);
    RAM8(0x2C) = 0x15;                              /* mmc3_r2_shadow */
    RAM8(0x8E) = 0x09;
    song_init(r);                                   /* JSR $FC08 */
    sub_B648(r);
    RAM8(0x24) = 0x1E;
    REG_W(0x2001, 0x1E);                            /* PPUMASK */
    RAM8(0x36) = 0x78;
#ifdef LOTW_SHIM
    while (RAM8(0x36) != 0) nes_vblank_wait(r);     /* L_AEBE: ~120-frame title hold */
#else
    while (RAM8(0x36) != 0) { }                     /* L_AEBE: vblank wait */
#endif
    sub_B6A6(r);
    RAM8(0x8C) = 0x14;

    /* L_AEC9 — title-screen cursor / input loop */
    for (;;) {
        u8 pad;
        RAM8(0x36) = 0x01;
        pad = read_pad(r);                          /* JSR $CC43 */
        if (pad == 0xFF) {                          /* CMP #$FF / BNE */
            RAM8(0x8F) = 0x1A;
            RAM8(0x37) = 0x1A;
        }
        (void)pad;
        if (RAM8(0x20) & 0x10) {                    /* AND #$10 / BNE -> JMP L_B0B1 */
            sub_B0B1(r);
            return;
        }
        if (RAM8(0x21) == 0x83) {                   /* CMP #$83 / BEQ -> JMP L_B13D */
            sub_B13D(r);
            return;
        }
        if ((RAM8(0x84) & 0x07) == 0) {             /* L_AEDA path: animate cursor */
            u8 lo = RAM8(0x0182) & 0x0F;
            u8 hi = RAM8(0x0182) & 0xF0;
            RAM8(0x08) = lo;
            if ((u8)(hi - 0x10) > hi) hi = 0x30;    /* SBC #$10 / BCS .. else $30 */
            else hi = (u8)(hi - 0x10);
            RAM8(0x0193) = hi;
            RAM8(0x0182) = (u8)(hi | RAM8(0x08));
        }
        farcall_cce4(r, 0x35, 0xC1, sub_C135);      /* far-call $C135 (commit frame) */
        if (RAM8(0x8C) != 0) continue;              /* BNE L_AEC9 */
        break;                                      /* JMP L_AF1D */
    }

    /* L_AF1D — character-select / world entry */
    sub_C461(r);
    sub_C375(r);
    sub_D08A(r);
    sub_B10E(r);
    r->a = 0x04; rng_update(r); RAM8(0x47) = r->a;  /* map_screen_x */
    r->a = 0x10; rng_update(r); RAM8(0x48) = r->a;  /* map_screen_y */
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);    /* far-call $C8F2 */

    /* L_AF42 — pick a valid spawn tile */
    for (;;) {
        u8 t;
        r->a = 0x40; rng_update(r);
        RAM8(0x44) = r->a;                          /* player_x_tile */
        RAM8(0x0C) = r->a;
        RAM8(0x43) = 0x00;                          /* player_x_fine */
        r->a = 0x0B; rng_update(r);
        r->a = (u8)(r->a << 4);                     /* ASL x4 */
        RAM8(0x45) = r->a;                          /* player_y */
        RAM8(0x0D) = r->a;
        sub_CA54(r);
        {
            u16 p = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
            t = RAM8(p) & 0x3F;                      /* ($0C),Y=0 */
            if (t >= 0x30) continue;
            if (t == 0x02) continue;
            if (t == RAM8(0x70)) continue;
            t = RAM8((u16)(p + 1)) & 0x3F;           /* ($0C),Y=1 */
            if (t < 0x30) continue;                  /* BCC L_AF42 */
            if (t == 0x30) continue;                 /* BEQ L_AF42 */
        }
        break;
    }
    {                                               /* compute scroll_x_tile */
        u8 x = RAM8(0x44);                          /* player_x_tile */
        if ((u8)(x - 0x08) > x) x = 0x00;           /* SBC #$08 / BCS else 0 */
        else x = (u8)(x - 0x08);
        if (x >= 0x30) x = 0x30;
        RAM8(0x7C) = x;                             /* scroll_x_tile */
        RAM8(0x7B) = 0x00;                          /* scroll_x_fine */
    }

    /* L_AF91 — pick an available character (bit test against roster $41) */
    {
        u8  chr;
        for (;;) {
            u8 mask, a, c; int y;
            r->a = 0x05; rng_update(r);
            chr = r->a;
            a = 0x00; c = 1;                        /* SEC / LDA #$00 */
            for (y = chr; y >= 0; y--) {            /* ROL A / DEY / BPL */
                u8 nc = (a >> 7) & 1;
                a = (u8)((a << 1) | c);
                c = nc;
            }
            mask = a;                               /* = 1 << chr */
            if ((mask & RAM8(0x41)) != 0) break;    /* AND $41 / BEQ retry */
        }
        RAM8(0x51) = RAM8((u16)(0xB0AC + chr));     /* carried_item0 <- table */
        RAM8(0x55) = 0x00;                          /* equipped_item */
        RAM8(0x40) = chr;                           /* cur_character */
        {                                           /* copy 4 stat bytes from $FFA7 */
            int i;
            u16 y = (u16)(0xFFA7 + ((chr << 2) + 0x03));
            for (i = 3; i >= 0; i--) {
                RAM8((u16)(0x5C + i)) = RAM8(y);    /* stat_jump,X */
                y--;
            }
        }
        RAM8(0x2C) = (u8)(RAM8(0x40) + 0x38);       /* mmc3_r2_shadow */
    }
    RAM8(0x2E) = 0x3E;                              /* mmc3_r4_shadow */
    RAM8(0x2F) = 0x20;                              /* mmc3_r5_shadow */
    RAM8(0x56) = 0x0D;
    RAM8(0x57) = 0x00;
    RAM8(0x42) = 0x01;
    RAM8(0x58) = 0x64;                              /* health */
    RAM8(0x59) = 0x64;                              /* magic */
    farcall_cce4(r, 0x8B, 0xC3, sub_C38B);          /* far-call $C38B */
    sub_C57A(r);
    farcall_cce4(r, 0xCB, 0xC5, sub_C5CB);          /* far-call $C5CB */
    sub_CAB6(r);
    sub_CACC(r);
    sub_CAF8(r);
    sub_CAE2(r);
    sub_C1C7(r);
    sub_D07C(r);
    sub_C1D8(r);
    sub_C234(r);
    farcall_cce4(r, 0x92, 0xC4, sub_C492);          /* far-call $C492 */
    RAM8(0x8C) = 0x0A;

    /* L_B021 — intro/select frame loop */
    for (;;) {
        RAM8(0x36) = 0x01;
        RAM8(0x7E) = RAM8(0x7C);                    /* $7E <- scroll_x_tile */
        sub_B11A(r);
        read_controllers(r);                        /* JSR $CC43 */
        if (RAM8(0x20) & 0x10) {                    /* AND #$10 / BEQ else JMP L_B0B1 */
            sub_B0B1(r);
            return;
        }
        /* L_B036 */
        RAM8(0x20) = RAM8(0xFE);                    /* LDA $FE / STA $20 */
        {
            int do_b044 = 1;
            if ((RAM8(0x49) | RAM8(0x4B)) != 0) {   /* ORA / BEQ L_B044 */
                RAM8(0x42) = (u8)(RAM8(0x42) - 1);  /* DEC $42 */
                if (RAM8(0x42) != 0) do_b044 = 0;   /* BNE L_B04F */
            }
            if (do_b044) {                           /* L_B044 */
                RAM8(0x42) = 0x80;
                sub_B0E4(r);
                RAM8(0xFE) = RAM8(0x20);
            }
        }
        /* L_B04F — frame-commit far-call chain */
        farcall_cce4(r, 0x2B, 0xD4, game_update);   /* far-call $D42B */
        farcall_cce4(r, 0x28, 0xF6, sub_F628);      /* far-call $F628 */
        farcall_cce4(r, 0x7C, 0xE8, sub_E87C);      /* far-call $E87C */
        farcall_cce4(r, 0x82, 0xF7, sub_F782);      /* far-call $F782 */
        farcall_cce4(r, 0x5D, 0xC1, sub_C15D);      /* far-call $C15D */
        sub_C1D8(r);
        sub_C2B1(r);
        if (RAM8(0x7E) != RAM8(0x7C)) RAM8(0x3D)++; /* INC $3D */
        farcall_cce4(r, 0x35, 0xC1, sub_C135);      /* far-call $C135 */
        if (RAM8(0x8C) != 0) continue;              /* BNE L_B021 */
        break;                                       /* L_B0A6 */
    }
    /* L_B0A6 */
    sub_C461(r);
    goto restart;                                   /* JMP L_AE64 */
}
