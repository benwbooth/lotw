/* $B307 — player death / game-over sequence (far-called from the main loop).
 * Saves the current song id ($8E) on the stack, advances the death sound state
 * ($8D), plays the death animation: D07C, a metasprite flash via B4C5 ($35/$00),
 * a 60-frame hold, the death SFX ($D02E #$08), then DEC $8D and a 5-iteration
 * flicker loop (L_B328) of four B4C5 metasprite draws. It commits a frame
 * (C1D8/C135 with $56=$31) and branches on the lives/continue flag $EC:
 *
 *   $EC == 0  -> respawn path:
 *       - if $37 (continues) still positive: INC $37, skip the item check
 *       - else (L_B363): if the equipped slot holds item $0C, clear it (=$FF) and
 *         run C234
 *       - L_B372: D16A, $56=$19, debounced press (CC09), restore the saved song
 *         ($8E via D02E), return with X=0 (respawn).
 *
 *   $EC != 0  -> game-over screen (L_B383): discard saved song, reset state
 *       (C461/C38B/D08A/C2B1), program CHR banks, blit three VRAM strips of the
 *       "game over / continue" menu text via $CC8F, place the cursor sprite at
 *       player_x/y, build it (C375/C1D8), and far-call $C4E0 (cursor init) through
 *       $CCE4. Then L_B41D: poll CC09 for the confirm button (bit4); while not
 *       pressed, blink the cursor (EOR player_y #$10) and loop. On press, if the
 *       cursor is on the lower option (player_y != $70) -> continue path: C461,
 *       a $78-frame wait far-call to $C135, return X=2. Otherwise (L_B450) ->
 *       new-game/quit path: D0C5, wipe the carried-item slots (=$FF), reset
 *       character/map state, rebuild (C461/C38B/C57A/CAB6/CACC/CAE2/CAF8),
 *       far-call $C8F2 (scene_assemble) via $CCE4, fill $0180.. with $0F, hide two
 *       sprites, far-call $C4B4 via $CCE4, and return X=1.
 *
 * INSPECTION-PORT (no diff-test spec): frame-timer / read-controllers wait loops,
 * far-calls with bank-shadow + PPU-queue side effects, and a balanced PHA/PLA song
 * save; not isolation-testable in flat memory. Integration-verified.
 * Far-calls handled via farcall_return_home ($CCE4): $C4E0, $C135, $C8F2
 * (scene_assemble), $C4B4. Primitives: $CC8F=queue_ppu_job_and_wait,
 * $CC09 debounced read, $D02E song set. The PHA/PLA is balanced (each exit path
 * pulls the one pushed $8E) — not a non-local return. */
#include "ram.h"
#include "regs.h"

void sub_D07C(Regs *r); void sub_B4C5(Regs *r); void sub_C135(Regs *r);
void sub_D02E(Regs *r); void sub_C1D8(Regs *r); void sub_C234(Regs *r);
void sub_D16A(Regs *r); void sub_CC09(Regs *r); void sub_C461(Regs *r);
void sub_C38B(Regs *r); void sub_D08A(Regs *r); void sub_C2B1(Regs *r);
void sub_C375(Regs *r); void sub_C4E0(Regs *r); void sub_C4B4(Regs *r);
void sub_D0C5(Regs *r); void sub_C57A(Regs *r); void sub_CAB6(Regs *r);
void sub_CACC(Regs *r); void sub_CAE2(Regs *r); void sub_CAF8(Regs *r);
void scene_assemble(Regs *r);           /* $C8F2 */
void queue_ppu_job_and_wait(Regs *r);   /* $CC8F (A = job id) */

/* $CCE4 farcall_return_home: restore banks from $32/$33, run target, then the
 * $CD08 seed re-maps banks 12/13 with select=$07 (see src/ported/sub_B6A6.c). */
static void farcall_cce4(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32); RAM8(0x31) = RAM8(0x33); RAM8(0x25) = 0x06; NES_PRG_SYNC();
    target(r);
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
}

