/* $C8FF:  JSR L_C9D2 / JSR text_attr_build / JSR L_C9FB / RTS
 * Sequences three already-ported sub-routines; no register inputs of its own. */
#include "ram.h"
#include "regs.h"

void sub_C9D2(Regs *r);
void text_attr_build(Regs *r);
void sub_C9FB(Regs *r);

void sub_C8FF(Regs *r)
{
    sub_C9D2(r);
    text_attr_build(r);
    sub_C9FB(r);
}
