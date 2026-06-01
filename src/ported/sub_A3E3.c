/* $A3E3 — warp / between-levels teleport cutscene driver (bank 13, far-called
 * from main_init / main_loop, the "warp web" entry per disasm/entries.txt). It is
 * the largest routine in the game.
 *
 * Two top-level phases:
 *  1. The per-frame "warp web animation" dispatcher (L_A3E3..L_A56D). Each call
 *     advances a small state machine selected by $F3 (web phase 0..4) when $FA==0,
 *     moving the web center ($1C/$1E) and choosing the web sprite tile ($7A). A
 *     companion OAM bookkeeping pass runs when bit6 of nmi_scratch is set
 *     (L_A3F5), decaying particle entries in the $0401,X table and damaging the
 *     boss (boss_life). The dispatcher then redraws one column of the web
 *     nametable strip (JSR L_A574 = sub_A574, which itself far-shadow-calls $C833)
 *     and runs the engine tick $E99A, then RTS to the caller. When boss_life has
 *     reached 0 it instead tail-jumps into phase 2 (JMP L_A7FF) and never returns
 *     through the dispatcher path.
 *  2. The completion cutscene (L_A7FF..JMP L_B13D). Resets particle slots and
 *     $FA/$85/$88, walks the player sprite up the screen frame by frame (waiting
 *     on the NMI PPU-job queue $CC8F = queue_ppu_job_and_wait), animates the web
 *     opening/closing (web tiles $B6/$B7 via repeated sub_A574 strip draws),
 *     scrolls a fixed number of columns ($10 frames), then either RTS (if
 *     health==0 — death) or runs the level-entry build: sub_B29B, the C46x/C3xx
 *     scene/PPU helpers, MMC3 bank shadows, OAM enemy slot setup, a chain of
 *     far-calls (scene_assemble $C8F2, $C5CB, $C492, game_update $D42B, $C15D),
 *     character animation loops (sub_AAEE), and finally tail-jumps to the
 *     resume/in-game routine (JMP L_B13D = sub_B13D).
 *
 * ABI: void sub_A3E3(Regs *r). 6502 regs/flags travel in r; memory is RAM8()/
 * REG_W() over the flat 64K space. Wait loops on the NMI-decremented $36 counter
 * and on the NMI-cleared queue_ppu_job_and_wait flag are kept as faithful
 * while-loops. BIT tests read bit7 (N) / bit6 (V) of the operand.
 *
 * Far-calls use the $CCE4 farcall_return_home epilogue (modelled per
 * src/ported/sub_B6A6.c / sub_AE64.c): store $0E/$0F, restore the bank shadows
 * $30/$31 from the saved $32/$33, run the $hi:$lo target, then the $CD08 seed
 * re-maps banks 12/13 ($30=$0C,$31=$0D) with select_shadow $25=$07. Far-call
 * targets resolved here: scene_assemble ($C8F2), $C5CB, $C492, game_update
 * ($D42B), $C15D.
 *
 * INSPECTION-PORT (no diff-test spec): a far-call-driven, NMI-synchronised
 * multi-second cutscene (PPU-job queue waits, frame counter $36 spins, OAM/
 * scroll register effects via callees) that cannot be exercised in flat host
 * memory — the NMI never runs, so $36 never decrements and the PPU-job flag
 * ($CC8F) never clears. Integration-verified. */
#include "ram.h"
#include "regs.h"
#ifdef LOTW_SHIM
#include "ppu.h"         /* nes_vblank_wait */
#endif

/* JSR targets in the fixed / current bank (regular calls). */
void sub_E98F(Regs *r);  void sub_E99A(Regs *r);  void sub_CB69(Regs *r);
void sub_CB7F(Regs *r);  void sub_A574(Regs *r);  void sub_A6E0(Regs *r);
void sub_A75D(Regs *r);  void sub_AD7A(Regs *r);  void sub_ACE0(Regs *r);
void sub_AE2F(Regs *r);  void sub_AAAE(Regs *r);  void sub_AAEE(Regs *r);
void sub_B29B(Regs *r);  void sub_C461(Regs *r);  void sub_C38B(Regs *r);
void sub_C375(Regs *r);  void sub_C57A(Regs *r);  void sub_CAB6(Regs *r);
void sub_CACC(Regs *r);  void sub_CAF8(Regs *r);  void sub_CAE2(Regs *r);
void sub_C1C7(Regs *r);  void sub_D07C(Regs *r);  void sub_C1D8(Regs *r);
void sub_C234(Regs *r);  void sub_C2B1(Regs *r);  void sub_D08A(Regs *r);
void queue_ppu_job_and_wait(Regs *r);   /* $CC8F (A = job code) */

