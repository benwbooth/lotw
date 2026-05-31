/* $A6A2 — copy the boss facing bits ($EE & $0C) into $ED. Reads boss flag $EE,
 * masks bits 2-3, then merges them into $ED (clearing $ED bits 2-3 first). RTS. */
#include "ram.h"
#include "regs.h"

void sub_A6A2(Regs *r)
{
    RAM8(0x08) = (u8)(RAM8(0xEE) & 0x0C);              /* LDA $EE / AND #$0C / STA $08 */
    RAM8(0xED) = (u8)((RAM8(0xED) & 0xF3) | RAM8(0x08)); /* LDA $ED / AND #$F3 / ORA $08 / STA $ED */
    (void)r;
}
