/* $A7F0 — load the boss/title meter sprite block then draw the HP bar.
 * Copies the 64-byte OAM template at $AB7C into the sprite buffer $02C0..$02FF
 * (X = $3F..$00), then calls the player HP-bar setup $CB7F. Same-bank JSR
 * (no far-call wrapper). Pure data copy + tail call, RTS. */
#include "ram.h"
#include "regs.h"

void sub_CB7F(Regs *r);

void sub_A7F0(Regs *r)
{
    int x;
    for (x = 0x3F; x >= 0; x--)                  /* L_A7F2: LDA $AB7C,X / STA $02C0,X / DEX / BPL */
        RAM8((u16)(0x02C0 + x)) = RAM8((u16)(0xAB7C + x));
    sub_CB7F(r);                                 /* JSR $CB7F */
}
