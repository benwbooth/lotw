/* $AD3B — animate/flip the player metasprite. If the tile index $56 < $20,
 * fold the V-flag of buttons $20 into bit4 of $56 (set if BVS, else clear).
 * Then, while moving on the ground (low nibble of $20 nonzero and not climbing/
 * jumping $4E/$4F), advance the walk counter $4D; every 8th step toggle the
 * animation: if $56 bit3 set, flip $57 bit6 ($40), else flip $56 bit2 ($04).
 * Pure RAM/buttons, RTS. No callees. */
#include "ram.h"
#include "regs.h"

void sub_AD3B(Regs *r)
{
    if (RAM8(0x56) < 0x20) {                        /* LDA $56 / CMP #$20 / BCS L_AD50 */
        u8 a = RAM8(0x56);                          /* LDA $56 */
        if (RAM8(0x20) & 0x40)                      /* BIT $20 / BVS L_AD4C (V = bit6) */
            a = (u8)(a | 0x10);                     /* ORA #$10 */
        else
            a = (u8)(a & 0xEF);                     /* AND #$EF */
        RAM8(0x56) = a;                             /* STA $56 */
    }
    /* L_AD50 */
    if ((RAM8(0x20) & 0x0F) == 0) return;           /* AND #$0F / BEQ L_AD79 */
    if ((RAM8(0x4F) | RAM8(0x4E)) != 0) return;     /* ORA $4E / BNE L_AD79 */
    RAM8(0x4D) = (u8)(RAM8(0x4D) + 1);              /* INC $4D */
    if ((RAM8(0x4D) & 0x07) != 0) return;           /* AND #$07 / BNE L_AD79 */
    if (RAM8(0x56) & 0x08) {                        /* AND #$08 / BNE L_AD73 */
        RAM8(0x57) ^= 0x40;                         /* L_AD73: EOR #$40 */
    } else {
        RAM8(0x56) ^= 0x04;                         /* EOR #$04 */
    }
}
