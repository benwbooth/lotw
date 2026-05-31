/* $A5B5 — palette/attribute fade-darken loop over the 13-byte block $0180-$018C.
 * For 4 passes: set PPU-job count $36=5, then for each byte $0180+X (X=$0C..0):
 * split into lo nibble ($08) and hi nibble; subtract $10 from the hi nibble
 * (SEC/SBC #$10). If it underflows (BCC) the byte saturates to #$0F, otherwise
 * recombine hi|lo. Then far-frame-sync via $C135. Bounded 4-iteration loop.
 * C135 frame-syncs on $28/$36 (sync_clear). */
#include "ram.h"
#include "regs.h"

void sub_C135(Regs *r);

void sub_A5B5(Regs *r)
{
    u8 y = 0x04;
    do {                                   /* L_A5B7 */
        RAM8(0x36) = 0x05;                 /* LDA #$05 / STA $36 */
        for (int x = 0x0C; x >= 0; x--) {  /* L_A5BF: LDX #$0C ... DEX/BPL */
            u8 lo = (u8)(RAM8((u16)(0x0180 + x)) & 0x0F);   /* AND #$0F / STA $08 */
            RAM8(0x08) = lo;
            u8 hi = (u8)(RAM8((u16)(0x0180 + x)) & 0xF0);
            u8 out;
            if ((int)hi - 0x10 < 0)        /* SEC/SBC #$10 -> BCC (no carry) */
                out = 0x0F;                /* LDA #$0F */
            else
                out = (u8)((hi - 0x10) | lo);  /* ORA $08 */
            RAM8((u16)(0x0180 + x)) = out;
        }
        sub_C135(r);                       /* JSR $C135 */
        y--;                               /* PLA/TAY/DEY */
    } while (y != 0);                      /* BNE L_A5B7 */
}
