/* $C8F2 scene_assemble:
 *   JSR L_C9D2 (sub_C9D2)
 *   JSR L_C9A9 (sub_C9A9)
 *   JSR text_attr_build
 *   JSR L_C9FB (sub_C9FB)
 *   RTS
 * Pure sequential dispatch; each callee reads its inputs from r/RAM. */
#include "ram.h"
#include "regs.h"

void sub_C9D2(Regs *r);
void sub_C9A9(Regs *r);
void text_attr_build(Regs *r);
void sub_C9FB(Regs *r);

void scene_assemble(Regs *r)
{
    sub_C9D2(r);
    /* sub_C9D2's ported C does not model the carry left by its final
     * "CLC / ADC #$03 / STA $78". The real chain propagates that carry through
     * sub_C9A9 (which only does INY/INC, no carry change) into text_attr_build's
     * "ADC #$A0". Reproduce it here so $7A is computed correctly. */
    r->c = (u8)(((RAM8(0x76) + 0x03) > 0xFF) ? 1 : 0);
    sub_C9A9(r);
    text_attr_build(r);
    sub_C9FB(r);
}
