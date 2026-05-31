/* $A5E6 (bank13) — warp-web entry update loop. Walks the 4 warp/portal
 * descriptor slots through the ($E5) pointer (seeded $E5=$10,$E6=$04 ->
 * $0410, advanced +$10 each pass to $0440) and a 1-based slot index $E3.
 * For each slot it reads byte +1 of the descriptor: if non-zero the slot is
 * "active" -> L_A657 (advance/expire an in-flight warp), else it is "armed"
 * -> if the pad ($20 bit6) is held AND $FD bit6 is clear, L_A622 (launch a new
 * warp from the player position). After 4 slots, L_A6E0 finalises (rebuilds
 * the $8810 sprite table) and returns.
 *
 *   LDA #$01 / STA $E3            ; slot index = 1
 *   LDA #$10 / STA $E5            ; descriptor ptr lo
 *   LDA #$04 / STA $E6            ; descriptor ptr hi ($0410)
 * L_A5F2:
 *   LDY #$01 / LDA ($E5),Y / BNE L_A606   ; +1 byte != 0 -> active
 *   BIT $20 / BVC L_A609         ; pad bit6 clear -> skip
 *   BIT $FD / BVS L_A609         ; $FD  bit6 set   -> skip
 *   JSR L_A622 / JMP L_A609      ; else launch
 * L_A606: JSR L_A657
 * L_A609:
 *   INC $E3
 *   CLC / LDA #$10 / ADC $E5 / STA $E5 / LDA #$00 / ADC $E6 / STA $E6  ; ptr += $10
 *   LDA $E3 / CMP #$04 / BCC L_A5F2
 *   JSR L_A6E0 / RTS
 *
 * Diff-tested: terminating, no wait-loop / PLA / frame-sync far-call. The
 * BIT $20 test reads the controller latch as plain RAM (deterministic). */
#include "ram.h"
#include "regs.h"

void sub_A622(Regs *r); void sub_A657(Regs *r); void sub_A6E0(Regs *r);

void sub_A5E6(Regs *r)
{
    RAM8(0xE3) = 0x01;                  /* slot index */
    RAM8(0xE5) = 0x10;                  /* descriptor ptr -> $0410 */
    RAM8(0xE6) = 0x04;

    do {                                /* L_A5F2 */
        u16 p = (u16)(RAM8(0xE5) | (RAM8(0xE6) << 8));
        if (RAM8((u16)(p + 1)) != 0) {  /* LDA ($E5),Y=1 / BNE */
            sub_A657(r);                /* L_A606: active slot */
        } else if ((RAM8(0x20) & 0x40) && !(RAM8(0xFD) & 0x40)) {
            sub_A622(r);                /* pad held & $FD clear -> launch */
        }
        /* L_A609: advance index and descriptor pointer by $10 */
        RAM8(0xE3) = (u8)(RAM8(0xE3) + 1);
        {
            u16 np = (u16)(RAM8(0xE5) + 0x10);
            RAM8(0xE5) = (u8)np;
            RAM8(0xE6) = (u8)(RAM8(0xE6) + (np >> 8));   /* carry into hi */
        }
    } while (RAM8(0xE3) < 0x04);        /* CMP #$04 / BCC L_A5F2 */

    sub_A6E0(r);                        /* finalise */
}
