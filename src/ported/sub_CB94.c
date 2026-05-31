/* $CB94 — draw a two-row meter bar (HP/boss life). X=full-cell tile, Y=empty
 * tile; $09 = OAM column base, $08 = bar length. Fills the 5+5 OAM tile slots
 * at $0259+/$026D+ with X/Y, splits $08 into tens (via CBFA) and units, then
 * decrements the bar-segment OAM bytes ($0241,col) two-at-a-time for the tens
 * run (base+$18) and the units run (base+$2C).  Pure OAM/data; tail of CB53. */
#include "ram.h"
#include "regs.h"

void sub_CBFA(Regs *r);

void sub_CB94(Regs *r)
{
    u8 x = RAM8(0x09);                 /* LDX $09 (column base) */
    u8 full = r->x;                    /* TXA (full-cell tile) */
    RAM8((u16)(0x0259 + x)) = full;
    RAM8((u16)(0x025D + x)) = full;
    RAM8((u16)(0x0261 + x)) = full;
    RAM8((u16)(0x0265 + x)) = full;
    RAM8((u16)(0x0269 + x)) = full;
    {
        u8 empty = r->y;               /* TYA (empty-cell tile) */
        RAM8((u16)(0x026D + x)) = empty;
        RAM8((u16)(0x0271 + x)) = empty;
        RAM8((u16)(0x0275 + x)) = empty;
        RAM8((u16)(0x0279 + x)) = empty;
        RAM8((u16)(0x027D + x)) = empty;
    }

    sub_CBFA(r);                       /* JSR L_CBFA: Y = tens count, $08 = units */

    {   /* L_CBBF: tens run at column base + $18 */
        u8 y = r->y;
        u8 xx = (u8)(RAM8(0x09) + 0x18);
        for (;;) {
            if (--y == 0) break;       /* DEY / BEQ */
            RAM8((u16)(0x0241 + xx)) -= 2;
            if (--y == 0) break;
            RAM8((u16)(0x0241 + xx)) -= 2;
            xx = (u8)(xx + 4);         /* INX x4 */
        }
    }
    {   /* L_CBE0: units run at column base + $2C */
        u8 xx = (u8)(RAM8(0x09) + 0x2C);
        u8 y = RAM8(0x08);             /* LDY $08 */
        for (;;) {
            if (--y == 0) break;
            RAM8((u16)(0x0241 + xx)) -= 2;
            if (--y == 0) break;
            RAM8((u16)(0x0241 + xx)) -= 2;
            xx = (u8)(xx + 4);
        }
    }
}
