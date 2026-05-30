/* $D0E5 — build a sprite-row VRAM buffer at $0362 from source values at $0322,
 * clamping each ((v|$80) >= $A0 -> $7F), then queue two PPU column updates.
 *   Y=$1F, X=$26; unrolled x4 per outer pass: out=$0322[Y]|$80; if >=$A0 out=$7F;
 *   $0362[X]=out; X--,Y--; ... after 4: X-- extra; BPL.
 * Then sets vram params and submits two jobs (type $05).
 * Net RAM: $0362.. buffer, $1A/$1B/$16-$19 vram params, $28=0 (queue). */
#include "ram.h"
#include "regs.h"

void queue_ppu_job_and_wait(Regs *r);

void sub_D0E5(Regs *r)
{
    int y = 0x1F;
    int x = 0x26;
    int i;

    do {
        for (i = 0; i < 4; ++i) {
            u8 out = (u8)(RAM8((u16)(0x0322 + y)) | 0x80);  /* ORA #$80 */
            if (out >= 0xA0)                                /* CMP #$A0 / BCC */
                out = 0x7F;
            RAM8((u16)(0x0362 + (x & 0xFF))) = out;         /* STA $0362,X */
            x = (x - 1) & 0xFF;                             /* DEX */
            y = (y - 1) & 0xFF;                             /* DEY */
        }
        x = (x - 1) & 0xFF;                                 /* extra DEX */
    } while ((x & 0x80) == 0);                              /* BPL */

    RAM8(0x1A) = 0x13;   /* vram_len */
    RAM8(0x1B) = 0x00;
    RAM8(0x16) = 0xE6;   /* vram_dst_lo */
    RAM8(0x17) = 0x24;   /* vram_dst_hi -> $24E6 */
    RAM8(0x18) = 0x62;   /* vram_src_lo */
    RAM8(0x19) = 0x03;   /* vram_src_hi -> $0362 */
    r->a = 0x05;
    queue_ppu_job_and_wait(r);

    RAM8(0x16) = 0x06;   /* vram_dst_lo */
    RAM8(0x17) = 0x25;   /* vram_dst_hi -> $2506 */
    RAM8(0x18) = 0x76;   /* vram_src_lo */
    RAM8(0x19) = 0x03;   /* vram_src_hi -> $0376 */
    r->a = 0x05;
    queue_ppu_job_and_wait(r);
}
