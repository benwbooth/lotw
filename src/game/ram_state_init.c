







#include "game_memory.h"
#include "routine_context.h"

void ram_state_init(RoutineContext *r)
{
    u8 x;
    signed char i;

    x = 0;
    do { GAME_MEM8(0x0000 + x) = GAME_MEM8(0x9B9F + x); } while (++x != 0);

    for (i = 0x3F; i >= 0; i--)
        GAME_MEM8(0x0100 + (u8)i) = GAME_MEM8(0x9C9E + (u8)i);

    for (i = 0x1F; i >= 0; i--)
        GAME_MEM8(0x0180 + (u8)i) = 0x0F;

    x = 0;
    do { GAME_MEM8(0x0300 + x) = GAME_MEM8(0x9D3E + x); } while (++x != 0);

    x = 0;
    do { GAME_MEM8(0x0400 + x) = GAME_MEM8(0x9DC9 + x); } while (++x != 0);
}
