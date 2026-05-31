/* $A657 — title/warp boss move step. Loads the boss descriptor (E98F), counts
 * down its timer $EE; when it hits 0 it just writes the state back. Otherwise it
 * computes the next position (A6C5) and bounds-checks it (A6B1): off-screen kills
 * the boss ($EE=0), on-screen commits the new origin ($F9/$FB). Then if still
 * active it refreshes the facing bits (A6A2) and stores the descriptor (E99A). */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r);
void sub_A6C5(Regs *r);
void sub_A6B1(Regs *r);
void sub_A6A2(Regs *r);
void sub_E99A(Regs *r);

void sub_A657(Regs *r)
{
    sub_E98F(r);                        /* JSR $E98F */

    RAM8(0xEE) = (u8)(RAM8(0xEE) - 1);  /* DEC $EE */
    if (RAM8(0xEE) != 0) {              /* BEQ L_A678 (skip movement if timer expired) */
        sub_A6C5(r);                    /* JSR L_A6C5 */
        sub_A6B1(r);                    /* JSR L_A6B1 */
        if (r->c) {                     /* BCS L_A669 (off-screen) */
            RAM8(0xEE) = 0x00;          /* L_A669: LDA #$00 / STA $EE */
            /* JMP L_A678 */
        } else {
            RAM8(0xF9) = RAM8(0x0E);    /* L_A670: LDA $0E / STA $F9 */
            RAM8(0xFB) = RAM8(0x0A);    /* LDA $0A / STA $FB */
            /* fall to L_A678 */
        }
    }

    /* L_A678 */
    if (RAM8(0xEE) != 0)                /* LDA $EE / BEQ L_A67F */
        sub_A6A2(r);                    /* JSR L_A6A2 */
    sub_E99A(r);                        /* L_A67F: JSR $E99A */
}
