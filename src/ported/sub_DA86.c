/* $DA86 — item-action dispatcher (no OAM-clear prologue).
 *   n = A - 2; $04A1 = 0.
 *   n >= $18  -> $8F = 6, RTS                       (L_DB01)
 *   n in [8,$18) -> inventory-add path (shared L_DAD2)
 *   n < 8     -> JMP ($DB16 + n*2): one of
 *                {D16A,D199,DB47,DB52,DB66,DB7B,DBB7,DB9B}[n]
 * The JMP ($000C) tail-dispatches; the handler's RTS returns to DA86's caller,
 * modelled here as call-then-return. Targets ignore the dispatcher's A/X. */
#include "ram.h"
#include "regs.h"

void sub_D16A(Regs *r); void sub_D199(Regs *r); void sub_DB47(Regs *r);
void sub_DB52(Regs *r); void sub_DB66(Regs *r); void sub_DB7B(Regs *r);
void sub_DBB7(Regs *r); void sub_DB9B(Regs *r);
void sub_CA36(Regs *r); void sub_D620(Regs *r);

void sub_DA86(Regs *r)
{
    u8 n = (u8)(r->a - 0x02);            /* SEC / SBC #$02 */
    RAM8(0x04A1) = 0x00;                 /* STA $04A1 (A preserved via PHA/PLA) */

    if (n >= 0x18) {                     /* CMP #$18 / BCS -> L_DB01 */
        RAM8(0x8F) = 0x06;
        return;
    }
    if (n < 0x08) {                      /* CMP #$08 / BCC L_DA97-table */
        /* $DB16 table -> $0C:$0D, then JMP ($000C). The handler inherits these
         * scratch bytes, so replicate the stores even though we call directly. */
        static const u16 tbl[8] = { 0xD16A, 0xD199, 0xDB47, 0xDB52,
                                    0xDB66, 0xDB7B, 0xDBB7, 0xDB9B };
        RAM8(0x0C) = (u8)(tbl[n] & 0xFF);
        RAM8(0x0D) = (u8)(tbl[n] >> 8);
        r->a = (u8)(n << 1);             /* ASL A */
        r->x = r->a;                     /* TAX */
        switch (n) {                     /* JMP ($000C) */
            case 0: sub_D16A(r); break;
            case 1: sub_D199(r); break;
            case 2: sub_DB47(r); break;
            case 3: sub_DB52(r); break;
            case 4: sub_DB66(r); break;
            case 5: sub_DB7B(r); break;
            case 6: sub_DBB7(r); break;
            case 7: sub_DB9B(r); break;
        }
        return;
    }

    /* L_DAD2 — inventory-add path (n in [8,$18); carry set, so SBC #$08 = n-8) */
    {
        u8 x = (u8)(n - 0x08);
        if (RAM8((u16)(0x60 + x)) >= 0x0B) {   /* CMP #$0B / BCS L_DAEC */
            RAM8(0x8F) = 0x1D;
            return;
        }
        RAM8((u16)(0x60 + x))++;         /* INC inventory_counts,X */
        RAM8(0x8F) = 0x13;
        if (x == 0x0E) {                 /* CPX #$0E / BEQ L_DAE6 */
            sub_CA36(r);
            sub_D620(r);                 /* JMP L_D620 */
        }
    }
}
