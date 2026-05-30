/* $F275: wider collision test. Builds pointer $0C/$0D from $0F (lo) / $0A (hi),
 * runs sub_CA54, then probes map bytes via sub_F2D3 at many Y offsets. Carry set
 * on first solid hit (early-out), else CLC.
 *
 *   LDA $0F / STA $0C / LDA $0A / STA $0D / JSR L_CA54
 *   LDY #$00 / F2D3 / BCS out ; LDY #$01 / F2D3 / BCS out
 *   LDY #$0C / F2D3 / BCS out ; LDY #$0D / F2D3 / BCS out
 *   LDA $0E / BEQ skip ; LDY #$18 / F2D3 / BCS out ; LDY #$19 / F2D3 / BCS out
 * skip: LDA $0A / CMP #$B0 / BCS done / AND #$0F / BEQ done
 *   LDY #$02 / F2D3 / BCS out ; LDY #$0E / F2D3 / BCS out
 *   LDA $0E / BEQ done ; LDY #$1A / F2D3 / BCS out
 * done: CLC
 * out:  RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_F2D3(Regs *r);

static int probe(Regs *r, u8 y)
{
    r->y = y;
    sub_F2D3(r);
    return r->c;   /* nonzero => solid hit, caller returns */
}

void sub_F275(Regs *r)
{
    RAM8(0x0C) = RAM8(0x0F);
    RAM8(0x0D) = RAM8(0x0A);
    sub_CA54(r);

    if (probe(r, 0x00)) return;
    if (probe(r, 0x01)) return;
    if (probe(r, 0x0C)) return;
    if (probe(r, 0x0D)) return;

    if (RAM8(0x0E) != 0) {
        if (probe(r, 0x18)) return;
        if (probe(r, 0x19)) return;
    }

    /* L_F2AE */
    if (RAM8(0x0A) >= 0xB0) { r->c = 0; return; }
    if ((RAM8(0x0A) & 0x0F) == 0) { r->c = 0; return; }

    if (probe(r, 0x02)) return;
    if (probe(r, 0x0E)) return;

    if (RAM8(0x0E) == 0) { r->c = 0; return; }

    if (probe(r, 0x1A)) return;

    r->c = 0;   /* L_F2D1: CLC */
}
