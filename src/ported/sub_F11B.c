/* $F11B:
 *   JSR L_EFF1
 *   JSR L_CE7C / BCC L_F128
 *     JSR L_F136 / SEC / RTS
 * L_F128:
 *   JSR L_CF08 / BCC L_F135
 *     LDA #$00 / STA $EE / LDA #$F0 / STA $F3
 * L_F135: RTS
 */
#include "ram.h"
#include "regs.h"

void sub_EFF1(Regs *r);
void sub_CE7C(Regs *r);
void sub_F136(Regs *r);
void sub_CF08(Regs *r);

void sub_F11B(Regs *r)
{
    sub_EFF1(r);

    sub_CE7C(r);
    if (r->c) {                 /* BCC L_F128 taken when C=0; here C=1 */
        sub_F136(r);
        r->c = 1;               /* SEC */
        return;
    }

    sub_CF08(r);
    if (r->c == 0)              /* BCC L_F135 */
        return;

    RAM8(0xEE) = 0x00;
    RAM8(0xF3) = 0xF0;
}
