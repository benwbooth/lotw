/* $DBDD: player vertical/landing collision handler.
 *   LDA $86 / BNE DBE5 ; LDA $4F / BEQ DBEC ; -> DBE5
 * DBE5: $50=0 / JMP DC82
 * DBEC: $0C=$0F=player_x_tile; $0E=player_x_fine; X=player_y -> $0D; INX -> $0A
 *       JSR CA54
 *       LDA player_x_fine / BNE DC10
 *         $50=1 / LDY#0 / LDA($0C),Y &$3F / BEQ DC4D
 * DC10: $50=0 / LDA player_y / CMP #$B0 / BCS DC4A
 *       JSR CDB2 / BCC DC38
 *         LDA $2D / CMP #$30 / BCS DC4D
 *         LDY equipped_item / LDX carried_item0,Y / CPX #$05 / BNE DC4D
 *         LDA $4E / BEQ DC4D
 *         LDX $09 / LDA #$80 / STA $0401,X
 * DC38: LDY#1 / JSR DCCC / BCS DC4D
 *       LDA player_x_fine / BEQ DC4A
 *       LDY#$0D / JSR DCCC / BCS DC4D
 * DC4A: INC $4E / RTS
 * DC4D: LDA $4E / CMP stat_jump / BCC DC6E
 *         SEC SBC #$07 / CMP stat_jump / BCC DC5C / LDA stat_jump
 *       DC5C: SEC SBC #$01 / STA $4F / CLC ADC #$0A / STA $46 / LDA#$0A / STA $8F / JSR E7CE
 * DC6E: LDA $4E / BNE DC82
 *       LDY#1 / JSR DCA8 / BCS DC82
 *       LDA player_x_fine / BEQ DC82
 *       LDY#$0D / JSR DCA8
 * DC82: $4E=0 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_CDB2(Regs *r);
void sub_DCCC(Regs *r);
void sub_DCA8(Regs *r);
void sub_E7CE(Regs *r);

#define player_x_fine RAM8(0x43)
#define player_x_tile RAM8(0x44)
#define player_y      RAM8(0x45)
#define equipped_item RAM8(0x55)
#define stat_jump     RAM8(0x5C)

void sub_DBDD(Regs *r)
{
    u8 x;

    /* DBDD */
    if (RAM8(0x86) == 0 && RAM8(0x4F) == 0) {
        /* DBEC */
        RAM8(0x0C) = player_x_tile;
        RAM8(0x0F) = player_x_tile;
        RAM8(0x0E) = player_x_fine;
        RAM8(0x0D) = player_y;
        RAM8(0x0A) = (u8)(player_y + 1);
        sub_CA54(r);

        if (player_x_fine == 0) {
            RAM8(0x50) = 0x01;
            r->y = 0x00;
            {
                u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
                if ((RAM8((u16)(ptr + r->y)) & 0x3F) == 0)
                    goto dc4d;
            }
            /* fall through to DC10 */
        }

        /* DC10 */
        RAM8(0x50) = 0x00;
        if (player_y >= 0xB0)
            goto dc4a;

        sub_CDB2(r);
        if (r->c) {                                  /* BCC DC38 not taken */
            if (RAM8(0x2D) >= 0x30) goto dc4d;       /* CMP #$30 / BCS DC4D */
            {
                u8 y = equipped_item;                /* LDY equipped_item */
                x = RAM8((u16)(0x0051 + y));         /* LDX carried_item0,Y */
            }
            if (x != 0x05) goto dc4d;                /* CPX #$05 / BNE DC4D */
            if (RAM8(0x4E) == 0) goto dc4d;          /* LDA $4E / BEQ DC4D */
            x = RAM8(0x09);                          /* LDX $09 */
            RAM8((u16)(0x0401 + x)) = 0x80;          /* LDA #$80 / STA $0401,X */
        }

        /* DC38 */
        r->y = 0x01;
        sub_DCCC(r);
        if (r->c) goto dc4d;                         /* BCS DC4D */
        if (player_x_fine == 0) goto dc4a;           /* BEQ DC4A */
        r->y = 0x0D;
        sub_DCCC(r);
        if (r->c) goto dc4d;                         /* BCS DC4D */

    dc4a:
        RAM8(0x4E) = (u8)(RAM8(0x4E) + 1);           /* INC $4E */
        return;

    dc4d:
        {
            u8 v = RAM8(0x4E);                        /* LDA $4E */
            if (v >= stat_jump) {                     /* CMP / BCC DC6E inverted */
                v = (u8)(v - 0x07);                   /* SEC SBC #$07 */
                if (v >= stat_jump)                   /* CMP / BCC DC5C inverted */
                    v = stat_jump;                    /* LDA stat_jump */
                /* DC5C */
                v = (u8)(v - 0x01);                   /* SEC SBC #$01 */
                RAM8(0x4F) = v;                       /* STA $4F */
                RAM8(0x46) = (u8)(v + 0x0A);          /* CLC ADC #$0A / STA $46 */
                RAM8(0x8F) = 0x0A;                    /* LDA #$0A / STA $8F */
                sub_E7CE(r);                          /* JSR E7CE */
            }
        }
        /* DC6E */
        if (RAM8(0x4E) != 0) goto dc82;              /* BNE DC82 */
        r->y = 0x01;
        sub_DCA8(r);
        if (r->c) goto dc82;                         /* BCS DC82 */
        if (player_x_fine == 0) goto dc82;           /* BEQ DC82 */
        r->y = 0x0D;
        sub_DCA8(r);
        /* fall to DC82 */
    } else {
        /* DBE5 */
        RAM8(0x50) = 0x00;
        /* JMP DC82 */
    }

dc82:
    RAM8(0x4E) = 0x00;
}
