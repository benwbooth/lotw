/* $D067:
 *   LDX equipped_item        ; X = $55
 *   LDA $51,X                ; B5 51 -> zero-page,X, wraps & 0xFF (NOT abs,X)
 *   CMP #$09 / BNE L_D078
 *   LDA magic / BEQ L_D078   ; $59
 *   LDA shot_range / ASL A / CLC / RTS   ; A = $5F<<1, C=0
 * L_D078:
 *   LDA shot_range / SEC / RTS           ; A = $5F,    C=1
 * Outputs: A (return value) and carry.
 *   carried_item0 = $0051, equipped_item = $0055, magic = $0059, shot_range = $005F
 */
#include "ram.h"
#include "regs.h"

void sub_D067(Regs *r)
{
    u8 x = RAM8(0x55);                          /* LDX equipped_item */
    r->x = x;
    if (RAM8((0x51 + x) & 0xFF) == 0x09 &&      /* carried item == 9 (zp,X wrap) */
        RAM8(0x59) != 0) {                      /* magic != 0 */
        r->a = (u8)(RAM8(0x5F) << 1);           /* shot_range * 2 */
        r->c = 0;                               /* CLC */
        return;
    }
    r->a = RAM8(0x5F);                          /* shot_range */
    r->c = 1;                                   /* SEC */
}
