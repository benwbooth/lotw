/* $ED2A:  boss-state handler [4].
 *   LDA $F4 / BEQ L_ED31 / JMP L_EBD8            ; if dir!=0, tail-call sub_EBD8
 * L_ED31 (dir==0): try four step directions via sub_ED5D (A = 1,2,4,8); the
 *   first that returns carry set jumps to L_ED58 (set $F4=1). If none hit,
 *   read ($E7)+4 into boss_life ($F2) and clear $FC.
 * L_ED58: LDA #$01 / STA $F4 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_ED5D(Regs *r);
void sub_EBD8(Regs *r);

void sub_ED2A(Regs *r)
{
    if (RAM8(0xF4) != 0) {
        /* JMP L_EBD8 : tail-call */
        sub_EBD8(r);
        return;
    }

    /* L_ED31: four directional probes */
    r->a = 0x01; sub_ED5D(r); if (r->c) goto hit;
    r->a = 0x02; sub_ED5D(r); if (r->c) goto hit;
    r->a = 0x04; sub_ED5D(r); if (r->c) goto hit;
    r->a = 0x08; sub_ED5D(r); if (r->c) goto hit;

    {
        u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
        u8 v = RAM8((u16)(ptr + 4));
        RAM8(0x00F2) = v;          /* boss_life */
        r->a = 0x00;
        RAM8(0xFC) = 0x00;
    }
    return;

hit:
    /* L_ED58 */
    r->a = 0x01;
    RAM8(0xF4) = 0x01;
}
