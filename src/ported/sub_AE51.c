/* $AE51 — derive the per-frame x/y velocity ($49/$4B) from the D-pad. Takes the
 * low nibble of buttons $20 (the four direction bits), doubles it to index the
 * paired ROM velocity tables $FE8B (x) / $FE8C (y), and stores the looked-up
 * signed deltas into $49 and $4B. ROM tables are mapped by the harness; read via
 * RAM8(). RTS. No callees. */
#include "ram.h"
#include "regs.h"

void sub_AE51(Regs *r)
{
    r->x = (u8)((RAM8(0x20) & 0x0F) << 1);          /* AND #$0F / ASL A / TAX */
    RAM8(0x49) = RAM8((u16)(0xFE8B + r->x));        /* LDA $FE8B,X / STA $0049 */
    RAM8(0x4B) = RAM8((u16)(0xFE8C + r->x));        /* LDA $FE8C,X / STA $004B */
}
