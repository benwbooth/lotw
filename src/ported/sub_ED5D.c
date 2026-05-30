/* $ED5D: step a projectile and test for terrain collision.
 *   LDY #$01
 *   JSR CD70          ; build per-step delta (A = caller's selector, Y=1 count)
 *   JSR EFF1          ; target pos = base + delta -> $0E/$0F/$0A
 *   JSR CE7C          ; terrain check; sets C
 *   BCC L_ED6E        ; no collision -> return C=0
 *   JSR F136          ; collision -> apply effect
 *   SEC               ; return C=1
 * L_ED6E: RTS
 * Output carry = CE7C's carry. A on entry is the delta selector for CD70. */
#include "ram.h"
#include "regs.h"

void sub_CD70(Regs *r);
void sub_EFF1(Regs *r);
void sub_CE7C(Regs *r);
void sub_F136(Regs *r);

void sub_ED5D(Regs *r)
{
    r->y = 0x01;                /* LDY #$01; A already holds caller's selector */
    sub_CD70(r);

    sub_EFF1(r);

    sub_CE7C(r);
    if (r->c == 0)              /* BCC L_ED6E */
        return;

    sub_F136(r);
    r->c = 1;                   /* SEC */
}
