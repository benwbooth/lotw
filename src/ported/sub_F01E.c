/* $F01E phase dispatcher (jump table @ $F033):
 *   LDY #$07 / LDA ($E7),Y / AND #$03 / ASL A / TAX
 *   LDA $F033,X / STA $0E / LDA $F034,X / STA $0F / JMP ($000E)
 * Reads the phase byte at offset 7 of the struct pointed to by $E7/$E8,
 * masks to 0..3, and dispatches to one of four phase handlers.
 * Table: $F03B, $F04B, $F071, $F0B9. */
#include "ram.h"
#include "regs.h"

void sub_F03B(Regs *r);
void sub_F04B(Regs *r);
void sub_F071(Regs *r);
void sub_F0B9(Regs *r);

void sub_F01E(Regs *r)
{
    u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
    static const u16 table[4] = { 0xF03B, 0xF04B, 0xF071, 0xF0B9 };
    u8 idx = (u8)(RAM8((u16)(ptr + 7)) & 0x03);
    u16 handler = table[idx];

    /* asm stores the table entry into the indirect-jump pointer $0E/$0F */
    RAM8(0x0E) = (u8)(handler & 0xFF);
    RAM8(0x0F) = (u8)(handler >> 8);

    /* registers as the asm leaves them at JMP ($000E) */
    r->y = 0x07;
    r->x = (u8)(idx << 1);
    r->a = (u8)(idx << 1);

    switch (idx) {
    case 0: sub_F03B(r); break;
    case 1: sub_F04B(r); break;
    case 2: sub_F071(r); break;
    case 3: sub_F0B9(r); break;
    }
}
