/* $B1EA:  JSR L_B2FC / LDY #$00 / loop reading ($0C),Y, packing nibbles into
 *         $0140,Y until a terminator:
 *           byte == 0    -> SEC / RTS                 (carry set)
 *           byte == $0D  -> JSR L_B24E / LDA #$05 / JSR L_B278 / CLC / RTS
 *           else         -> $08 = byte&$0F;
 *                           $0140,Y = ((byte&$F0)<<1) | $08;  INY; continue
 */
#include "ram.h"
#include "regs.h"

void sub_B2FC(Regs *r);
void sub_B24E(Regs *r);
void sub_B278(Regs *r);

void sub_B1EA(Regs *r)
{
    u16 ptr;
    u8 y;

    sub_B2FC(r);

    ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    /* The asm loop (INY / JMP, no bound) never terminates unless a 0x00 or
     * 0x0D byte appears within the 256-byte (Y-wrapped) window; the oracle
     * skips such inputs (max_steps). Cap at 256 so the harness never hangs —
     * skipped states are not compared. */
    {
        int i;
        for (i = 0, y = 0; i < 256; ++i, ++y) {
            u8 b = RAM8((u16)(ptr + y));
            if (b == 0x00) {            /* L_B213 */
                r->c = 1;
                return;
            }
            if (b == 0x0D) {            /* L_B209 */
                sub_B24E(r);
                r->a = 0x05;
                sub_B278(r);
                r->c = 0;
                return;
            }
            RAM8(0x08) = b & 0x0F;
            RAM8((u16)(0x0140 + y)) = (u8)(((b & 0xF0) << 1) | RAM8(0x08));
        }
    }
}
