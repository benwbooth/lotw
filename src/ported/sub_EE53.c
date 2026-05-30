/* $EE53:
 *   LDA $FA / STA $0F
 *   LDA $F9 / STA $0E
 *   LDA $FB / STA $0A
 *   JSR L_CE90 / LDX #$00 / BCS EE6F          ; horizontal proximity
 *     LDA $FA / SEC / SBC player_x_tile / INX / BCC EE6F / INX
 * EE6F: STX $F4
 *   JSR L_CEB6 / LDX #$00 / BCS EE83          ; vertical proximity
 *     LDA $FB / SEC / SBC player_y / LDX #$04 / BCC EE83 / LDX #$08
 * EE83: TXA / ORA $F4 / STA $F4
 *   LDA #$00 / STA $F3
 *   RTS
 * Builds a direction code in $F4 toward the player; clears $F3.
 * Output: RAM ($0F,$0E,$0A,$F4,$F3).
 */
#include "ram.h"
#include "regs.h"

#define player_x_tile RAM8(0x44)
#define player_y      RAM8(0x45)

void sub_CE90(Regs *r);
void sub_CEB6(Regs *r);

void sub_EE53(Regs *r)
{
    u8 x;

    RAM8(0x0F) = RAM8(0xFA);
    RAM8(0x0E) = RAM8(0xF9);
    RAM8(0x0A) = RAM8(0xFB);

    sub_CE90(r);
    x = 0x00;
    if (r->c == 0) {                            /* BCS EE6F skipped */
        u8 d = (u8)(RAM8(0xFA) - player_x_tile);   /* SEC SBC */
        u8 carry = (RAM8(0xFA) >= player_x_tile) ? 1 : 0;
        x = 0x01;                               /* INX */
        if (carry) x = 0x02;                    /* BCC EE6F skipped -> INX */
        (void)d;
    }
    RAM8(0xF4) = x;

    sub_CEB6(r);
    x = 0x00;
    if (r->c == 0) {                            /* BCS EE83 skipped */
        u8 carry = (RAM8(0xFB) >= player_y) ? 1 : 0;  /* SEC SBC */
        x = 0x04;
        if (carry) x = 0x08;                    /* BCC EE83 skipped */
    }
    RAM8(0xF4) = (u8)(x | RAM8(0xF4));

    RAM8(0xF3) = 0x00;
}
