/* $EE19: compute an X-direction code into $F4 from ($FA - player_x_tile)
 * (0=equal, 1=positive/no-borrow, 2=borrow), then handle the Y axis:
 *   if ($FB - player_y) did NOT borrow: read flag ($E7),Y=9; if set and rng(3)==0
 *     set bit7 of $F4. If it borrowed: if rng(3)==0 set $F4 = $04.
 *   LDX #$00 / LDA $FA / SEC / SBC player_x_tile / BEQ + / INX / BCC + / INX
 *   + STX $F4 / LDA $FB / SEC / SBC player_y / BCC neg
 *     LDY #$09 / LDA ($E7),Y / BEQ done / LDA #$03 / JSR rng_update / TAX
 *     BNE done / LDA #$80 / ORA $F4 / STA $F4 / JMP done
 *   neg: LDA #$03 / JSR rng_update / TAX / BNE done / LDA #$04 / STA $F4
 *   done: RTS
 */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

#define player_x_tile RAM8(0x0044)
#define player_y      RAM8(0x0045)

void sub_EE19(Regs *r)
{
    u8 x = 0x00;
    u16 dx = (u16)((u16)RAM8(0xFA) - player_x_tile);   /* SEC / SBC */
    if ((u8)dx != 0) {                                 /* BEQ + */
        ++x;
        if (!(dx & 0x100))                             /* BCC + : carry set (no borrow) falls through to INX */
            ++x;
    }
    RAM8(0xF4) = x;

    {
        u16 dy = (u16)((u16)RAM8(0xFB) - player_y);    /* SEC / SBC */
        if (!(dy & 0x100)) {                           /* BCC neg => carry set here */
            u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
            u8 flag = RAM8((u16)(ptr + 0x09));          /* LDA ($E7),Y, Y=9 */
            if (flag != 0) {                            /* BEQ done */
                r->a = 0x03;
                rng_update(r);
                r->x = r->a;                            /* TAX */
                if (r->x == 0)                          /* BNE done */
                    RAM8(0xF4) = (u8)(RAM8(0xF4) | 0x80);
            }
        } else {                                        /* neg */
            r->a = 0x03;
            rng_update(r);
            r->x = r->a;                                /* TAX */
            if (r->x == 0)                              /* BNE done */
                RAM8(0xF4) = 0x04;
        }
    }
}
