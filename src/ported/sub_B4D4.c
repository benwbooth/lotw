/* $B4D4 (bank13, far-call target $0C0D) — password ENCODE.
 * Packs the save state into 32 nibble cells ($0322-$0341), folds in two
 * checksums, then XOR-encrypts every cell with the RNG (rng_update @ $CC64).
 *   - $0322-$0331 <- the 8 stat bytes $0308-$030F split hi/lo nibble
 *   - $0332-$0341 <- low nibble of save_inventory_counts ($0310+X)
 *   - save_keys bits shifted into bit0 of the odd $0322 cells (LSR/ROL chain)
 *   - save_gold bits shifted into bit0 of the odd $0332 cells
 *   - $0389 = sum of $0322-$0341; $038A = $0A EOR-folded over them
 *   - those two checksums shifted into the even cells
 *   - rng seeded from $0331/$0341, then each cell ^= rng_update(#$20)
 * Pure data + RNG; the far-call wrapper only switches banks (no-op in flat mem). */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

void sub_B4D4(Regs *r)
{
    int x, y;

    /* L_B4D8: unpack $0308-$030F into $0322-$0331 (hi nibble then lo) */
    x = 0x0F; y = 0x07;
    do {
        u8 b = RAM8((u16)(0x0308 + y));
        RAM8((u16)(0x0322 + x)) = (u8)(b >> 4);   /* LSR x4 */
        x--;
        RAM8((u16)(0x0322 + x)) = (u8)(b & 0x0F); /* AND #$0F */
        x--; y--;
    } while (y >= 0);                              /* DEY / BPL */

    /* L_B4F1: $0332-$0341 = low nibble of save_inventory_counts */
    for (x = 0x0F; x >= 0; x--)
        RAM8((u16)(0x0332 + x)) = (u8)(RAM8((u16)(0x0310 + x)) & 0x0F);

    /* L_B501: shift save_keys bits into bit0 of $0322 odd cells (LSR A / ROL cell) */
    {
        u8 a = RAM8(0x0320);                       /* save_keys */
        for (x = 0x0F; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;          /* LSR A -> carry */
            u8 c = RAM8((u16)(0x0322 + x));
            RAM8((u16)(0x0322 + x)) = (u8)((c << 1) | cin);  /* ROL (bit7 lost via next LSR) */
        }
    }
    /* L_B50E: same for save_gold into $0332 odd cells */
    {
        u8 a = RAM8(0x0321);                       /* save_gold */
        for (x = 0x0F; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = RAM8((u16)(0x0332 + x));
            RAM8((u16)(0x0332 + x)) = (u8)((c << 1) | cin);
        }
    }

    /* L_B51A: $0389 = sum of all 32 cells $0322-$0341 */
    {
        u8 a = 0x00;
        for (x = 0x1F; x >= 0; x--)
            a = (u8)(a + RAM8((u16)(0x0322 + x)));  /* CLC/ADC (carry-out discarded each step) */
        RAM8(0x0389) = a;
    }
    /* L_B528: $038A = $0A EOR-folded over all 32 cells */
    {
        u8 a = 0x0A;
        for (x = 0x1F; x >= 0; x--)
            a = (u8)(a ^ RAM8((u16)(0x0322 + x)));
        RAM8(0x038A) = a;
    }

    /* L_B536: shift $0389 bits into $0322 even cells ($0F..$01 step 2 over X=$0E..) */
    {
        u8 a = RAM8(0x0389);
        for (x = 0x0E; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = RAM8((u16)(0x0322 + x));
            RAM8((u16)(0x0322 + x)) = (u8)((c << 1) | cin);
        }
    }
    /* L_B543: shift $038A bits into $0332 even cells */
    {
        u8 a = RAM8(0x038A);
        for (x = 0x0E; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = RAM8((u16)(0x0332 + x));
            RAM8((u16)(0x0332 + x)) = (u8)((c << 1) | cin);
        }
    }

    /* seed RNG from the last cells, then XOR-encrypt each pair of cells */
    RAM8(0x3A) = RAM8(0x0331);            /* $0331 -> rng_s1 */
    RAM8(0x3B) = RAM8(0x0341);            /* $0341 -> rng_s2 */
    for (x = 0x0E; x >= 0; x--) {         /* L_B557 */
        RAM8(0x08) = (u8)x;               /* STX $08 */
        r->a = 0x20; rng_update(r);       /* LDA #$20 / JSR $CC64 */
        x = RAM8(0x08);                   /* LDX $08 */
        RAM8((u16)(0x0322 + x)) = (u8)(r->a ^ RAM8((u16)(0x0322 + x)));
        r->a = 0x20; rng_update(r);
        x = RAM8(0x08);
        RAM8((u16)(0x0332 + x)) = (u8)(r->a ^ RAM8((u16)(0x0332 + x)));
    }
}
