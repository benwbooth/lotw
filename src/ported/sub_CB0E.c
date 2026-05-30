/* $CB0E:
 *   TXA / PHA
 *   LDY #$05 / LDA #$DC
 * L_CB14: STA $0101,X / INX / DEY / BNE L_CB14
 *   PLA / PHA / TAX
 *   LDY #$05 / LDA #$DF
 * L_CB22: STA $0121,X / INX / DEY / BNE L_CB22
 *   PLA / TAX
 *   JSR L_CBFA           ; divides $08 by 10: Y=count, $08=remainder
 *   TXA
 * L_CB2F: DEY / BEQ L_CB3F / DEC $0101,X / DEY / BEQ L_CB3F / DEC $0101,X / INX / JMP L_CB2F
 * L_CB3F: TAX / LDY $08
 * L_CB42: DEY / BEQ L_CB52 / DEC $0121,X / DEY / BEQ L_CB52 / DEC $0121,X / INX / JMP L_CB42
 * L_CB52: RTS
 *
 * Input X = starting OAM-ish index. Fills 5 tiles at $0101+X with $DC and 5 at
 * $0121+X with $DF, then decrements them per the $08/10 split from sub_CBFA.
 *
 * NB: the routine PHA/PLA-saves X through the stack at $01FB (the slot just below
 * the caller's return address). The two fill loops walk $0101+X / $0121+X and, for
 * large X, can write OVER $01FB — so the value PLA later restores is the clobbered
 * byte, not the original X. We model the saved-X slot as RAM8($01FB) to reproduce
 * this aliasing exactly (matches the diff oracle's run_routine stack layout). */
#include "ram.h"
#include "regs.h"

#define STK_SLOT 0x01FB     /* PHA slot: s=0xFB after the oracle pushes its sentinel */

void sub_CBFA(Regs *r);

void sub_CB0E(Regs *r)
{
    u8 x, y, a;
    int i;

    /* TXA / PHA: save X to the stack slot */
    a = r->x;
    RAM8(STK_SLOT) = a;

    /* first fill: $DC into $0101,X for 5 bytes (X increments) */
    x = a;
    for (i = 0; i < 5; i++)
        RAM8((u16)(0x0101 + x++)) = 0xDC;

    /* PLA / PHA / TAX: reload saved X from the (possibly clobbered) slot */
    a = RAM8(STK_SLOT);
    RAM8(STK_SLOT) = a;          /* PHA re-push */
    x = a;                       /* TAX */

    /* second fill: $DF into $0121,X for 5 bytes */
    for (i = 0; i < 5; i++)
        RAM8((u16)(0x0121 + x++)) = 0xDF;

    /* PLA / TAX: reload again (second fill may have clobbered the slot) */
    a = RAM8(STK_SLOT);
    x = a;

    r->x = x;
    sub_CBFA(r);                 /* uses $08; sets r->y = count, $08 = remainder */
    y = r->y;

    a = x;                       /* TXA */

    /* L_CB2F loop over $0101,X using Y */
    x = a;
    for (;;) {
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0101 + x))--;
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0101 + x))--;
        x = (u8)(x + 1);
    }

    /* L_CB3F: TAX (restore X=a), LDY $08 */
    x = a;
    y = RAM8(0x08);
    for (;;) {
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0121 + x))--;
        y = (u8)(y - 1);
        if (y == 0) break;
        RAM8((u16)(0x0121 + x))--;
        x = (u8)(x + 1);
    }

    r->y = y;
    r->x = x;
    r->a = a;
}
