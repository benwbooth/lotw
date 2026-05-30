/* $EDF0:
 *   LDA $0A / AND #$0F / BNE EE17(CLC)        ; low nibble of $0A nonzero -> C=0
 *   LDA $0F / STA $0C
 *   LDA $0A / SEC / SBC #$10 / STA $0D        ; pointer = ($0F, $0A-$10)
 *   JSR L_CA54                                ; transform pointer
 *   LDY #$00 / JSR L_F2D3 / BCC EE17(CLC)     ; tile&$3F<$30 -> C=0
 *   LDA $0E / BEQ EE16(RTS, C=1)
 *   LDY #$0C / JSR L_F2D3 / BCC EE17(CLC)
 * EE16: RTS                                   ; C=1
 * Output: carry.
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_F2D3(Regs *r);

void sub_EDF0(Regs *r)
{
    if ((RAM8(0x0A) & 0x0F) != 0) { r->c = 0; return; }   /* BNE EE17 */

    RAM8(0x0C) = RAM8(0x0F);
    RAM8(0x0D) = (u8)(RAM8(0x0A) - 0x10);

    sub_CA54(r);

    r->y = 0x00;
    sub_F2D3(r);
    if (r->c == 0) return;          /* BCC EE17, C=0 */

    if (RAM8(0x0E) == 0) return;    /* BEQ EE16, C=1 */

    r->y = 0x0C;
    sub_F2D3(r);
    if (r->c == 0) return;          /* BCC EE17, C=0 */
    /* EE16: C=1 */
}
