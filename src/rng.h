#ifndef LOTW_RNG_H
#define LOTW_RNG_H
#include "nes.h"
u8 rng_update(u8 count);   /* port of $CC64; A=count in, returns rng_s2 ($3B) */
#endif
