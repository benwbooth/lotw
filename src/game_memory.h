
#ifndef LOTW_GAME_MEMORY_H
#define LOTW_GAME_MEMORY_H
#include "platform.h"


#define rng_count GAME_MEM8(0x38)
#define rng_s0    GAME_MEM8(0x39)
#define rng_s1    GAME_MEM8(0x3A)
#define rng_s2    GAME_MEM8(0x3B)


#define cur_character GAME_MEM8(0x40)
#define health        GAME_MEM8(0x58)
#define magic         GAME_MEM8(0x59)
#define gold          GAME_MEM8(0x5A)
#define keys          GAME_MEM8(0x5B)

#endif
