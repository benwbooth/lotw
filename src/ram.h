/* Named RAM variables (Data Crystal + analysis-verified). Mirrors disasm/lotw.inc
 * so the C port reads like the disassembly. Addresses are authoritative. */
#ifndef LOTW_RAM_H
#define LOTW_RAM_H
#include "nes.h"

/* RNG (rng_update @ $CC64): $39-$3B state, $38 count/modulus, result &$7F -> $3B */
#define rng_count RAM8(0x38)
#define rng_s0    RAM8(0x39)   /* loop scratch */
#define rng_s1    RAM8(0x3A)
#define rng_s2    RAM8(0x3B)   /* result */

/* player / stats */
#define cur_character RAM8(0x40)
#define health        RAM8(0x58)
#define magic         RAM8(0x59)
#define gold          RAM8(0x5A)
#define keys          RAM8(0x5B)

#endif /* LOTW_RAM_H */
