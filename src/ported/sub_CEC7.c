/* $CEC7:  LDA #$00 / STA $EA / LDA $0A / SEC SBC player_y / CMP #$10 / BCC y_ok
 *         CMP #$E1 / BCC clr / (fall) y_ok: SEC LDA $0F SBC player_x_tile
 *         BEQ set / CMP #$FF BEQ set / CMP #$02 BCC one / CMP #$FE BCC clr
 *         SEC LDA $0E SBC player_x_fine / BEQ clr / BMI clr / JMP set
 *  one:   LDA $0E SEC SBC player_x_fine / BMI set / (fall) clr
 *  clr: CLC RTS    set: LDA #$01 STA $EA SEC RTS
 * Proximity test: sets $EA and carry when player is adjacent in X and within Y window.
 * Outputs: RAM ($EA), carry. */
#include "ram.h"
#include "regs.h"

#define player_x_fine RAM8(0x43)
#define player_x_tile RAM8(0x44)
#define player_y      RAM8(0x45)

void sub_CEC7(Regs *r)
{
    u8 dy, dx;
    RAM8(0xEA) = 0x00;

    dy = (u8)(RAM8(0x0A) - player_y);          /* SEC SBC -> exact */
    /* CMP #$10 BCC y_ok: if dy < $10 proceed; else CMP #$E1 BCC clr */
    if (dy >= 0x10 && dy < 0xE1) {             /* clr */
        r->c = 0;
        return;
    }

    dx = (u8)(RAM8(0x0F) - player_x_tile);
    if (dx == 0)    goto set;                  /* BEQ set */
    if (dx == 0xFF) goto set;                  /* CMP #$FF BEQ set */
    if (dx < 0x02) {                           /* CMP #$02 BCC one -> dx==1 */
        u8 f = (u8)(RAM8(0x0E) - player_x_fine);
        if (f & 0x80) goto set;                /* BMI set */
        r->c = 0;                              /* fall clr */
        return;
    }
    if (dx < 0xFE) { r->c = 0; return; }       /* CMP #$FE BCC clr */
    /* dx == $FE */
    {
        u8 f = (u8)(RAM8(0x0E) - player_x_fine);
        if (f == 0)   { r->c = 0; return; }    /* BEQ clr */
        if (f & 0x80) { r->c = 0; return; }    /* BMI clr */
        goto set;
    }

set:
    RAM8(0xEA) = 0x01;
    r->c = 1;
}
