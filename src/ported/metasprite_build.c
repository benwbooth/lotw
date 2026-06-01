/* $C871 metasprite_build — assemble a metasprite's OAM/tile data into the sprite
 * buffers ($0140/$0141/$0158/$0159 tiles, $0170/$0171 positions+attrs) from the
 * metasprite definition pointed to by ($0C) and the tile table ($79), then queue
 * a PPU job (type $03).
 *
 * Phase 1 (X=$16..0 step -2, $0B=0..11): for each entry e=($0C)[$0B], read 4
 *   consecutive bytes from ($79)[e*4 .. e*4+3] into $0141,X / $0140,X / $0159,X /
 *   $0158,X.
 * Phase 2: vram_src_hi = vram_dst_hi+3; $0B = (vram_dst_lo>>2)+$C0;
 *   vram_src_lo = (vram_dst_lo&2)? $33 : $CC; then X=$0A..0 step -2, Y=0..:
 *     $0170,X = $0B; $0B += 8;
 *     b0=($0C)[Y++]; $0171,X = (b0&$C0)>>4;
 *     b1=($0C)[Y++]; $0171,X = (b1&$C0) | $0171,X;
 *     if (vram_dst_lo&2)==0: $0171,X >>= 2;
 * Net RAM: sprite buffers, $0B, $18/$19 vram_src, $28=0 (queue). */
#include "ram.h"
#include "regs.h"

void queue_ppu_job_and_wait(Regs *r);

void metasprite_build(Regs *r)
{
    u16 p0C = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));   /* metasprite def ptr */
    u16 p79 = (u16)(RAM8(0x79) | (RAM8(0x7A) << 8));   /* tile table ptr */
    int x, y;
    u8 dst_lo, mask2;

#ifdef LOTW_SHIM
    { extern int printf(const char*,...); static int n=0;
      if (p79 == 0xAD00 && n++ < 6) printf("[msb] p0C=$%04X p79=$%04X e0=$%02X ty=$%02X t0=$%02X t1=$%02X R6=%02X R7=%02X\n",
          p0C, p79, RAM8(p0C), (u8)(RAM8(p0C)<<2),
          RAM8((u16)(p79+((RAM8(p0C)<<2)&0xFF))), RAM8((u16)(p79+(((RAM8(p0C)<<2)+1)&0xFF))),
          RAM8(0x30), RAM8(0x31)); }
#endif
    RAM8(0x0B) = 0x00;
    for (x = 0x16; x >= 0; x -= 2) {
        u8 e = RAM8((u16)(p0C + RAM8(0x0B)));   /* ($0C)[$0B] */
        u16 ty = (u16)((u8)(e << 2));           /* ASL ASL / TAY (8-bit) */
        RAM8((u16)(0x0141 + x)) = RAM8((u16)(p79 + ((ty + 0) & 0xFF)));
        RAM8((u16)(0x0140 + x)) = RAM8((u16)(p79 + ((ty + 1) & 0xFF)));
        RAM8((u16)(0x0159 + x)) = RAM8((u16)(p79 + ((ty + 2) & 0xFF)));
        RAM8((u16)(0x0158 + x)) = RAM8((u16)(p79 + ((ty + 3) & 0xFF)));
        RAM8(0x0B) += 1;
    }

    RAM8(0x19) = (u8)(RAM8(0x17) + 0x03);   /* vram_src_hi = vram_dst_hi + 3 */
    dst_lo = RAM8(0x16);
    RAM8(0x0B) = (u8)((dst_lo >> 2) + 0xC0); /* LSR LSR / ADC #$C0 */
    mask2 = (u8)(dst_lo & 0x02);
    RAM8(0x18) = mask2 ? 0x33 : 0xCC;        /* vram_src_lo */

    y = 0x00;
    for (x = 0x0A; x >= 0; x -= 2) {
        u8 b0, b1, v;
        RAM8((u16)(0x0170 + x)) = RAM8(0x0B);   /* STA $0170,X */
        RAM8(0x0B) = (u8)(RAM8(0x0B) + 0x08);   /* ADC #$08 */

        b0 = RAM8((u16)(p0C + (y++)));          /* ($0C),Y / INY */
        v = (u8)((b0 & 0xC0) >> 4);             /* AND #$C0 / LSR x4 */
        RAM8((u16)(0x0171 + x)) = v;

        b1 = RAM8((u16)(p0C + (y++)));          /* ($0C),Y / INY */
        v = (u8)((b1 & 0xC0) | RAM8((u16)(0x0171 + x))); /* AND #$C0 / ORA */
        RAM8((u16)(0x0171 + x)) = v;

        if (mask2 == 0) {                       /* BNE skip else LSR x2 */
            RAM8((u16)(0x0171 + x)) = (u8)(RAM8((u16)(0x0171 + x)) >> 1);
            RAM8((u16)(0x0171 + x)) = (u8)(RAM8((u16)(0x0171 + x)) >> 1);
        }
    }

    r->a = 0x03;
    queue_ppu_job_and_wait(r);
}
