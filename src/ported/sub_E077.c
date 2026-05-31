/* $E077 — top-level "warp/transition resolved" game-state handler. Reached by a
 * non-local PLA/PLA + JMP from sub_DCE2 when the player steps onto a tile whose
 * id&$3F == $05. It branches on map_screen_y ($48):
 *
 *  - map_screen_y == $10  -> L_E0F4: the "enter the master-wizard / become the
 *    final character" path. Zeroes health/magic, banks any carried items back
 *    into inventory_counts for non-final characters (cur_character<6), then sets
 *    cur_character=6, lays out that screen (E620/E660/CAB6/CACC, equips item $03,
 *    C234, places via $56/$57, E7B2/D08A/C492) and runs an input poll loop
 *    (L_E13F). The loop reads frame/scene state ($0A,$0F,$37,$20) and dispatches
 *    on the high nibble of $0A: $50 advances a song-cycle / arms a cheat that
 *    floods inventory ($37 sign + $20==$C3), $70/$80/$90 select one of the four
 *    starting characters (or, for $80, fire the E27D / E2AA sub-screens). A
 *    character pick (L_E1DC) loads that character's 4-byte stat block from ROM
 *    $FFA7 into stat_jump, sets MMC3 CHR banks, full HP/MP ($63), equips item $02,
 *    rebuilds the scene, then falls through to L_E5FD.
 *
 *  - otherwise -> L_E080: the normal level-transition path. Lay out the new
 *    screen, then loop in E08E doing the "spend up to 10 gold to advance"
 *    sequence (E514 gate; if gold<10 set $8F=$06 and retry, else burn 10 gold one
 *    at a time with CAF8/C135 feedback, rebuild scene, loop). E514 returning carry
 *    set falls through to L_E5FD.
 *
 * L_E5FD is the shared "redraw the level and resume play" tail (E642/C3E5/E79D/
 * D02E/C8FF/C5CB/C1D8/C2B1/C1C7/C492 then JMP L_D8AF). It is a separate routine
 * that is NOT yet ported, and it ends in its own non-local JMP to L_D8AF, so both
 * `JMP L_E5FD` exits here are documented non-local transfers, not C calls.
 *
 * INSPECTION-PORT (no diff-test spec): this routine is entered via a non-local
 * PLA/PLA+JMP (the caller frame is already gone) and contains read_controllers
 * wait-loops (inside the called E27D/E2AA and via E5B4) that never terminate in
 * flat memory, plus two `JMP L_E5FD` non-local exits to an unported tail. The data
 * effects and control flow are translated faithfully; validate by whole-ROM
 * integration. Integration-verified. */
#include "ram.h"
#include "regs.h"

/* named RAM not in ram.h */
#define map_screen_y     RAM8(0x48)   /* $0048 */
#define player_x_fine_v  RAM8(0x43)   /* $0043 */
#define player_y_v       RAM8(0x45)   /* $0045 */
#define carried_item0    0x51         /* $0051 carried_item0..2 */
#define equipped_item    RAM8(0x55)   /* $0055 */
#define stat_jump        0x5C         /* $005C stat_jump..+3 */
#define inventory_counts 0x60         /* $0060 inventory_counts[] */
#define scroll_x_fine    RAM8(0x7B)   /* $007B */
#define mmc3_r2_shadow   RAM8(0x2C)   /* $002C */
#define mmc3_r3_shadow   RAM8(0x2D)   /* $002D */
#define mmc3_r4_shadow   RAM8(0x2E)   /* $002E */
#define mmc3_r5_shadow   RAM8(0x2F)   /* $002F */

void sub_E5FD(Regs *r);
void sub_E620(Regs *r); void sub_E660(Regs *r); void sub_E778(Regs *r);
void sub_C492(Regs *r); void sub_E514(Regs *r); void sub_CAF8(Regs *r);
void sub_C135(Regs *r); void sub_C430(Regs *r); void sub_D16A(Regs *r);
void sub_D199(Regs *r); void sub_E667(Regs *r); void sub_E6B7(Regs *r);
void sub_CF30(Regs *r); void sub_CF82(Regs *r); void sub_C1C7(Regs *r);
void sub_C1D8(Regs *r); void sub_E4AA(Regs *r); void sub_E79D(Regs *r);
void sub_D0A5(Regs *r); void sub_CAB6(Regs *r); void sub_CACC(Regs *r);
void sub_C234(Regs *r); void sub_E7B2(Regs *r); void sub_D08A(Regs *r);
void sub_E5B4(Regs *r); void sub_E27D(Regs *r); void sub_E2AA(Regs *r);
void sub_C540(Regs *r); void sub_D07C(Regs *r); void sub_C3E5(Regs *r);
void song_init(Regs *r);

