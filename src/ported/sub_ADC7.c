/* $ADC7 — slide the player toward a valid landing spot along the $4B velocity
 * axis. Saves $4B, then repeatedly: recompute the candidate position (ACC5) and
 * test it (AE41). If clear (in bounds) it returns carry CLEAR. If blocked it
 * shrinks $4B toward zero (stepping the magnitude in by 1, the two DEX handle a
 * positive value; negative skips them) and retries while $4B != 0. If $4B
 * reaches 0 it gives up with carry SET. Original $4B is always restored. */
#include "ram.h"
#include "regs.h"

void sub_ACC5(Regs *r);
void sub_AE41(Regs *r);

void sub_ADC7(Regs *r)
{
    u8 saved = RAM8(0x4B);              /* LDA $4B / PHA */

    for (;;) {                          /* L_ADCA */
        sub_ACC5(r);                    /* JSR L_ACC5 */
        sub_AE41(r);                    /* JSR L_AE41 */
        if (!r->c)                      /* BCC L_ADE0 (carry clear: in bounds) */
            break;
        {
            u8 x = RAM8(0x4B);          /* LDX $4B */
            if (x == 0) {               /* BEQ L_ADDF */
                r->c = 1;               /* L_ADDF: SEC */
                break;
            }
            if (!(x & 0x80)) {          /* BMI L_ADDA skipped if non-negative */
                x = (u8)(x - 1);        /* DEX / DEX */
                x = (u8)(x - 1);
            }
            x = (u8)(x + 1);            /* L_ADDA: INX */
            RAM8(0x4B) = x;             /* STX $4B */
            if (x != 0)                 /* BNE L_ADCA (loop while non-zero) */
                continue;
            r->c = 1;                   /* fall through to L_ADDF: SEC */
            break;
        }
    }
    /* L_ADE0 */
    RAM8(0x4B) = saved;                 /* PLA / STA $4B */
}
