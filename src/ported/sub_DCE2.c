/* $DCE2 — player-vs-tile interaction dispatch (the "step into the tile ahead"
 * state machine). Using the player's current cell (player_y-1 row, player_x_tile
 * column) it builds the tile pointer via sub_CA54 and reads the tile id at the
 * head ([$0C],Y=#$00). The low 6 bits select an interaction:
 *   id $05 -> non-local transfer to L_E077 (warp/stair-style handoff)
 *   id $04 -> non-local transfer to L_E424 (door/transition handoff)
 *   id $03 -> "key-door" check: requires the equipped item to be #$0E and that
 *             exactly four #$0E items are held (counted across carried_item0..2
 *             plus the running tally in $6E); if so, non-local transfer to L_D5F3.
 * When the player straddles two tile columns (player_x_fine != 0) the same three
 * id tests are repeated against the right-hand cell ([$0C],Y=#$0C). Any other
 * result (or player_y==0, or a failed key-door check) just returns (RTS).
 *
 * INSPECTION-PORT (no diff-test spec): the $05/$04/$03 hits each do PLA / PLA
 * before JMP, a non-local return that discards this routine's caller frame and
 * tail-jumps to the grandparent's chosen handler — control flow the flat Regs ABI
 * cannot model. Those JMP targets (L_E077/L_E424/L_D5F3) are not yet ported, so
 * the transfers are documented and stubbed here. Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
/* Non-local JMP targets (PLA/PLA then JMP). Not yet ported — see header.
 * void sub_E077(Regs *r);   ($05 handler)
 * void sub_E424(Regs *r);   ($04 handler)
 * void sub_D5F3(Regs *r);   ($03 key-door handler) */

void sub_DCE2(Regs *r)
{
    u8 a;
    u16 ptr;
    u8 x = RAM8(0x45);               /* LDX player_y */
    if (x == 0)                      /* BEQ L_DD18 (RTS) */
        return;
    x = (u8)(x - 1);                 /* DEX */
    RAM8(0x0D) = x;                  /* STX $0D */
    x = RAM8(0x44);                  /* LDX player_x_tile */
    RAM8(0x0C) = x;                  /* STX $0C */

    sub_CA54(r);                     /* JSR L_CA54 (build tile pointer $0C/$0D) */
    ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));

    r->y = 0x00;                     /* LDY #$00 */
    a = RAM8((u16)(ptr + r->y)) & 0x3F;  /* LDA ($0C),Y / AND #$3F */
    if (a == 0x05) goto hit_E077;    /* CMP #$05 / BEQ L_DD19 */
    if (a == 0x04) goto hit_E424;    /* CMP #$04 / BEQ L_DD1E */
    if (a == 0x03) goto hit_D5F3;    /* CMP #$03 / BEQ L_DD23 */

    if (RAM8(0x43) == 0)             /* LDA player_x_fine / BEQ L_DD18 (RTS) */
        return;

    r->y = 0x0C;                     /* LDY #$0C — right-hand cell */
    a = RAM8((u16)(ptr + r->y)) & 0x3F;  /* LDA ($0C),Y / AND #$3F */
    if (a == 0x05) goto hit_E077;    /* CMP #$05 / BEQ L_DD19 */
    if (a == 0x04) goto hit_E424;    /* CMP #$04 / BEQ L_DD1E */
    if (a == 0x03) goto hit_D5F3;    /* CMP #$03 / BEQ L_DD23 */

    return;                          /* L_DD18: RTS */

hit_E077:
    /* L_DD19: PLA / PLA / JMP L_E077 — non-local return to grandparent.
     * Target L_E077 not yet ported; transfer documented, no data effects here. */
    return;

hit_E424:
    /* L_DD1E: PLA / PLA / JMP L_E424 — non-local return to grandparent.
     * Target L_E424 not yet ported; transfer documented, no data effects here. */
    return;

hit_D5F3:
    /* L_DD23: key-door check. */
    {
        u8 ei = RAM8(0x55);                  /* LDX equipped_item */
        if (RAM8((u16)(0x51 + ei)) != 0x0E)  /* LDA carried_item0,X / CMP #$0E / BNE L_DD18 */
            return;

        {
            u8 cnt = RAM8(0x6E);             /* LDY $6E (running tally) */
            int idx;                         /* LDX #$02; loop DEX while BPL */
            for (idx = 2; idx >= 0; idx--) { /* L_DD31: CMP carried_item0,X */
                if (RAM8((u16)(0x51 + idx)) == 0x0E)  /* (A held #$0E) BNE L_DD36 else INY */
                    cnt = (u8)(cnt + 1);
            }
            if (cnt != 0x04)                 /* CPY #$04 / BNE L_DD18 */
                return;
        }

        /* PLA / PLA / JMP L_D5F3 — non-local return to grandparent.
         * Target L_D5F3 not yet ported; transfer documented, no data effects here. */
        return;
    }
}