/* Non-local JMP target (PLA/PLA-style tail transfer): the shared redraw/resume
 * tail at $E5FD, itself ending in JMP $D8AF. Not yet ported.
 *   void sub_E5FD(Regs *r);   ($E5FD level-redraw tail -> JMP L_D8AF) */

void sub_E077(Regs *r)
{
    if (map_screen_y != 0x10)            /* LDA map_screen_y / CMP #$10 / BNE L_E080 */
        goto L_E080;
    goto L_E0F4;                         /* JMP L_E0F4 */

L_E080:
    sub_E620(r);
    r->a = 0x04; sub_E660(r);            /* LDA #$04 / JSR L_E660 */
    sub_E778(r);
    sub_C492(r);

L_E08E:
    sub_E514(r);                         /* JSR L_E514 */
    if (r->c)                            /* BCC L_E096 (carry set -> tail) */
        goto L_E5FD;                     /* JMP L_E5FD */
    /* L_E096 */
    if (gold < 0x0A) {                   /* LDA gold / CMP #$0A / BCS L_E0A3 */
        RAM8(0x8F) = 0x06;
        goto L_E08E;                     /* JMP L_E08E */
    }

    /* L_E0A3 — burn 10 gold one unit at a time */
    {
        u8 x = 0x0A;                     /* LDX #$0A */
        do {                             /* L_E0A5 */
            gold = (u8)(gold - 1);       /* DEC gold */
            sub_CAF8(r);
            RAM8(0x8F) = 0x0C;
            RAM8(0x36) = 0x0A;
            sub_C135(r);
            x = (u8)(x - 1);             /* DEX */
        } while (x != 0);                /* BNE L_E0A5 */
    }
    sub_C430(r);
    sub_D16A(r);
    sub_D199(r);
    r->a = 0x08; sub_E667(r);            /* LDA #$08 / JSR L_E667 */
    sub_E6B7(r);
    sub_CF30(r);
    sub_CF82(r);
    scroll_x_fine = 0x08;                /* LDA #$08 / STA scroll_x_fine */
    sub_C1C7(r);
    sub_C1D8(r);
    sub_C492(r);
    sub_E4AA(r);
    r->a = 0x04; sub_E667(r);            /* LDA #$04 / JSR L_E667 */
    sub_E79D(r);
    sub_E778(r);
    sub_C492(r);
    goto L_E08E;                         /* JMP L_E08E */

L_E0F4:
    health = 0x00;                       /* LDA #$00 / STA health */
    magic = 0x00;                        /* STA magic */
    if (cur_character < 0x06) {          /* LDA cur_character / CMP #$06 / BCS L_E112 */
        int y;                           /* LDY #$02 */
        for (y = 2; y >= 0; y--) {       /* L_E102 .. DEY / BPL L_E102 */
            u8 x = RAM8((u16)(carried_item0 + y));  /* LDX carried_item0,Y */
            if ((x & 0x80) == 0) {       /* BMI L_E108 (>=0 -> bank it) */
                RAM8((u16)(inventory_counts + x))++;  /* INC inventory_counts,X */
            }
            /* L_E108 */
            RAM8((u16)(carried_item0 + y)) = 0xFF;    /* LDX #$FF / STX carried_item0,Y */
        }
        sub_D0A5(r);                     /* JSR L_D0A5 */
    }

    /* L_E112 (BCS target falls through here when cur_character >= 6) */
    sub_E620(r);
    cur_character = 0x06;                /* LDA #$06 / STA cur_character */
    r->a = 0x06; sub_E660(r);            /* LDA #$06 / JSR L_E660 */
    sub_CAB6(r);
    sub_CACC(r);
    equipped_item = 0x03;                /* LDA #$03 / STA equipped_item */
    sub_C234(r);
    RAM8(0x56) = 0xF1;                   /* LDA #$F1 / STA $56 */
    RAM8(0x57) = 0x00;                   /* LDA #$00 / STA $57 */
    sub_C1D8(r);
    sub_E7B2(r);
    sub_D08A(r);
    sub_C492(r);

L_E13F:
    sub_E5B4(r);                         /* JSR L_E5B4 */
    {
        u8 hi = (u8)(RAM8(0x0A) & 0xF0); /* LDA $0A / AND #$F0 */
        if (hi != 0x50)                  /* CMP #$50 / BNE L_E186 */
            goto L_E186;

        /* hi nibble of $0A == $50 */
        if ((RAM8(0x0F) & 0x0F) != 0x05) /* LDA $0F / AND #$0F / CMP #$05 / BNE L_E13F */
            goto L_E13F;
        if (RAM8(0x37) == 0)             /* LDA $37 / BEQ L_E13F */
            goto L_E13F;
        {
            u8 x = (u8)(RAM8(0x8E) + 1); /* LDX $8E / INX */
            if (x >= 0x10)               /* CPX #$10 / BCC L_E15F */
                x = 0x00;                /* LDX #$00 */
            RAM8(0x8E) = x;              /* L_E15F: STX $8E */
        }
        song_init(r);
        if ((RAM8(0x37) & 0x80) == 0)    /* LDA $37 / BPL L_E13F */
            goto L_E13F;
        if (RAM8(0x20) != 0xC3)          /* LDA $20 / CMP #$C3 / BNE L_E13F */
            goto L_E13F;
        {
            int x;                       /* LDX #$0D / LDA #$10 */
            for (x = 0x0D; x >= 0; x--)  /* L_E172: STA inventory_counts,X / DEX / BPL */
                RAM8((u16)(inventory_counts + x)) = 0x10;
        }
        RAM8(0x37) = 0x80;               /* LDA #$80 / STA $37 */
        gold = 0x80;                     /* STA gold */
        keys = 0x80;                     /* STA keys */
        RAM8(0x8F) = 0x1A;               /* LDA #$1A / STA $8F */
        goto L_E13F;                     /* JMP L_E13F */
    }

L_E186:
    {
        u8 x = 0x00;                     /* LDX #$00 */
        u8 hi = (u8)(RAM8(0x0A) & 0xF0); /* A still holds $0A&$F0 from L_E13F */
        if (hi == 0x70)                  /* CMP #$70 / BEQ L_E1A8 */
            goto L_E1A8;
        x = 0x02;                        /* LDX #$02 */
        if (hi == 0x80)                  /* CMP #$80 / BEQ L_E1B8 */
            goto L_E1B8;
        if (hi != 0x90)                  /* CMP #$90 / BNE L_E13F */
            goto L_E13F;
        /* hi == $90 */
        x = 0x03;                        /* LDX #$03 */
        {
            u8 lo = (u8)(RAM8(0x0F) & 0x0F);  /* LDA $0F / AND #$0F */
            if (lo == 0x06)              /* CMP #$06 / BEQ L_E1DC */
                goto L_E1DC;
            x = (u8)(x + 1);             /* INX */
            if (lo == 0x0A)              /* CMP #$0A / BEQ L_E1DC */
                goto L_E1DC;
            goto L_E13F;                 /* JMP L_E13F */
        }

    L_E1A8:
        {
            u8 lo = (u8)(RAM8(0x0F) & 0x0F);  /* LDA $0F / AND #$0F */
            if (lo == 0x06)              /* CMP #$06 / BEQ L_E1DC */
                goto L_E1DC;
            x = (u8)(x + 1);             /* INX */
            if (lo == 0x08)              /* CMP #$08 / BEQ L_E1DC */
                goto L_E1DC;
            goto L_E13F;                 /* JMP L_E13F */
        }

    L_E1B8:
        {
            u8 lo = (u8)(RAM8(0x0F) & 0x0F);  /* LDA $0F / AND #$0F */
            if (lo == 0x04)              /* CMP #$04 / BEQ L_E1DC */
                goto L_E1DC;
            if (lo != 0x0A) {            /* CMP #$0A / BNE L_E1CE */
                /* L_E1CE */
                if (lo == 0x0C) {        /* CMP #$0C / BNE L_E1D9 */
                    RAM8(0x8F) = 0x03;   /* LDA #$03 / STA $8F */
                    sub_E2AA(r);
                }
                goto L_E13F;             /* L_E1D9: JMP L_E13F */
            }
            /* lo == $0A */
            RAM8(0x8F) = 0x03;           /* LDA #$03 / STA $8F */
            sub_E27D(r);
            goto L_E13F;                 /* JMP L_E13F */
        }

    L_E1DC:
        cur_character = x;               /* STX cur_character */
        {
            u8 a = x;                    /* TXA */
            a = (u8)(a << 1);            /* ASL A */
            a = (u8)(a << 1);            /* ASL A */
            a = (u8)(a + 0x03);          /* CLC / ADC #$03 */
            r->y = a;                    /* TAY */
        }
        {
            int xi;                      /* LDX #$03 */
            for (xi = 3; xi >= 0; xi--) {            /* L_E1E7 */
                RAM8((u16)(stat_jump + xi)) =
                    RAM8((u16)(0xFFA7 + r->y));      /* LDA $FFA7,Y / STA stat_jump,X */
                r->y = (u8)(r->y - 1);               /* DEY */
            }                                        /* DEX / BPL L_E1E7 */
        }
        RAM8(0x8F) = 0x18;               /* LDA #$18 / STA $8F */
        RAM8(0x90) = 0xFF;               /* LDA #$FF / STA $90 */
        RAM8(0x36) = 0x04;               /* LDA #$04 / STA $36 */
        sub_C135(r);
        r->x = 0x05; sub_C540(r);        /* LDX #$05 / JSR L_C540 */
        mmc3_r2_shadow = (u8)(cur_character + 0x38);  /* LDA cur_character / CLC / ADC #$38 */
        mmc3_r3_shadow = 0x3D;           /* LDA #$3D / STA mmc3_r3_shadow */
        mmc3_r4_shadow = 0x3E;           /* LDA #$3E / STA mmc3_r4_shadow */
        mmc3_r5_shadow = 0x3F;           /* LDA #$3F / STA mmc3_r5_shadow */
        RAM8(0x56) = 0x0D;               /* LDA #$0D / STA $56 */
        RAM8(0x57) = 0x00;               /* LDA #$00 / STA $57 */
        player_y_v = (u8)(player_y_v & 0xF0);  /* LDA player_y / AND #$F0 / STA player_y */
        player_x_fine_v = 0x04;          /* LDA #$04 / STA player_x_fine */
        sub_D07C(r);
        sub_C1D8(r);
        sub_C135(r);
        r->x = 0x05; sub_C540(r);        /* LDX #$05 / JSR L_C540 */
        RAM8(0x36) = 0x78;               /* LDA #$78 / STA $36 */
        sub_C135(r);
        sub_C3E5(r);
        RAM8(0x56) = 0x08;               /* LDA #$08 / STA $56 */
        RAM8(0x57) = 0x00;               /* LDA #$00 / STA $57 */
        health = 0x63;                   /* LDA #$63 / STA health */
        magic = 0x63;                    /* STA magic */
        sub_CAB6(r);
        sub_CACC(r);
        equipped_item = 0x02;            /* LDA #$02 / STA equipped_item */
        sub_C234(r);
        r->a = 0x08; sub_E660(r);        /* LDA #$08 / JSR L_E660 */
        sub_E6B7(r);
        sub_CF30(r);
        sub_CF82(r);
        scroll_x_fine = 0x08;            /* LDA #$08 / STA scroll_x_fine */
        sub_C1C7(r);
        sub_C1D8(r);
        sub_C492(r);
        sub_E4AA(r);
        goto L_E5FD;                     /* JMP L_E5FD */
    }

L_E5FD:
    /* JMP L_E5FD — tail to the shared level-redraw/resume handler ($E5FD). */
    sub_E5FD(r);
    return;
}
