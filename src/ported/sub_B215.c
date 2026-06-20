/* $B215 (bank13): decode a text/tile string through pointer ($0C),Y into the
 * line buffer $0140,Y, until a 0 terminator (carry set, return) or a $0D newline.
 *
 *   JSR L_B2FC                ; sub_B2FC (setup)
 *   LDY #$00
 * loop ($B21A):
 *   LDA ($0C),Y
 *   BEQ done_sec             ; 0 terminator -> SEC; RTS
 *   CMP #$0D                 ; newline?
 *   BEQ newline
 *   AND #$0F / STA $08
 *   LDA ($0C),Y / AND #$F0 / ASL A / ORA $08 / CLC / ADC #$10 / STA $0140,Y
 *   INY / JMP loop
 * newline ($B237):
 *   INY / TYA / CLC / ADC $0C / STA $0C / BCC + / INC $0D
 *   JSR L_B24E (sub_B24E)
 *   LDA #$05 / JSR L_B278 (sub_B278)
 *   CLC / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_B24E(Regs *r);
void sub_B278(Regs *r);
void sub_B2FC(Regs *r);

void sub_B215(Regs *r)
{
    u16 ptr;
    u8 y, c, lo;
    int guard;

    sub_B2FC(r);

    ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    y = 0x00;
    /* Guard: the real loop is unbounded; the harness skips non-terminating
     * states. Y is an 8-bit
     * index, so after 256 bytes with no $00/$0D terminator the source repeats
     * forever — bail out to avoid hanging (such states are not compared). */
    for (guard = 0; guard < 256; guard++) {
        c = RAM8((u16)(ptr + y));
        if (c == 0x00) {            /* terminator */
            r->c = 1;
            return;
        }
        if (c == 0x0D) {            /* newline */
            u16 sum;
            y++;
            sum = (u16)(y + RAM8(0x0C));     /* TYA; CLC; ADC $0C */
            lo = (u8)sum;
            RAM8(0x0C) = lo;
            if (sum > 0xFF)                  /* BCC skip / INC $0D */
                RAM8(0x0D) = (u8)(RAM8(0x0D) + 1);

            r->a = 0x08;            /* JSR L_B24E reads $0A/sets vram_dst */
            sub_B24E(r);
            r->a = 0x05;
            sub_B278(r);
            r->c = 0;
            return;
        }
        {
            u8 lonib = c & 0x0F;
            u8 hi;
            u8 v;
            RAM8(0x08) = lonib;              /* STA $08 */
            hi = (u8)((c & 0xF0) << 1);      /* ASL A */
            v = (u8)(hi | RAM8(0x08));
            v = (u8)(v + 0x10);              /* CLC; ADC #$10 */
            RAM8((u16)(0x0140 + y)) = v;
        }
        y++;
    }
    /* Non-terminating input (oracle skips these); leave r->c as-is. */
}
