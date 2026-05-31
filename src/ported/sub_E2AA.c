/* $E2AA — password-entry screen. Draws the keypad, zeroes the cursor state
 * ($F9/$F5/$F7) and its OAM cursor sprites, then runs the input loop: each frame
 * read the pad and dispatch the lowest set button bit to a handler —
 *   bit7 -> E372 (keypad select)      bit0 -> E39E (cursor right)
 *   bit1 -> E3AD (cursor left)        bit2 -> E3C7 (cursor down)
 *   bit3 -> E3BA (cursor up)          bit4 -> E347 (confirm/validate)
 *   bit5 -> exit (E347 tail: redraw the game screen and RTS)
 * then debounce ($8F/$36) and C135, and loop.
 *
 * INSPECTION-PORT (no diff-test spec): infinite input loop that only exits via
 * E347's PLA/PLA non-local return or the bit5 tail — not isolation-testable
 * (read_controllers yields $20=0 in flat memory). Integration-verified. */
#include "ram.h"
#include "regs.h"

void sub_C7B5(Regs *r); void sub_D15F(Regs *r); void sub_D0E5(Regs *r);
void sub_C1C7(Regs *r); void read_controllers(Regs *r); void sub_E3D6(Regs *r);
void sub_E400(Regs *r); void sub_E347(Regs *r); void sub_E3C7(Regs *r);
void sub_E39E(Regs *r); void sub_E3AD(Regs *r); void sub_E3BA(Regs *r);
void sub_E372(Regs *r); void sub_C135(Regs *r); void sub_E7B2(Regs *r);

void sub_E2AA(Regs *r)
{
    RAM8(0x7C) = 0x30;                  /* scroll_x_tile */
    sub_C7B5(r); sub_D15F(r); sub_D0E5(r); sub_C1C7(r);

    do { read_controllers(r); }         /* L_E2BA: wait for release */
    while (RAM8(0x20) != 0);

    RAM8(0xF9) = 0; RAM8(0xF5) = 0; RAM8(0xF7) = 0;
    RAM8(0x0281) = 0xF5; RAM8(0x0291) = 0xF5;     /* cursor OAM Y */
    RAM8(0x0285) = 0xF7; RAM8(0x0295) = 0xF7;
    RAM8(0x0282) = 0x00; RAM8(0x0286) = 0x00;     /* cursor OAM attrs */
    RAM8(0x0292) = 0x00; RAM8(0x0296) = 0x00;
    sub_E3D6(r);
    sub_E400(r);

    for (;;) {                          /* L_E2EB */
        u8 b;
        RAM8(0x36) = 0x01;
        read_controllers(r);
        b = RAM8(0x20);

        if (b & 0x80) {                 /* BIT $20 / BMI L_E32D */
            sub_E372(r);
            sub_D0E5(r);                /* L_E330 */
        } else if (b & 0x40) {          /* BVS L_E333 */
            /* no handler */
        } else if (b & 0x01) {          /* L_E31B */
            sub_E39E(r);
        } else if (b & 0x02) {          /* L_E321 */
            sub_E3AD(r);
        } else if (b & 0x04) {          /* L_E315 */
            sub_E3C7(r);
        } else if (b & 0x08) {          /* L_E327 -> E330 */
            sub_E3BA(r);
            sub_D0E5(r);                /* L_E330 */
        } else if (b & 0x10) {          /* L_E30F */
            sub_E347(r);
        } else if (b & 0x20) {          /* L_E364: E347 tail — exit screen */
            RAM8(0x7C) = 0x20;          /* scroll_x_tile */
            sub_C7B5(r); sub_C1C7(r); sub_E7B2(r);
            return;
        }

        /* L_E333: debounce */
        if (RAM8(0x20) & 0xCF) {
            RAM8(0x8F) = 0x0C;
            RAM8(0x36) = 0x0A;
        }
        sub_C135(r);                    /* L_E341 */
    }                                   /* JMP L_E2EB */
}
