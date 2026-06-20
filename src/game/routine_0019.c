


#include "game_memory.h"
#include "routine_context.h"

void routine_0019(RoutineContext *r)
{
    (void)r;
    u8 v = (u8)(GAME_MEM8(0x56) & 0x1F);
    GAME_MEM8(0x08) = v;

    GAME_MEM8(0x0410) = (u8)((GAME_MEM8(0x0410) & 0xE0) | v);
    GAME_MEM8(0x0420) = (u8)((GAME_MEM8(0x0420) & 0xE0) | v);
    GAME_MEM8(0x0430) = (u8)((GAME_MEM8(0x0430) & 0xE0) | v);

    u8 xf = GAME_MEM8(0x43);
    GAME_MEM8(0x041C) = xf;
    GAME_MEM8(0x042C) = xf;
    GAME_MEM8(0x043C) = xf;

    u8 x = GAME_MEM8(0x44);
    x++;
    GAME_MEM8(0x042D) = x;
    x -= 3;
    GAME_MEM8(0x043D) = x;
    x--;
    GAME_MEM8(0x041D) = x;
}
