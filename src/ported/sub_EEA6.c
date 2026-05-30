/* $EEA6:  LDA #$03 / JSR rng_update / ASL A / TAX / LDA $EEB3,X / STA $F4 / RTS
 * Pick a random sound id: rng_update(mod 3) -> 0..2, *2 indexes a ROM table at
 * $EEB3 ({01,05,04,06,02,0A,08,09}), stores selected byte into $F4. */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);

/* ROM table at $EEB3 (mapped, read by the oracle) */
static const u8 sound_lookup_eeb3[8] = {
    0x01, 0x05, 0x04, 0x06, 0x02, 0x0A, 0x08, 0x09
};

void sub_EEA6(Regs *r)
{
    u8 x;
    r->a = 0x03;                   /* LDA #$03 (modulus); sets Z=0 for rng */
    rng_update(r);                 /* result in r->a, 0..2 */
    x = (u8)(r->a << 1);           /* ASL A / TAX */
    RAM8(0xF4) = sound_lookup_eeb3[x];
}
