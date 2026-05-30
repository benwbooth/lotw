/* $F23A: collision test. Builds pointer $0C/$0D from $0F (lo) and $0A (hi),
 * runs sub_CA54 to finalize the map pointer, then probes map bytes via sub_F2D3
 * at several Y offsets. Carry set on first solid hit (early-out), else CLC.
 *
 *   LDA $0F / STA $0C / LDA $0A / STA $0D / JSR L_CA54
 *   LDY #$00 / JSR L_F2D3 / BCS out
 *   LDA $0E / BEQ skip1 / LDY #$0C / JSR L_F2D3 / BCS out
 * skip1: LDA $0A / CMP #$B0 / BCS done / AND #$0F / BEQ done
 *   LDY #$01 / JSR L_F2D3 / BCS out / LDA $0E / BEQ done
 *   LDY #$0D / JSR L_F2D3 / BCS out
 * done: CLC
 * out:  RTS
 */
#include "ram.h"
#include "regs.h"

void sub_CA54(Regs *r);
void sub_F2D3(Regs *r);

void sub_F23A(Regs *r)
{
    RAM8(0x0C) = RAM8(0x0F);
    RAM8(0x0D) = RAM8(0x0A);
    sub_CA54(r);

    r->y = 0x00;
    sub_F2D3(r);
    if (r->c) return;

    if (RAM8(0x0E) != 0) {
        r->y = 0x0C;
        sub_F2D3(r);
        if (r->c) return;
    }

    /* L_F257 */
    if (RAM8(0x0A) >= 0xB0) { r->c = 0; return; }
    if ((RAM8(0x0A) & 0x0F) == 0) { r->c = 0; return; }

    r->y = 0x01;
    sub_F2D3(r);
    if (r->c) return;

    if (RAM8(0x0E) == 0) { r->c = 0; return; }

    r->y = 0x0D;
    sub_F2D3(r);
    if (r->c) return;

    r->c = 0;   /* L_F273: CLC */
}
