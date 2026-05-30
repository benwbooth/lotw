/* $CFBC:
 *   LDA #$47 / STA vram_dst_lo ($16)
 *   LDA #$22 / STA vram_dst_hi ($17)
 *   LDA scroll_x_tile ($7C) / AND #$10 / BEQ .skip
 *     CLC / LDA #$00 / ADC vram_dst_lo / STA vram_dst_lo
 *     LDA #$04 / ADC vram_dst_hi / STA vram_dst_hi   ; dst += $0400 (other nametable)
 *   .skip:
 *   LDA $81 / JSR L_CFF9 / LDA #$06 / JSR queue_ppu_job_and_wait
 *   CLC / LDA #$0E / ADC vram_dst_lo / STA vram_dst_lo
 *   LDA #$00 / ADC vram_dst_hi / STA vram_dst_hi      ; dst += $0E
 *   LDA $83 / JSR L_CFF9 / LDA #$06 / JSR queue_ppu_job_and_wait
 *   RTS
 * Draws two values ($81, $83) into the status area, selecting the nametable by
 * the $10 bit of scroll_x_tile.
 */
#include "ram.h"
#include "regs.h"

void sub_CFF9(Regs *r);
void queue_ppu_job_and_wait(Regs *r);

void sub_CFBC(Regs *r)
{
    u8 lo, hi, c;

    RAM8(0x16) = 0x47;                  /* vram_dst_lo */
    RAM8(0x17) = 0x22;                  /* vram_dst_hi */

    if (RAM8(0x7C) & 0x10) {            /* scroll_x_tile & $10 */
        /* CLC; lo = $00 + lo; hi = $04 + hi + carry */
        u16 s = (u16)(0x00 + RAM8(0x16));
        RAM8(0x16) = (u8)s;
        RAM8(0x17) = (u8)(0x04 + RAM8(0x17) + (s >> 8));
    }

    r->a = RAM8(0x81);
    sub_CFF9(r);
    r->a = 0x06;
    queue_ppu_job_and_wait(r);

    /* CLC; lo = $0E + lo; hi = $00 + hi + carry */
    lo = RAM8(0x16);
    c = (u8)((0x0E + lo) >> 8);
    RAM8(0x16) = (u8)(0x0E + lo);
    hi = RAM8(0x17);
    RAM8(0x17) = (u8)(0x00 + hi + c);

    r->a = RAM8(0x83);
    sub_CFF9(r);
    r->a = 0x06;
    queue_ppu_job_and_wait(r);
}