/* Upload one VRAM strip: set dst/src/len, then queue (A = job id $05). */
static void vram_blit(Regs *r, u8 dlo, u8 dhi, u8 slo, u8 shi, u8 len)
{
    RAM8(0x16) = dlo; RAM8(0x17) = dhi;     /* vram_dst_lo/hi */
    RAM8(0x18) = slo; RAM8(0x19) = shi;     /* vram_src_lo/hi */
    RAM8(0x1A) = len;                       /* vram_len */
    r->a = 0x05;
    queue_ppu_job_and_wait(r);              /* JSR $CC8F */
}

void sub_B307(Regs *r)
{
    u8 saved_song = RAM8(0x8E);             /* LDA $8E / PHA */

    RAM8(0x8D) = (u8)(RAM8(0x8D) + 1);      /* INC $8D */
    sub_D07C(r);                            /* JSR $D07C */
    r->x = 0x35; r->y = 0x00;
    sub_B4C5(r);                            /* LDX #$35 / LDY #$00 / JSR L_B4C5 */
    RAM8(0x36) = 0x3C;
    sub_C135(r);                            /* JSR $C135 */
    r->a = 0x08;
    sub_D02E(r);                            /* LDA #$08 / JSR $D02E (death SFX) */
    RAM8(0x8D) = (u8)(RAM8(0x8D) - 1);      /* DEC $8D */

    RAM8(0x0A) = 0x05;                      /* LDA #$05 / STA $0A */
    do {                                    /* L_B328 */
        r->x = 0x0D; r->y = 0x00; sub_B4C5(r);
        r->x = 0x01; r->y = 0x00; sub_B4C5(r);
        r->x = 0x09; r->y = 0x00; sub_B4C5(r);
        r->x = 0x01; r->y = 0x40; sub_B4C5(r);
        RAM8(0x0A) = (u8)(RAM8(0x0A) - 1);
    } while (RAM8(0x0A) != 0);              /* BNE L_B328 */

    RAM8(0x36) = 0x01;
    RAM8(0x56) = 0x31;
    sub_C1D8(r);                            /* JSR $C1D8 */
    sub_C135(r);                            /* JSR $C135 */

    if (RAM8(0xEC) == 0) {                  /* LDA $EC / BNE L_B383 */
        /* respawn path */
        if (RAM8(0x37) & 0x80) {            /* LDA $37 / BPL L_B363 */
            /* L_B363: equipped-item special-case */
            u8 x = RAM8(0x55);              /* LDX equipped_item */
            if (RAM8((u16)(0x51 + x)) == 0x0C) {   /* LDA carried_item0,X / CMP #$0C */
                RAM8((u16)(0x51 + x)) = 0xFF;      /* LDA #$FF / STA carried_item0,X */
                sub_C234(r);                       /* JSR $C234 */
            } else {
                goto l_b383;               /* BNE L_B383 */
            }
        } else {
            RAM8(0x37) = (u8)(RAM8(0x37) + 1);     /* INC $37 / JMP L_B372 */
        }

        /* L_B372 */
        sub_D16A(r);                        /* JSR $D16A */
        RAM8(0x56) = 0x19;
        sub_CC09(r);                        /* JSR $CC09 (debounced press) */
        r->a = saved_song;                  /* PLA */
        sub_D02E(r);                        /* JSR $D02E (restore song) */
        r->x = 0x00;                        /* LDX #$00 */
        return;                             /* RTS — respawn */
    }

l_b383:                                     /* L_B383 — game-over screen */
    /* PLA (discard saved song) */
    sub_C461(r);                            /* JSR $C461 */
    RAM8(0xEC) = 0x00;
    RAM8(0x3E) = 0x00;
    RAM8(0x3F) = 0x80;
    sub_C38B(r);                            /* JSR $C38B */
    sub_D08A(r);                            /* JSR $D08A */
    sub_C2B1(r);                            /* JSR $C2B1 */
    RAM8(0x2B) = 0x16;                      /* mmc3_r1_shadow */
    RAM8(0x2C) = 0x36;                      /* mmc3_r2_shadow */
    RAM8(0x1C) = 0x00;                      /* a:$001C */
    RAM8(0x1D) = 0x00;                      /* a:$001D */
    RAM8(0x1E) = 0x00;                      /* a:$001E */
    RAM8(0x7B) = 0x00;                      /* scroll_x_fine */
    RAM8(0x7C) = 0x00;                      /* scroll_x_tile */

    vram_blit(r, 0x6B, 0x21, 0xAF, 0xB4, 0x09);    /* menu text strip 1 */
    vram_blit(r, 0x4C, 0x22, 0xB8, 0xB4, 0x05);    /* strip 2 */
    vram_blit(r, 0x8C, 0x22, 0xBD, 0xB4, 0x08);    /* strip 3 */

    RAM8(0x44) = 0x05;                      /* player_x_tile */
    RAM8(0x43) = 0x00;                      /* player_x_fine */
    RAM8(0x45) = 0x70;                      /* player_y */
    RAM8(0x56) = 0x39;
    sub_C375(r);                            /* JSR $C375 */
    sub_C1D8(r);                            /* JSR $C1D8 */
    farcall_cce4(r, 0xE0, 0xC4, sub_C4E0);  /* far-call $C4E0 (cursor init) */

    for (;;) {                              /* L_B41D */
        sub_CC09(r);                        /* JSR $CC09 */
        if (r->a & 0x10) break;             /* AND #$10 / BNE L_B431 */
        RAM8(0x45) = (u8)(RAM8(0x45) ^ 0x10);  /* player_y EOR #$10 (blink) */
        RAM8(0x8F) = 0x0C;
        /* JMP L_B41D */
    }

    /* L_B431 */
    RAM8(0x8F) = 0x18;
    if (RAM8(0x45) != 0x70) {               /* player_y CMP #$70 / BEQ L_B450 */
        /* continue path */
        sub_C461(r);                        /* JSR $C461 */
        RAM8(0x36) = 0x78;
        farcall_cce4(r, 0x35, 0xC1, sub_C135);  /* far-call $C135 (frame commit) */
        r->x = 0x02;                        /* LDX #$02 */
        return;                             /* RTS — continue */
    }

    /* L_B450 — new-game / quit path */
    sub_D0C5(r);                            /* JSR $D0C5 */
    RAM8(0x51) = 0xFF;                      /* carried_item0 */
    RAM8(0x52) = 0xFF;                      /* carried_item1 */
    RAM8(0x53) = 0xFF;                      /* carried_item2 */
    RAM8(0x55) = 0x03;                      /* equipped_item */
    RAM8(0x40) = 0x06;                      /* cur_character */
    RAM8(0x47) = 0x03;                      /* map_screen_x */
    RAM8(0x48) = 0x10;                      /* map_screen_y */
    sub_C461(r);                            /* JSR $C461 */
    RAM8(0x8E) = 0x02;
    sub_C38B(r);                            /* JSR $C38B */
    sub_C57A(r);                            /* JSR $C57A */
    sub_CAB6(r);                            /* JSR $CAB6 */
    sub_CACC(r);                            /* JSR $CACC */
    sub_CAE2(r);                            /* JSR $CAE2 */
    sub_CAF8(r);                            /* JSR $CAF8 */
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);   /* far-call $C8F2 */

    {                                       /* fill $0180.. with $0F */
        int x;
        r->a = 0x0F;
        for (x = 0x1F; x >= 0; x--)         /* L_B493 */
            RAM8((u16)(0x0180 + x)) = 0x0F;
    }
    RAM8(0x0210) = 0xEF;                    /* hide sprites */
    RAM8(0x0214) = 0xEF;
    farcall_cce4(r, 0xB4, 0xC4, sub_C4B4);  /* far-call $C4B4 */
    r->x = 0x01;                            /* LDX #$01 */
    /* RTS — new game */
}
