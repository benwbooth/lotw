/* $B6A6 — palette fade-out loop. For $09 = $40,$30,$20,$10,$00: set the PPU-job
 * count $36=5, rebuild the sprite palette (B6F0), fade the BG palette one step
 * (B6D0 over $0180, threshold $09), commit a frame (far-call $C135 via CCE4),
 * then a final C569. Bounded 5-iteration loop.
 * The CCE4 far-call to $C135 restores the bank shadows ($30/$31 <- $32/$33). */
#include "ram.h"
#include "regs.h"
void sub_B6F0(Regs *r); void sub_B6D0(Regs *r); void sub_C135(Regs *r); void sub_C569(Regs *r);
void sub_B6A6(Regs *r)
{
    RAM8(0x09) = 0x40;                  /* LDA #$40 / STA $09 */
    do {                                /* L_B6AA */
        RAM8(0x36) = 0x05;              /* LDA #$05 / STA $36 */
        sub_B6F0(r);
        r->x = 0x00; r->y = 0x20;       /* LDX #$00 / LDY #$20 */
        sub_B6D0(r);
        /* JSR farcall_return_home ($CCE4) -> $C135: restore banks from $32/$33,
         * run C135, and on return the $CD08 seed re-maps banks 12/13 ($30/$31). */
        RAM8(0x0E) = 0x35; RAM8(0x0F) = 0xC1;
        RAM8(0x30) = RAM8(0x32); RAM8(0x31) = RAM8(0x33);  /* return_home restore */
        sub_C135(r);
        RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07;  /* $CD08 seed: banks 12/13, select=$07 */
        RAM8(0x09) = (u8)(RAM8(0x09) - 0x10);   /* SEC / SBC #$10 / STA $09 */
    } while (!(RAM8(0x09) & 0x80));     /* BPL L_B6AA */
    sub_C569(r);                        /* JSR $C569 */
}