/* Far-call targets (run through the $CCE4 return-home epilogue). */
void scene_assemble(Regs *r);   /* $C8F2 */
void game_update(Regs *r);      /* $D42B */
void sub_C5CB(Regs *r);         /* $C5CB */
void sub_C492(Regs *r);         /* $C492 */
void sub_C15D(Regs *r);         /* $C15D */

/* Non-local tail-jump target (separate routine, reached via JMP L_B13D). */
void sub_B13D(Regs *r);

/* Direct ($C135) frame-commit call, ported. */
void sub_C135(Regs *r);

/* $CCE4 farcall_return_home: restore banks from $32/$33, run target, then the
 * $CD08 seed re-maps banks 12/13 with select=$07 (see src/ported/sub_B6A6.c). */
static void farcall_cce4(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32); RAM8(0x31) = RAM8(0x33); RAM8(0x25) = 0x06; NES_PRG_SYNC();
    target(r);
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
}

/* L_A7FF — completion cutscene (entered only via the boss_life==0 tail-jump).
 * Runs to its terminal JMP L_B13D (or an early RTS on death). */
static void cutscene_a7ff(Regs *r)
{
    sub_CB69(r);                                /* JSR $CB69 */
    RAM8(0x0411) = 0x00;
    RAM8(0x0421) = 0x00;
    RAM8(0x0431) = 0x00;
    RAM8(0x00F2) = 0x00;                        /* boss_life */
    RAM8(0x85) = 0x00;
    RAM8(0x88) = 0x00;
    sub_AD7A(r);                                /* JSR L_AD7A */
    sub_A6E0(r);                                /* JSR L_A6E0 */
    RAM8(0x0200) = 0xEF;

    for (;;) {                                  /* L_A81E: walk player up */
        if (RAM8(0x45) >= 0xA0) break;          /* player_y CMP #$A0 / BCS L_A834 */
        RAM8(0x45)++;                           /* INC player_y */
        sub_AD7A(r);
        RAM8(0x36) = 0x01;
#ifdef LOTW_SHIM
        while (RAM8(0x36) != 0) nes_vblank_wait(r);  /* L_A82D: vblank wait */
#else
        while (RAM8(0x36) != 0) { }             /* L_A82D: vblank wait */
#endif
        /* JMP L_A81E */
    }

    /* L_A834 */
    RAM8(0x4E) = 0x00;
    RAM8(0x4F) = 0x00;
    sub_ACE0(r);
    sub_AD7A(r);
    RAM8(0x7C) = 0x20;                          /* scroll_x_tile */
    RAM8(0x1D) = 0x01;
    RAM8(0x8F) = 0x20;
    RAM8(0x90) = 0x80;
    RAM8(0x7A) = 0xB6;

    do { sub_A574(r); } while (RAM8(0xFA) != 0);   /* L_A854: until $FA cleared by strip draw */
    do { sub_A574(r); } while (RAM8(0xFA) != 0);   /* L_A85B */

    RAM8(0x8F) = 0x20;
    RAM8(0x90) = 0x80;
    RAM8(0x7A) = 0xB7;
    do { sub_A574(r); } while (RAM8(0xFA) != 0);   /* L_A86E */
    do { sub_A574(r); } while (RAM8(0xFA) != 0);   /* L_A875 */

    RAM8(0x10) = 0x00;
    do {                                        /* L_A880: scroll loop ($10 frames) */
        if ((RAM8(0x84) & 0x07) == 0) {         /* L_A880 head */
            RAM8(0x1D) ^= 0x01;
            RAM8(0x8F) = 0x20;
            RAM8(0x90) = 0x80;
        }
        /* L_A894 */
        r->a = 0xFF; queue_ppu_job_and_wait(r); /* LDA #$FF / JSR $CC8F */
        if (RAM8(0x26) & 0x40) {                /* BIT nmi_scratch / BVC L_A8A5 */
            r->a = 0x05; sub_AE2F(r);           /* JSR L_AE2F */
            sub_CB7F(r);                        /* JSR $CB7F */
        }
        /* L_A8A5 */
        if (RAM8(0x3E) == 0) RAM8(0x3E) = 0x02; /* LDA $3E / BNE / STA #$02 */
        /* L_A8AD */
        sub_AD7A(r);
        sub_A75D(r);
        RAM8(0x10)--;                           /* DEC $10 */
    } while (RAM8(0x10) != 0);                  /* BNE L_A880 */

    RAM8(0x1D) = 0x01;
    r->a = 0xFF; queue_ppu_job_and_wait(r);     /* LDA #$FF / JSR $CC8F */
    if (RAM8(0x58) == 0) return;                /* LDA health / BNE L_A8C5 / RTS (death) */

    /* L_A8C5 — drop player down the screen */
    RAM8(0x0200) = 0xEF;
    RAM8(0x8F) = 0x18;
    RAM8(0x90) = 0xFF;
    RAM8(0x08) = 0x01;
    for (;;) {                                  /* L_A8D6 */
        u8 prev = RAM8(0x45);                   /* player_y */
        u8 ny = (u8)(prev - RAM8(0x08));        /* SEC / SBC $08 */
        RAM8(0x45) = ny;
        /* ADC #$2B with carry from the SBC: borrow set carry=1 iff prev>=$08. */
        {
            int c = (prev >= RAM8(0x08)) ? 1 : 0;
            u8 t = (u8)(ny + 0x2B + c);
            if (t >= 0xEF) break;               /* CMP #$EF / BCS L_A8F0 */
        }
        sub_AD7A(r);
        RAM8(0x08)++;                           /* INC $08 */
        r->a = 0xFF; queue_ppu_job_and_wait(r);
        /* JMP L_A8D6 */
    }

    /* L_A8F0 — level-entry build */
    RAM8(0x0210) = 0xEF;
    RAM8(0x0214) = 0xEF;
    RAM8(0x3E) = 0x00;
    RAM8(0x3F) = 0x80;
    sub_D08A(r);                                /* JSR $D08A */
    sub_B29B(r);                                /* JSR L_B29B */
    sub_C461(r);                                /* JSR $C461 */
    sub_C38B(r);                                /* JSR $C38B */
    sub_C375(r);                                /* JSR $C375 */
    RAM8(0x48) = 0x10;                          /* map_screen_y */
    RAM8(0x47) = 0x03;                          /* map_screen_x */
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);/* far-call $C8F2 */
    RAM8(0x7C) = 0x12;                          /* scroll_x_tile */
    RAM8(0x45) = 0xC0;                          /* player_y */
    RAM8(0x44) = 0x1A;                          /* player_x_tile */
    RAM8(0x43) = 0x01;                          /* player_x_fine */
    RAM8(0x7B) = 0x01;                          /* scroll_x_fine */
    RAM8(0x56) = 0x09;
    RAM8(0x2C) = 0x35;                          /* mmc3_r2_shadow */
    RAM8(0x2D) = 0x34;                          /* mmc3_r3_shadow */
    RAM8(0x2E) = 0x36;                          /* mmc3_r4_shadow */
    RAM8(0x2F) = 0x37;                          /* mmc3_r5_shadow */
    RAM8(0x0411) = 0x01;
    RAM8(0x0421) = 0x01;
    RAM8(0x0431) = 0x01;
    RAM8(0x0441) = 0x01;
    RAM8(0x041E) = 0xA0;
    RAM8(0x042E) = 0xA0;
    RAM8(0x043E) = 0xA0;
    RAM8(0x044E) = 0x70;
    RAM8(0x044D) = 0x33;
    sub_AAAE(r);                                /* JSR L_AAAE */
    {                                           /* CLC chain of ADC #$20 stores */
        u8 v = 0x2D;
        RAM8(0x0410) = v;
        v = (u8)(v + 0x20); RAM8(0x0420) = v;
        v = (u8)(v + 0x20); RAM8(0x0430) = v;
    }
    RAM8(0x0440) = 0x81;
    RAM8(0x0412) = 0x40;
    RAM8(0x0422) = 0x40;
    RAM8(0x0432) = 0x40;
    RAM8(0x0442) = 0x40;
    sub_C57A(r);                                /* JSR $C57A */
    farcall_cce4(r, 0xCB, 0xC5, sub_C5CB);      /* far-call $C5CB */
    sub_CAB6(r);
    sub_CACC(r);
    sub_CAF8(r);
    sub_CAE2(r);
    sub_C1C7(r);
    sub_D07C(r);
    sub_C1D8(r);
    sub_C234(r);
    sub_C2B1(r);
    RAM8(0x40) = 0x07;                          /* cur_character */
    farcall_cce4(r, 0x92, 0xC4, sub_C492);      /* far-call $C492 */
    RAM8(0x8C) = 0x05;
    do { sub_AAEE(r); } while (RAM8(0x8C) != 0);/* L_A9CD */

    for (;;) {                                  /* L_A9D4 — raise player to $A0 */
        if (RAM8(0x45) == 0xA0) break;          /* player_y CMP #$A0 / BEQ L_A9FF */
        RAM8(0x45)--;
        sub_AAEE(r);
        sub_AAEE(r);
        if (RAM8(0x45) == 0xA0) break;          /* BEQ L_A9FF */
        RAM8(0x45)--;
        RAM8(0x57) ^= 0x40;
        sub_C1D8(r);
        sub_AAEE(r);
        sub_AAEE(r);
        sub_C135(r);                            /* JSR $C135 (frame commit) */
        /* JMP L_A9D4 */
    }

    /* L_A9FF */
    RAM8(0x56) = 0x0D;
    sub_C1D8(r);
    RAM8(0x8C) = 0x03;
    do { sub_AAEE(r); } while (RAM8(0x8C) != 0);/* L_AA0A */

    for (;;) {                                  /* L_AA11 — scroll columns until player_x_tile==$37 */
        RAM8(0x36) = 0x01;
        RAM8(0x7E) = RAM8(0x7C);                /* $7E <- scroll_x_tile */
        RAM8(0x20) = 0x01;
        farcall_cce4(r, 0x2B, 0xD4, game_update);   /* far-call $D42B */
        farcall_cce4(r, 0x5D, 0xC1, sub_C15D);      /* far-call $C15D */
        sub_AAAE(r);
        sub_C1D8(r);
        sub_C2B1(r);
        if (RAM8(0x7E) != RAM8(0x7C)) RAM8(0x3D)++; /* INC $3D */
        /* L_AA44 */
        sub_C135(r);                            /* JSR $C135 commit */
        if (RAM8(0x44) != 0x37) continue;       /* player_x_tile CMP #$37 / BNE L_AA11 */
        break;
    }

    /* tail: enemy spinner setup then JMP L_B13D */
    RAM8(0x56) = 0x19;
    RAM8(0x0410) = 0x39;
    RAM8(0x0420) = 0x59;
    RAM8(0x0430) = 0x79;
    RAM8(0x0440) = 0x91;
    RAM8(0x8C) = 0x14;
    do {                                        /* L_AA69 */
        RAM8(0x56)   ^= 0x04;
        RAM8(0x0410) ^= 0x04;
        RAM8(0x0420) ^= 0x04;
        RAM8(0x0430) ^= 0x04;
        RAM8(0x0440) ^= 0x04;
        sub_AAEE(r); sub_AAEE(r); sub_AAEE(r); sub_AAEE(r);
        sub_AAEE(r); sub_AAEE(r); sub_AAEE(r); sub_AAEE(r);
    } while (RAM8(0x8C) != 0);                   /* BNE L_AA69 */

    sub_B13D(r);                                /* JMP L_B13D (resume / in-game) */
}

