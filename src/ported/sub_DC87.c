/* $DC87:
 *   LDA $86 / ORA $4F / BNE DCA6(SEC)        ; if $86|$4F != 0 -> C=1
 *   LDA $0E / BNE DCA4(CLC)                   ; if $0E != 0 -> C=0
 *   LDA $0F / STA $0C / LDA #$00 / STA $0D    ; pointer = ($0F, 0)
 *   JSR L_CA54                                ; transforms $0C/$0D pointer
 *   LDY #$00 / LDA ($0C),Y / AND #$3F / BEQ DCA6(SEC)  ; tile&$3F==0 -> C=1
 *   (fall through) DCA4: CLC RTS              ; else C=0
 * Output: carry.
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);

void sub_DC87(Regs *r)
{
    if ((RAM8(0x86) | RAM8(0x4F)) != 0) { r->c = 1; return; }   /* BNE DCA6 */
    if (RAM8(0x0E) != 0)                { r->c = 0; return; }   /* BNE DCA4 */

    RAM8(0x0C) = RAM8(0x0F);
    RAM8(0x0D) = 0x00;

    sub_CA54(r);                                /* mutates $0C/$0D */

    {
        u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
        u8 v = RAM8(ptr) & 0x3F;                /* LDY #$00; LDA ($0C),Y */
        r->c = (v == 0) ? 1 : 0;                /* BEQ DCA6(SEC) else DCA4(CLC) */
    }
}
