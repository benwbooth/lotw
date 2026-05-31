/* $E87C — per-room enemy/boss update dispatcher. Skips the HUD room
 * (map_screen_y==$10). In a boss room (mmc3_r3_shadow >= $30 -> the $E901 path),
 * runs the boss spawn/update (F349/F430/F53B/F552/F3B0 + F55E) and the 9-slot
 * minor-enemy loop; otherwise iterates the $E3..$E4 enemy slots, calling E98F to
 * load each slot's state via the ($E7)/($E8) pointers and dispatching on $EE
 * (E9A5/EF1C/EA94/EA2E/EABF), with E99A after each. Cycles $E9 each call. */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r); void sub_E99A(Regs *r); void sub_E9A5(Regs *r);
void sub_EA2E(Regs *r); void sub_EABF(Regs *r); void sub_EA94(Regs *r);
void sub_EF1C(Regs *r); void sub_F430(Regs *r); void sub_F53B(Regs *r);
void sub_F552(Regs *r); void sub_F3B0(Regs *r); void sub_F349(Regs *r);
void sub_F55E(Regs *r); void sub_EA4F(Regs *r);

void sub_E87C(Regs *r)
{
    if (RAM8(0x48) == 0x10)                 /* map_screen_y == $10 -> RTS */
        return;
    if (RAM8(0x2D) >= 0x30)                 /* mmc3_r3_shadow >= $30 -> L_E901 */
        goto L_E901;

    /* L_E88C: enemy slots $E3 = $E9*3 .. +3 */
    {
        u8 e9 = RAM8(0xE9);
        u8 v = (u8)((e9 << 1) + e9);        /* $E9*3 */
        RAM8(0xE3) = v;
        RAM8(0xE4) = (u8)(v + 3);
        u8 e5 = (u8)(RAM8(0xE3) << 4);
        RAM8(0xE5) = e5;
        RAM8(0xE7) = (u8)(e5 + 0x20);
        RAM8(0xE6) = 0x04;
        RAM8(0xE8) = RAM8(0x78);
    }
    do {                                    /* L_E8AE */
        u8 ee;
        sub_E98F(r);
        ee = RAM8(0xEE);
        if (ee == 0)         sub_E9A5(r);
        else if (ee & 0x80)  sub_EF1C(r);
        else if (ee == 0x01) sub_EA94(r);
        else if (ee >= 0x18) sub_EA2E(r);
        else                 sub_EABF(r);
        sub_E99A(r);                        /* L_E8DA */
        RAM8(0xE3)++;
        RAM8(0xE5) = (u8)(RAM8(0xE5) + 0x10);
        RAM8(0xE7) = (u8)(RAM8(0xE7) + 0x10);
    } while (RAM8(0xE3) < RAM8(0xE4));
    {                                       /* cycle $E9 0->1->2->0 */
        u8 e9 = (u8)(RAM8(0xE9) + 1);
        RAM8(0xE9) = (e9 >= 0x03) ? 0x00 : e9;
    }
    return;

L_E901:
    if (RAM8(0xE9) & 0x01)                  /* $E9 & 1 -> L_E945 */
        goto L_E945;
    /* L_E90A: single boss slot */
    RAM8(0xE5) = 0x00; RAM8(0xE6) = 0x04; RAM8(0xE3) = 0x00;
    RAM8(0xE7) = 0x20; RAM8(0xE8) = RAM8(0x78);
    sub_E98F(r);
    {
        u8 ee = RAM8(0xEE);
        if (ee == 0)             sub_F349(r);              /* spawn */
        else if (ee & 0x80)    { sub_F430(r); sub_F53B(r); sub_F552(r); }
        else                     sub_F3B0(r);
    }
    sub_E99A(r);                            /* L_E93C */
    sub_F55E(r);
    goto L_E988;

L_E945:
    RAM8(0xE3) = 0x04; RAM8(0xE5) = 0x40; RAM8(0xE6) = 0x04;
    RAM8(0xE7) = 0x60; RAM8(0xE8) = RAM8(0x78);
    do {                                    /* L_E959 */
        u8 ee;
        sub_E98F(r);
        ee = RAM8(0xEE);
        if (ee == 0 || (ee & 0x80)) {       /* free the slot */
            RAM8(0xEE) = 0x00;
            sub_EA4F(r);
        } else {
            sub_EA94(r);
        }
        sub_E99A(r);                        /* L_E96F */
        RAM8(0xE3)++;
        RAM8(0xE5) = (u8)(RAM8(0xE5) + 0x10);
        RAM8(0xE7) = (u8)(RAM8(0xE7) + 0x10);
    } while (RAM8(0xE3) < 0x09);

L_E988:
    RAM8(0xE9) ^= 0x01;
}
