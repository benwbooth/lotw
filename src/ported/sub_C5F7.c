/* $C5F7: render a column/strip of nametable + attribute data to the PPU.
 * Reads tile indices through pointer $0C/$0D, looks up 4-byte metatiles through
 * pointer $79/$7A, and computes attribute bytes. All graphics go to
 * PPUDATA/PPUADDR.
 *
 * Stack discipline (net RAM): $23 ($29) $24 pushed at top, $0C/$0D pushed too.
 * $0C/$0D popped (restored) before the second phase, then re-modified there.
 * $23, $24, $29 restored at the end. Non-restored RAM outputs: $08, $0A, $0B,
 * $0C, $0D, vram_dst_lo($16), vram_dst_hi($17).
 *
 * Note: $0A is decremented to 0 and loops exit; final $0A=0. $0B ends at 0.
 */
#include "ram.h"
#include "regs.h"

#define M(a) RAM8(a)

void sub_C5F7(Regs *r)
{
    u8 ctrl_save = M(0x23);
    u8 v29_save  = M(0x29);
    u8 v24_save  = M(0x24);
    u8 c0c_save  = M(0x0C);
    u8 c0d_save  = M(0x0D);
    u16 p0C, p79;
    int outer;

    REG_W(0x2000, (ctrl_save & 0x7F) | 0x04);  /* PPUCTRL */
    M(0x29) = 0x00;
    REG_W(0x2001, v24_save & 0xE7);             /* PPUMASK */

    p79 = (u16)(M(0x79) | (M(0x7A) << 8));

    /* vram_dst from scroll_x_tile */
    {
        u8 sx = M(0x7C);
        u8 lo = (u8)((sx << 1) & 0x1C);
        u8 hi = (u8)((sx & 0x10) >> 2);
        u16 t = (u16)(0x00 + lo);            /* CLC; LDA #$00; ADC lo */
        M(0x16) = (u8)t;
        M(0x17) = (u8)(0x20 + hi + (t >> 8));
    }

    /* ---- Phase 1: $12 outer iterations ---- */
    M(0x0A) = 0x12;
    p0C = (u16)(c0c_save | (c0d_save << 8));
    for (outer = 0; outer < 0x12; outer++) {
        u8 inner;
        /* first half: 12 cols, low bytes of metatile pair */
        M(0x0B) = 0x0C;
        REG_W(0x2006, M(0x17));
        REG_W(0x2006, M(0x16));
        M(0x08) = 0x00;
        do {
            u8 idx = M((u16)(p0C + M(0x08)));
            u8 y = (u8)(idx << 2);              /* ASL A / ASL A / TAY (8-bit Y) */
            REG_W(0x2007, M((u16)(p79 + y)));
            REG_W(0x2007, M((u16)(p79 + (u8)(y + 1))));   /* INY */
            M(0x08)++;
            M(0x0B)--;
        } while (M(0x0B) != 0);

        /* second half: 12 cols, high bytes (offset +2) */
        M(0x0B) = 0x0C;
        REG_W(0x2006, M(0x17));
        inner = (u8)(M(0x16) + 1);           /* LDY vram_dst_lo; INY */
        REG_W(0x2006, inner);
        M(0x08) = 0x00;
        do {
            u8 idx = M((u16)(p0C + M(0x08)));
            u8 y = (u8)((idx << 2) + 2);        /* ASL A / ASL A / TAY / INY / INY */
            REG_W(0x2007, M((u16)(p79 + y)));
            REG_W(0x2007, M((u16)(p79 + (u8)(y + 1))));   /* INY */
            M(0x08)++;
            M(0x0B)--;
        } while (M(0x0B) != 0);

        /* advance vram_dst_lo by 2, wrap across nametable */
        M(0x16) += 2;
        if (M(0x16) & 0x20) {
            M(0x16) = 0x00;
            M(0x17) ^= 0x04;
        }

        /* advance pointer $0C/$0D by 12 */
        {
            u16 t = (u16)(0x0C + M(0x0C));
            M(0x0C) = (u8)t;
            M(0x0D) = (u8)(M(0x0D) + (t >> 8));
            p0C = (u16)(M(0x0C) | (M(0x0D) << 8));
        }
        M(0x0A)--;
    }

    /* restore $0C/$0D from stack (PLA STA $0D / PLA STA $0C) */
    M(0x0D) = c0d_save;
    M(0x0C) = c0c_save;
    p0C = (u16)(c0c_save | (c0d_save << 8));

    /* ---- Phase 2: attribute table, $09 outer iterations ---- */
    {
        u8 sx = M(0x7C);
        u8 lo = (u8)((sx >> 1) & 0x07);
        u8 hi = (u8)((sx & 0x10) >> 2);
        u16 t = (u16)(0xC0 + lo);            /* CLC; LDA #$C0; ADC lo */
        M(0x16) = (u8)t;
        M(0x17) = (u8)(0x23 + hi + (t >> 8));
    }
    M(0x0A) = 0x09;

    for (;;) {                                 /* L_C6D8 */
        int x;
        for (x = 6; x > 0; x--) {              /* L_C6DA, LDX #$06 */
            u8 a;
            /* build attribute byte in $08 via 8 ROLs of bit7 from 4 tiles */
            /* Y=$0D */
            a = M((u16)(p0C + 0x0D));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            /* Y=$01 */
            a = M((u16)(p0C + 0x01));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            /* Y=$0C */
            a = M((u16)(p0C + 0x0C));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            /* Y=$00 */
            a = M((u16)(p0C + 0x00));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }

            REG_W(0x2006, M(0x17));
            REG_W(0x2006, M(0x16));
            REG_W(0x2007, M(0x08));

            /* $0C += 2 */
            { u16 t = (u16)(0x02 + M(0x0C)); M(0x0C) = (u8)t; M(0x0D) = (u8)(M(0x0D) + (t >> 8)); }
            /* vram_dst_lo += 8 */
            { u16 t = (u16)(0x08 + M(0x16)); M(0x16) = (u8)t; M(0x17) = (u8)(M(0x17) + (t >> 8)); }
            p0C = (u16)(M(0x0C) | (M(0x0D) << 8));
        }
        /* $0C += 12 */
        { u16 t = (u16)(0x0C + M(0x0C)); M(0x0C) = (u8)t; M(0x0D) = (u8)(M(0x0D) + (t >> 8)); }
        /* vram_dst += $FFD1 (i.e. -47) */
        { u16 t = (u16)(0xD1 + M(0x16)); M(0x16) = (u8)t; M(0x17) = (u8)(M(0x17) + 0xFF + (t >> 8)); }
        p0C = (u16)(M(0x0C) | (M(0x0D) << 8));

        if (M(0x16) & 0x08) {
            M(0x16) = 0xC0;
            M(0x17) ^= 0x04;
        }
        M(0x0A)--;
        if (M(0x0A) == 0) break;
    }

    /* restore from stack: PLA $24, PLA $29, PLA ppuctrl_shadow */
    M(0x24) = v24_save;
    M(0x29) = v29_save;
    M(0x23) = ctrl_save;
    REG_W(0x2000, ctrl_save);

    r->a = ctrl_save;
    r->x = 0;     /* DEX loop left X=0 */
}
