/* $D4DF — jump/fall physics step. If a jump is active ($4F!=0) or just starting
 * ($22==0), abort the normal update frame (PLA/PLA discards the caller's return)
 * and drive the jump: compute the vertical delta into $4B, run the object scan
 * (D991) for the new position, settle X via a second D991 / DF90, then commit the
 * trial position (player_x_fine/tile, player_y) or land ($4F=$4E=0). Always ends
 * by redrawing through D8AF (= D8E3 + D94E). The early "$4F==0 && $22!=0" case
 * returns normally (no jump in progress).
 *
 * INSPECTION-PORT (no diff-test spec): the PLA/PLA is a non-local return to the
 * grandparent frame that the flat Regs ABI can't model, and it tail-drives the
 * D991/DF90 integration tail. Validate by whole-ROM integration. */
#include "ram.h"
#include "regs.h"

void sub_E7F0(Regs *r); void sub_D991(Regs *r); void sub_DF90(Regs *r);
void sub_DBDD(Regs *r); void sub_D8E3(Regs *r); void sub_D94E(Regs *r);

void sub_D4DF(Regs *r)
{
    if (RAM8(0x4F) != 0)                 /* LDX $4F / BNE L_D506 */
        goto L_D506;
    if (RAM8(0x22) != 0)                 /* LDA $22 / BEQ L_D4E8; else RTS */
        return;

    /* L_D4E8 — start a jump */
    RAM8(0x8F) = 0x1B;
    RAM8(0x4F) = RAM8(0x5C);             /* stat_jump -> $4F */
    {
        u8 x = RAM8(0x55);               /* LDX equipped_item */
        if (RAM8((u16)(0x51 + x)) == 0x06) {  /* carried_item0,X CMP #$06 / BNE L_D506 */
            sub_E7F0(r);                 /* JSR L_E7F0 */
            if (!r->c) {                 /* BCS L_D506 (carry clear -> boost) */
                u8 f = RAM8(0x4F);       /* LDA $4F / LSR / LSR / CLC / ADC $4F */
                RAM8(0x4F) = (u8)((f >> 2) + f);
            }
        }
    }

L_D506:
    /* PLA / PLA — drop the caller's return frame (non-local return; see header) */
    RAM8(0x22) = 0x01;
    {
        u8 old4f = RAM8(0x4F);           /* LDA $4F */
        RAM8(0x4F) = (u8)(old4f - 1);    /* DEC $4F */
        u8 t = (u8)(old4f >> 2);         /* LSR A / LSR A */
        RAM8(0x4B) = (u8)((t ^ 0xFF) + 1);  /* EOR #$FF / CLC / ADC #$01 (negate) */
    }
    sub_D991(r);                         /* JSR L_D991 */
    if (!r->c)                           /* BCS L_D521; else JMP L_D536 */
        goto L_D536;

    /* L_D521 */
    RAM8(0x49) = 0x00; RAM8(0x4A) = 0x00;
    sub_D991(r);
    if (!r->c)                           /* BCC L_D536 */
        goto L_D536;
    RAM8(0x4F)++;                        /* INC $4F */
    sub_DF90(r);
    if (!r->c)                           /* BCC L_D536 */
        goto L_D536;
    goto L_D54E;

L_D536:
    RAM8(0x43) = RAM8(0x0E);             /* $0E -> player_x_fine */
    RAM8(0x44) = RAM8(0x0F);             /* $0F -> player_x_tile */
    {
        u8 y = RAM8(0x0A);               /* LDA $0A / CMP #$EF / BCC / LDA #$00 */
        if (y >= 0xEF) y = 0x00;
        RAM8(0x45) = y;                  /* player_y */
    }
    sub_DBDD(r);                         /* JSR L_DBDD */
    goto L_D8AF;

L_D54E:
    RAM8(0x4F) = 0x00; RAM8(0x4E) = 0x00;
    sub_DBDD(r);

L_D8AF:                                  /* L_D8AF: JSR D8E3 / JSR D94E / RTS */
    sub_D8E3(r);
    sub_D94E(r);
}
