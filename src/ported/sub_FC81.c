/* $FC81: sound — fetch note byte via ptr ($95+$02), advance ptr (inc16_95),
 * look up a 16-bit period in note_period_table ($FDB1)/$FDB2 by (note&$0F)*2,
 * subtract the per-channel detune $A1,X, then shift right by (note>>4).
 *   LDX $02 / LDA ($95,X) / JSR L_FD6B / TAY / AND #$0F / ASL A / TAX
 *   LDA $FDB1,X / STA $04 / LDA $FDB2,X / STA $05
 *   LDX $02 / LDA $04 / SEC / SBC $A1,X / STA $04 / BCS + / DEC $05
 *   + TYA / LSR x4 / BEQ done / TAY / loop: LSR $05 / ROR $04 / DEY / BNE loop / RTS
 */
#include "ram.h"
#include "regs.h"

void inc16_95(Regs *r);

void sub_FC81(Regs *r)
{
    u8 x = RAM8(0x02);
    u16 ptr = (u16)(RAM8((u8)(0x95 + x)) | (RAM8((u8)(0x96 + x)) << 8));
    u8 note = RAM8(ptr);                 /* LDA ($95,X) */

    inc16_95(r);                         /* advances ($95+$02):($96+$02) */

    {
        u8 y = note;                     /* TAY */
        u8 idx = (u8)((note & 0x0F) << 1);   /* AND #$0F / ASL / TAX */
        u8 lo = RAM8((u16)(0xFDB1 + idx));
        u8 hi = RAM8((u16)(0xFDB2 + idx));

        x = RAM8(0x02);
        {
            u16 sub = (u16)((u16)lo - RAM8((u8)(0xA1 + x)));  /* SEC / SBC */
            lo = (u8)sub;
            if (sub & 0x100)             /* borrow -> carry clear -> DEC $05 */
                hi = (u8)(hi - 1);
        }

        {
            u8 cnt = (u8)(y >> 4);       /* TYA / LSR x4 */
            while (cnt != 0) {           /* BEQ done; else TAY/loop */
                u8 newcarry = (u8)(hi & 1);
                hi = (u8)(hi >> 1);
                lo = (u8)((lo >> 1) | (newcarry << 7));
                --cnt;
            }
        }

        RAM8(0x04) = lo;
        RAM8(0x05) = hi;
    }
}
