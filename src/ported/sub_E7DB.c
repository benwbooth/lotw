/* $E7DB:  STA $08 / LDA health / SEC / SBC $08 / STA health / PHP
 *         BCS skip / LDA #$00 / STA health / skip: JSR L_CAB6 / PLP / RTS
 * Subtracts A (damage) from health ($58), clamping to 0 on borrow.
 * Output carry = SBC carry (set if no borrow). Then updates health bar via sub_CAB6. */
#include "ram.h"
#include "regs.h"

void sub_CAB6(Regs *r);

void sub_E7DB(Regs *r)
{
    u8 dmg = r->a;
    u16 res;
    u8 carry;

    RAM8(0x08) = dmg;
    res = (u16)health - dmg;          /* SEC then SBC */
    health = (u8)res;
    carry = (res < 0x100);            /* C=1 means no borrow */
    if (carry == 0)                   /* BCS skip; else clamp to 0 */
        health = 0x00;
    sub_CAB6(r);                      /* reads health from RAM */
    r->c = carry;                     /* PLP restores SBC carry */
}
