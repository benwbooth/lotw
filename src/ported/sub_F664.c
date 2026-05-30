/* $F664 — fire/throw player weapon: build a projectile entity from the player
 * position and equipped weapon, spending magic as needed.
 *
 *   JSR E98F                       ; load 16-byte entity record from ($E5)
 *   $FD = ($20 & $40) | $FD
 *   Y = 2; if $88 != 0: Y = 4
 *   A = $FD; JSR CD70              ; build per-step delta from selector $FD, count Y
 *   JSR F740                       ; target pos -> $0E/$0F/$0A
 *   JSR CF08; BCS done             ; off-screen? bail
 *   JSR E7F0; BCS done             ; no magic? bail
 *   $F9=$0E; $FA=$0F; $FB=$0A
 *   JSR D067; $EE=A; if C==0: JSR E7F0     ; range, spend magic
 *   JSR D051; $F8=A; if C==0: JSR E7F0     ; strength, spend magic
 *   $EF=0; $ED=$21; $8F = $22 + cur_character
 * done (L_F735 tail):
 *   if $EE != 0: JSR F773
 *   JSR E99A                       ; store 16-byte entity record back to ($E5)
 */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r);
void sub_CD70(Regs *r);
void sub_F740(Regs *r);
void sub_CF08(Regs *r);
void sub_E7F0(Regs *r);
void sub_D067(Regs *r);
void sub_D051(Regs *r);
void sub_F773(Regs *r);
void sub_E99A(Regs *r);

void sub_F664(Regs *r)
{
    sub_E98F(r);

    RAM8(0xFD) = (u8)((RAM8(0x20) & 0x40) | RAM8(0xFD));

    r->y = (RAM8(0x88) != 0) ? 0x04 : 0x02;
    r->a = RAM8(0xFD);
    sub_CD70(r);

    sub_F740(r);

    sub_CF08(r);
    if (r->c) goto done;             /* BCS L_F6B8 */

    sub_E7F0(r);
    if (r->c) goto done;             /* BCS L_F6B8 */

    RAM8(0xF9) = RAM8(0x0E);
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xFB) = RAM8(0x0A);

    sub_D067(r);
    RAM8(0xEE) = r->a;
    if (r->c == 0)                   /* BCS skips spend */
        sub_E7F0(r);

    sub_D051(r);
    RAM8(0xF8) = r->a;
    if (r->c == 0)
        sub_E7F0(r);

    RAM8(0xEF) = 0x00;
    RAM8(0xED) = 0x21;
    RAM8(0x8F) = (u8)(0x22 + RAM8(0x40));   /* $22 + cur_character */

done:
    /* L_F735 tail */
    if (RAM8(0xEE) != 0)
        sub_F773(r);
    sub_E99A(r);
}
