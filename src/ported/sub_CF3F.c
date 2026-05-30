/* $CF3F:
 *   TXA / PHA                         ; save X
 *   TXA / AND #$07 / ASL / ASL        ; (X&7)<<2
 *   STA vram_dst_lo ($16)
 *   TXA / AND #$08 / ASL / ASL / ASL / ASL  ; (X&8)<<4  -> bit3 to bit7
 *   ORA vram_dst_lo / STA vram_dst_lo
 *   LDA #$00 / STA vram_dst_hi ($17)
 *   CLC / LDA #$C2 / ADC vram_dst_lo / STA vram_dst_lo
 *   LDA #$20 / ADC vram_dst_hi / STA vram_dst_hi      ; dst = $20C2 + lo
 *   TYA / JSR L_CFF9                  ; split Y into vram_src ($18/$19)
 *   PLA / JSR L_D017                  ; A=orig X -> bit mask; carry = last bit out
 *   BCS L_CF7C
 *     LDA vram_src_lo / SEC / SBC #$40 / STA vram_src_lo
 *     LDA vram_src_hi / SEC / SBC #$40 / STA vram_src_hi   ; src -= $4040
 *   L_CF7C:
 *   LDA #$06 / JSR queue_ppu_job_and_wait
 *   RTS
 * Inputs: X (cell index), Y (digit value). Builds VRAM dst/src and submits a job.
 */
#include "ram.h"
#include "regs.h"

void sub_CFF9(Regs *r);
void sub_D017(Regs *r);
void queue_ppu_job_and_wait(Regs *r);

void sub_CF3F(Regs *r)
{
    u8 x = r->x;
    u8 lo, hi;
    u16 s;

    lo = (u8)((x & 0x07) << 2);
    lo = (u8)(((x & 0x08) << 4) | lo);     /* (x&8)<<4 OR lo */
    hi = 0x00;
    /* CLC; lo = $C2 + lo; hi = $20 + hi + carry */
    s = (u16)(0xC2 + lo);
    RAM8(0x16) = (u8)s;
    RAM8(0x17) = (u8)(0x20 + hi + (s >> 8));

    /* TYA / JSR L_CFF9  -> A = Y */
    r->a = r->y;
    sub_CFF9(r);

    /* PLA / JSR L_D017  -> A = bit mask. sub_D017 sets r->a but not the carry
     * the original leaves (last ASL's bit-out), which the BCS below tests, so
     * recompute that carry from the same inputs. */
    {
        u8 in = x;
        u8 dx = (u8)(RAM8(0x40) << 1);          /* cur_character*2 */
        u8 yy, carry, v;
        if (in >= 0x08) dx++;
        yy = (u8)((in & 0x07) + 1);
        v = RAM8((u16)(0xFFBB + dx));
        carry = 0;
        do {
            carry = (u8)(v >> 7);               /* bit shifted out by ASL */
            v = (u8)(v << 1);
        } while (--yy != 0);
        r->c = carry;
    }
    r->a = x;
    sub_D017(r);                                /* sets r->a = bit mask */

    if (!r->c) {                                /* BCS skips this when carry set */
        RAM8(0x18) = (u8)(RAM8(0x18) - 0x40);   /* vram_src_lo -= $40 (SEC/SBC) */
        RAM8(0x19) = (u8)(RAM8(0x19) - 0x40);   /* vram_src_hi -= $40 */
    }

    r->a = 0x06;
    queue_ppu_job_and_wait(r);
}
