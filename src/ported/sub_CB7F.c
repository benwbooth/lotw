/* $CB7F (bankfix) — set up the player HP meter bar then tail-call the generic
 * bar drawer. Clamps the bar length to health (max $6D), into $08; sets the OAM
 * column base $09 = $80, the full-cell tile X = $65, empty-cell tile Y = $6B,
 * and jumps to L_CB94 (sub_CB94). Terminating tail-call. */
#include "ram.h"
#include "regs.h"

void sub_CB94(Regs *r);

void sub_CB7F(Regs *r)
{
    u8 a = RAM8(0x58);                              /* LDA health */
    if (a >= 0x6D) a = 0x6D;                        /* CMP #$6D / BCC L_CB87 / LDA #$6D */
    RAM8(0x08) = a;                                 /* L_CB87: STA $08 */
    RAM8(0x09) = 0x80;                              /* LDA #$80 / STA $09 */
    r->x = 0x65;                                    /* LDX #$65 */
    r->y = 0x6B;                                    /* LDY #$6B */
    sub_CB94(r);                                    /* JMP L_CB94 */
}
