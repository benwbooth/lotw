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
    sub_C9A9(r);
    /* The carry into text_attr_build's first "ADC #$A0" (which forms the tile-table
     * pointer $7A) is the one left by sub_C9D2's final "CLC / ADC #$03 / STA $78".
     * sub_C9A9 in between only does INY/INC/LDA/STA — none touch the 6502 carry — so
     * on hardware C9D2's carry survives to here. The ported C9A9 clobbers r->c, so
     * recompute it from $76 right before text_attr_build (not before C9A9, where it
     * gets overwritten). Without this $7A was off by +1 -> wrong tile table -> the
     * whole room nametable rendered with wrong tiles. */
    r->c = (u8)(((RAM8(0x76) + 0x03) > 0xFF) ? 1 : 0);
    text_attr_build(r);
    sub_C9FB(r);
}
