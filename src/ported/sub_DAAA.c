/* $DAAA — item-action dispatcher (with OAM-clear prologue; X = active slot).
 *   n = A - 2; if n >= $18 -> RTS.
 *   L_DAB2: $0401,X = 0; $0406,X = $F0 (X = entry slot);
 *           X = ($08 << 3) | $80; $0200,X = $0204,X = $EF (hide OAM tiles).
 *   n < 8  -> JMP ($DB06 + n*2): {DB26,DB31,DB3C,DB52,DB5D,DB71,DBB7,DB85}[n]
 *   else   -> inventory-add path (shared L_DAD2, identical to DA86's).
 * JMP ($000C) tail-dispatches (handler RTS returns to DAAA's caller). */
#include "ram.h"
#include "regs.h"

void sub_DB26(Regs *r); void sub_DB31(Regs *r); void sub_DB3C(Regs *r);
void sub_DB52(Regs *r); void sub_DB5D(Regs *r); void sub_DB71(Regs *r);
void sub_DBB7(Regs *r); void sub_DB85(Regs *r);
void sub_CA36(Regs *r); void sub_D620(Regs *r);

void sub_DAAA(Regs *r)
{
    u8 n = (u8)(r->a - 0x02);            /* SEC / SBC #$02 */
    if (n >= 0x18)                       /* CMP #$18 / BCC L_DAB2; else RTS */
        return;

    /* L_DAB2 — clear this slot's sprite and hide its OAM tiles */
    {
        u8 slot = r->x;                  /* X = active slot */
        RAM8((u16)(0x0401 + slot)) = 0x00;
        RAM8((u16)(0x0406 + slot)) = 0xF0;
    }
    {
        u8 oam = (u8)((RAM8(0x08) << 3) | 0x80);  /* LDA $08 / ASL x3 / ORA #$80 / TAX */
        RAM8((u16)(0x0200 + oam)) = 0xEF;
        RAM8((u16)(0x0204 + oam)) = 0xEF;
        r->x = oam;
    }

    if (n < 0x08) {                      /* PLA / CMP #$08 / BCC L_DAF2 */
        /* $DB06 table -> $0C:$0D, then JMP ($000C). */
        static const u16 tbl[8] = { 0xDB26, 0xDB31, 0xDB3C, 0xDB52,
                                    0xDB5D, 0xDB71, 0xDBB7, 0xDB85 };
        RAM8(0x0C) = (u8)(tbl[n] & 0xFF);
        RAM8(0x0D) = (u8)(tbl[n] >> 8);
        r->a = (u8)(n << 1);             /* L_DAF2: ASL A */
        r->x = r->a;                     /* TAX */
        switch (n) {                     /* JMP ($000C) */
            case 0: sub_DB26(r); break;
            case 1: sub_DB31(r); break;
            case 2: sub_DB3C(r); break;
            case 3: sub_DB52(r); break;
            case 4: sub_DB5D(r); break;
            case 5: sub_DB71(r); break;
            case 6: sub_DBB7(r); break;
            case 7: sub_DB85(r); break;
        }
        return;
    }

    /* L_DAD2 — inventory-add path (n in [8,$18); carry set, SBC #$08 = n-8) */
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
