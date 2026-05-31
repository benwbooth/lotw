/* $B577 (bank13, far-call target) — password DECODE + VALIDATE (inverse of B4D4).
 *   copy encoded cells $0322-$0341 -> work area $0342-$0361
 *   seed rng from $0351/$0361, XOR-decrypt every cell (rng_update @ $CC64)
 *   reconstruct the two checksums (bit0s of even cells) into $038A/$0389
 *   verify: sum of $0342 cells == $0389, and $0A-EOR-fold == $038A; else FAIL
 *   on success unpack: save_keys / save_gold (odd-cell bit0s), the 8 stat bytes
 *     $0308-$030F (nibble pairs of $0342), save_inventory_counts ($0352)
 *   returns CLC (valid) / SEC (bad password: $8F=$90=$1C).
 * The LSR-memory steps shift the work cells in place, so they run in order.
 * Pure data + RNG; far-call wrapper only switches banks (no-op in flat mem). */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

void sub_B577(Regs *r)
{
    int x, y;

    for (x = 0x1F; x >= 0; x--)            /* L_B579: copy cells */
        RAM8((u16)(0x0342 + x)) = RAM8((u16)(0x0322 + x));

    RAM8(0x3A) = RAM8(0x0351);             /* seed rng_s1/rng_s2 */
    RAM8(0x3B) = RAM8(0x0361);

    for (x = 0x0E; x >= 0; x--) {          /* L_B58E: XOR-decrypt pairs */
        RAM8(0x08) = (u8)x;
        r->a = 0x20; rng_update(r); x = RAM8(0x08);
        RAM8((u16)(0x0342 + x)) ^= r->a;
        r->a = 0x20; rng_update(r); x = RAM8(0x08);
        RAM8((u16)(0x0352 + x)) ^= r->a;
    }

    /* L_B5AF: gather bit0 of $0352 even cells -> $038A (LSR mem / ROR A) */
    { u8 a = 0; for (x = 0x0E; x >= 0; x -= 2) {
        u8 c = RAM8((u16)(0x0352 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        RAM8((u16)(0x0352 + x)) = (u8)(c >> 1);
    } RAM8(0x038A) = a; }
    /* L_B5BC: gather bit0 of $0342 even cells -> $0389 */
    { u8 a = 0; for (x = 0x0E; x >= 0; x -= 2) {
        u8 c = RAM8((u16)(0x0342 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        RAM8((u16)(0x0342 + x)) = (u8)(c >> 1);
    } RAM8(0x0389) = a; }

    /* L_B5CB: checksum 1 — sum of all $0342 cells == $0389 */
    { u8 a = 0; for (x = 0x1F; x >= 0; x--) a = (u8)(a + RAM8((u16)(0x0342 + x)));
      if (a != RAM8(0x0389)) goto fail; }
    /* L_B5DE: checksum 2 — $0A EOR-folded == $038A */
    { u8 a = 0x0A; for (x = 0x1F; x >= 0; x--) a ^= RAM8((u16)(0x0342 + x));
      if (a != RAM8(0x038A)) goto fail; }

    /* L_B5EE: save_keys from $0342 odd-cell bit0s */
    { u8 a = 0; for (x = 0x0F; x >= 0; x -= 2) {
        u8 c = RAM8((u16)(0x0342 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        RAM8((u16)(0x0342 + x)) = (u8)(c >> 1);
    } RAM8(0x0320) = a; }
    /* L_B5FB: save_gold from $0352 odd-cell bit0s */
    { u8 a = 0; for (x = 0x0F; x >= 0; x -= 2) {
        u8 c = RAM8((u16)(0x0352 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        RAM8((u16)(0x0352 + x)) = (u8)(c >> 1);
    } RAM8(0x0321) = a; }

    /* L_B60A: repack nibble pairs $0342 -> stat bytes $0308-$030F */
    x = 0x0F; y = 0x07;
    do {
        u8 hi = RAM8((u16)(0x0342 + x)); x--;     /* ASL A x4 */
        u8 lo = RAM8((u16)(0x0342 + x)); x--;     /* ORA $0342,X */
        RAM8((u16)(0x0308 + y)) = (u8)((hi << 4) | lo);
        y--;
    } while (y >= 0);
    /* L_B61E: $0352 -> save_inventory_counts */
    for (x = 0x0F; x >= 0; x--)
        RAM8((u16)(0x0310 + x)) = RAM8((u16)(0x0352 + x));

    r->c = 0;                              /* CLC — valid */
    return;
fail:                                      /* L_B629 */
    RAM8(0x8F) = 0x1C;
    RAM8(0x90) = 0x1C;
    r->c = 1;                              /* SEC — bad password */
}
