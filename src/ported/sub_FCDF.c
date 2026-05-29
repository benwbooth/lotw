/* $FCDF:
 *   LDX $02
 *   LDA $A2,X / CLC / ADC #$0C / TAY      ; Y = ($A2,X + 0x0C) & 0xFF
 *   STY $9B,X
 *   LDA $FDCB,Y / STA $9C,X
 *   LDA $FDCC,Y / STA $9D,X
 *   LDA $FDCD,Y / STA $9E,X / RTS
 * X=$02. Y indexes a 3-byte ROM record at $FDCB; copies the record into
 * $9C/$9D/$9E,X and stores the index in $9B,X. All zp,X wrap & 0xFF.
 */
#include "ram.h"
#include "regs.h"

void sub_FCDF(Regs *r)
{
    u8 x = RAM8(0x02);
    u8 y = (u8)(RAM8((0xA2 + x) & 0xFF) + 0x0C);   /* CLC/ADC #$0C, TAY -> 8-bit */
    RAM8((0x9B + x) & 0xFF) = y;
    RAM8((0x9C + x) & 0xFF) = RAM8((u16)(0xFDCB + y));
    RAM8((0x9D + x) & 0xFF) = RAM8((u16)(0xFDCC + y));
    RAM8((0x9E + x) & 0xFF) = RAM8((u16)(0xFDCD + y));
    r->x = x;
    r->y = y;
    r->a = RAM8((u16)(0xFDCD + y));      /* final LDA $FDCD,Y value left in A */
}