void sub_A3E3(Regs *r)
{
    /* L_A3E3 — per-frame setup, then phase select on boss_life. */
    RAM8(0xE5) = 0x00;
    RAM8(0xE6) = 0x04;
    sub_E98F(r);                                /* JSR $E98F */
    if (RAM8(0x00F2) == 0) {                    /* LDA boss_life / BNE L_A3F5 */
        cutscene_a7ff(r);                       /* JMP L_A7FF */
        return;
    }

    /* L_A3F5 — OAM particle / boss-damage bookkeeping when nmi_scratch bit6 set. */
    if (RAM8(0x26) & 0x40) {                    /* BIT nmi_scratch / BVC L_A43C */
        u8 t = (u8)(RAM8(0x3E) + 2);            /* LDX $3E / INX / INX / TXA */
        t &= 0x06;                              /* AND #$06 */
        if (t != 0) {                           /* BEQ L_A43C (oam_sprite_engine) */
            u8 x = (u8)(t << 3);                /* ASL x3 / TAX */
            if (RAM8((u16)(0x0401 + x)) != 0) { /* LDA $0401,X / BEQ L_A43C */
                u8 sum;
                RAM8((u16)(0x0401 + x)) = 0x00;
                sum = (u8)(RAM8(0x1C) + RAM8((u16)(0x040C + x))); /* LDA $1C / CLC ADC $040C,X */
                if (sum >= 0xB0 && sum < 0xD0) {/* CMP #$B0 BCC L_A437 / CMP #$D0 BCS L_A437 */
                    /* in [$B0,$D0): damage boss */
                    u8 bl = RAM8(0x00F2);       /* boss_life */
                    if ((u8)(bl - 0x02) > bl)   /* SEC SBC #$02 / BCS L_A427 else 0 */
                        bl = 0x00;
                    else
                        bl = (u8)(bl - 0x02);
                    RAM8(0x00F2) = bl;          /* L_A427: STA boss_life */
                    sub_CB69(r);                /* JSR $CB69 */
                    RAM8(0x8F) = 0x20;
                    RAM8(0x90) = 0x01;
                } else {                        /* L_A437 */
                    RAM8(0x8F) = 0x01;
                }
            }
        }
    }

    /* L_A43C — web phase state machine (only when $FA==0). */
    if (RAM8(0xFA) != 0) goto draw;             /* LDA $FA / BNE L_A56D */

    switch (RAM8(0xF3)) {                        /* L_A443 — DEX chain dispatch */
    case 4:  goto phase4;                        /* JMP L_A549 */
    case 3:  goto phase_open;                    /* JMP L_A4F0 */
    case 2:  goto phase_close;                   /* JMP L_A4CF */
    case 1:  goto phase_grow;                    /* JMP L_A4A6 */
    default: break;                              /* 0 -> L_A45F / L_A462 */
    }

    /* L_A462 — idle: detect trigger position to start a web */
    {
        u8 sum = (u8)(RAM8(0x1C) + RAM8(0x43)); /* LDA $1C / CLC ADC player_x_fine */
        int carry = (sum < RAM8(0x1C));         /* BCS L_A48C */
        if (carry || sum >= 0xC0) goto trig_close; /* CMP #$C0 / BCS L_A48C */
        if (RAM8(0x1C) >= 0x40) goto trig_close;/* LDX $1C / CPX #$40 / BCS L_A48C */
        if (sum >= 0xA0) goto trig_l47b;        /* CMP #$A0 / BCS L_A47B */
        if (sum >= 0x80) goto trig_grow;        /* CMP #$80 / BCS L_A497 */
        /* fall through to L_A47B */
    }
trig_l47b:                                       /* L_A47B */
    if (RAM8(0x1E) >= 0xC3) goto trig_close;     /* LDA $1E / CMP #$C3 / BCS L_A48C */
    RAM8(0xF3) = 0x01;
    RAM8(0xE9) = 0x04;
    goto phase_grow;                             /* JMP L_A4A6 */
trig_close:                                      /* L_A48C */
    RAM8(0xF3) = 0x03;
    RAM8(0xE9) = 0x02;
    goto phase_open;                             /* JMP L_A4F0 */
trig_grow:                                       /* L_A497 */
    RAM8(0xF3) = 0x02;
    RAM8(0xE9) = 0x08;
    RAM8(0x7A) = 0xB3;
    goto draw;                                   /* JMP L_A56D */

phase_grow:                                      /* L_A4A6 */
    RAM8(0xE9)--;
    if (RAM8(0xE9) == 0) goto grow_done;         /* BEQ L_A4C8 */
    {
        u8 a = RAM8(0xE9);
        a = (u8)(a << 1) & 0x01;                 /* ASL A / AND #$01 */
        a = (u8)(a + 0xA0 + 0x10);               /* CLC ADC #$A0 / ADC #$10 */
        RAM8(0x7A) = a;
    }
    RAM8(0x1C) = (u8)(RAM8(0x1C) + 0x04);        /* LDA $1C / CLC ADC #$04 / STA $1C */
    if (RAM8(0x1C) >= 0x40) goto grow_done;      /* CMP #$40 / BCS L_A4C8 */
    RAM8(0x1E) = 0xC2;
    goto draw;                                   /* JMP L_A56D */
grow_done:                                       /* L_A4C8 */
    RAM8(0xF3) = 0x00;
    goto draw;                                   /* JMP L_A56D */

phase_close:                                     /* L_A4CF */
    RAM8(0xE9)--;
    if (RAM8(0xE9) == 0) goto close_done;        /* BEQ L_A4E5 */
    RAM8(0x7A) = 0xB4;
    if (RAM8(0x1E) >= 0xC3) {                    /* LDA $1E / CMP #$C3 / BCC L_A4E2 */
        RAM8(0x1E) = (u8)(RAM8(0x1E) - 0x04);    /* SEC SBC #$04 / STA $1E */
    }
    goto draw;                                   /* L_A4E2: JMP L_A56D */
close_done:                                      /* L_A4E5 */
    RAM8(0x7A) = 0xB3;
    RAM8(0xF3) = 0x00;
    goto draw;                                   /* JMP L_A56D */

phase_open:                                      /* L_A4F0 */
    RAM8(0xE9)--;
    if (RAM8(0xE9) == 0) goto open_done;         /* BEQ L_A531 */
    RAM8(0x7A) = 0xB2;
    if (RAM8(0x1C) != 0) {                       /* LDA $1C / BEQ L_A509 */
        u8 v = RAM8(0x1C);
        if ((u8)(v - 0x04) > v) v = 0x00;        /* SEC SBC #$04 / BCS L_A503 else 0 */
        else v = (u8)(v - 0x04);
        RAM8(0x1C) = v;                          /* L_A503: STA $1C */
        if (v >= 0x11) goto open_shrink;         /* CMP #$11 / BCS L_A517 */
    }
    /* L_A509 */
    if (RAM8(0x1E) < 0xC3) goto draw;            /* LDA $1E / CMP #$C3 / BCC L_A52E */
    RAM8(0x1E) = (u8)(RAM8(0x1E) - 0x04);        /* SEC SBC #$04 / STA $1E */
    goto draw;                                   /* JMP L_A52E -> L_A56D */
open_shrink:                                     /* L_A517 */
    if (RAM8(0x1E) < 0xD2) {                     /* LDA $1E / CMP #$D2 / BCC L_A529 */
        RAM8(0x1E) = (u8)(RAM8(0x1E) + 0x04);    /* L_A529: CLC ADC #$04 / STA $1E */
        goto draw;
    }
    if (RAM8(0x1C) == 0) goto draw;              /* LDA $1C / BEQ L_A52E */
    RAM8(0x1C) = (u8)(RAM8(0x1C) - 0x04);        /* SEC SBC #$04 / STA $1C */
    goto draw;                                   /* JMP L_A52E */
open_done:                                       /* L_A531 */
    if (RAM8(0x1C) != 0) {                        /* LDA $1C / BEQ L_A53C */
        RAM8(0xF3) = 0x00;
        goto draw;                               /* JMP L_A56D */
    }
    /* L_A53C */
    RAM8(0x7A) = 0xB0;
    RAM8(0xF3)++;                                /* INC $F3 */
    RAM8(0xE9) = 0x04;
    goto draw;                                   /* JMP L_A56D */

phase4:                                          /* L_A549 */
    RAM8(0xE9)--;
    if (RAM8(0xE9) == 0) goto phase4_done;        /* BEQ L_A562 */
    if (RAM8(0xE9) == 0x04) RAM8(0x8F) = 0x20;    /* CMP #$04 / BNE L_A557 */
    /* L_A557 */
    RAM8(0x7A) = 0xB5;
    RAM8(0x1E) = 0xC2;
    goto draw;                                    /* JMP L_A56D */
phase4_done:                                      /* L_A562 */
    RAM8(0x7A) = 0xB3;
    RAM8(0xF3) = 0x00;
    /* fall through to draw */

draw:                                             /* L_A56D */
    sub_A574(r);                                  /* JSR L_A574 (strip draw, far-shadow $C833) */
    sub_E99A(r);                                  /* JSR $E99A */
    return;
}
