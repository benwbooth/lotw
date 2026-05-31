/* $A622 — title/warp boss spawn step. Loads the boss descriptor into $00ED..
 * (E98F), folds the $20 direction bit into the boss-state byte $FD, derives the
 * per-axis step deltas ($F5/$F7) via A7B1 (selector=$FD, count=2), computes the
 * spawn point (A683) and checks bounds (A6B1). If on-screen it places the boss
 * ($F9/$FB), sets the active/anim flags ($EE/$EF/$ED/$8F). Finally, if $EE !=0
 * it refreshes the facing bits (A6A2), then writes the descriptor back (E99A). */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r);
void sub_A7B1(Regs *r);
void sub_A683(Regs *r);
void sub_A6B1(Regs *r);
void sub_A6A2(Regs *r);
void sub_E99A(Regs *r);

void sub_A622(Regs *r)
{
    sub_E98F(r);                        /* JSR $E98F */

    RAM8(0xFD) = (u8)((RAM8(0x20) & 0x40) | RAM8(0xFD)); /* LDA $20 / AND #$40 / ORA $FD / STA $FD */

    r->a = RAM8(0xFD);                  /* LDA $FD */
    r->y = 0x02;                        /* LDY #$02 */
    sub_A7B1(r);                        /* JSR L_A7B1 */
    sub_A683(r);                        /* JSR L_A683 */
    sub_A6B1(r);                        /* JSR L_A6B1 */
    if (!r->c) {                        /* BCS L_A678 (skip placement if off-screen) */
        RAM8(0xF9) = RAM8(0x0E);        /* LDA $0E / STA $F9 */
        RAM8(0xFB) = RAM8(0x0A);        /* LDA $0A / STA $FB */
        RAM8(0xEE) = 0x18;              /* LDA #$18 / STA $EE */
        RAM8(0xEF) = 0x00;              /* LDA #$00 / STA $EF */
        RAM8(0xED) = 0x21;              /* LDA #$21 / STA $ED */
        RAM8(0x8F) = 0x19;              /* LDA #$19 / STA $8F */
        /* JMP L_A678 */
    }

    /* L_A678 */
    if (RAM8(0xEE) != 0)                /* LDA $EE / BEQ L_A67F */
        sub_A6A2(r);                    /* JSR L_A6A2 */
    sub_E99A(r);                        /* L_A67F: JSR $E99A */
}
