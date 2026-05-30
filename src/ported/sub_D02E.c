/* $D02E:  CMP $8E / BEQ ret / STA $8E / JSR song_init / ret
 * If A == current song id ($8E), do nothing; else store A as new song id
 * and (re)initialise the song. */
#include "ram.h"
#include "regs.h"

void song_init(Regs *r);

void sub_D02E(Regs *r)
{
    if (r->a == RAM8(0x8E))
        return;
    RAM8(0x8E) = r->a;
    song_init(r);
}
