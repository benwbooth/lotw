/* $CB53 — set up the boss-life meter bar: clamp $0405 to $6D as the length,
 * column base $09=0, full tile $A5 / empty tile $AB, then draw (tail-call CB94).
 * Reached via far-call ($0C0D) from F55E/A7E1. */
#include "ram.h"
#include "regs.h"

void sub_CB94(Regs *r);

void sub_CB53(Regs *r)
{
    u8 a = RAM8(0x0405);            /* LDA $0405 */
    if (a >= 0x6D) a = 0x6D;        /* CMP #$6D / BCC / LDA #$6D */
    RAM8(0x08) = a;                 /* STA $08 (bar length) */
    RAM8(0x09) = 0x00;              /* STA $09 (column base) */
    r->x = 0xA5;                    /* LDX #$A5 (full tile) */
    r->y = 0xAB;                    /* LDY #$AB (empty tile) */
    sub_CB94(r);                    /* JMP L_CB94 */
}
