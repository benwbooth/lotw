/* $FBC5 (sound command handler, X = channel index):
 *   PHA / AND #$F0 / ASL A / ASL A / STA $00
 *   LDA $99,X / AND #$3F / ORA $00 / STA $99,X
 *   PLA / ASL A / ASL A / ASL A / ASL A / STA $A2,X / TAY
 *   LDA $FDD2,Y / STA $9A,X / RTS
 * Packs the high nibble of A into bits6-7 of $99,X; stores (A<<4) in $A2,X and
 * indexes ROM table $FDD2 to set $9A,X. A enters via the dispatcher (the byte
 * loaded at $FBA8); X is the channel index ($02). */
#include "ram.h"
#include "regs.h"

void sub_FBC5(Regs *r)
{
    u8 a = r->a;
    u8 x = r->x;
    u8 hi = (u8)((u8)(a & 0xF0) << 2);      /* ASL twice, 8-bit */
    RAM8(0x00) = hi;
    RAM8((0x99 + x) & 0xFF) = (u8)((RAM8((0x99 + x) & 0xFF) & 0x3F) | hi);
    a = (u8)(a << 4);                        /* PLA then ASL x4 */
    RAM8((0xA2 + x) & 0xFF) = a;
    RAM8((0x9A + x) & 0xFF) = RAM8((u16)(0xFDD2 + a));
    r->a = RAM8((u16)(0xFDD2 + a));
    r->y = a;
    r->x = x;
}
