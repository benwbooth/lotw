/* $A7D2 — load the player HP meter metasprite. Copies the $40-byte OAM template
 * at ROM $AAFC into the OAM shadow $0240, then renders the HP value via $CB69.
 * $AAFC is bank-13 ROM (mapped by the harness; read via RAM8()). RTS. */
#include "ram.h"
#include "regs.h"

void sub_CB69(Regs *r);

void sub_A7D2(Regs *r)
{
    int x;
    for (x = 0x3F; x >= 0; x--)               /* LDX #$3F / L_A7D4 / DEX / BPL */
        RAM8((u16)(0x0240 + x)) = RAM8((u16)(0xAAFC + x));
    sub_CB69(r);                              /* JSR $CB69 */
}
