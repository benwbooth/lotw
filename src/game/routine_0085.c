











#include "game_memory.h"
#include "routine_context.h"

void routine_0085(RoutineContext *r)
{
    u8 lo, hi;
    u16 ptr;
    int i;

    GAME_MEM8(0x77) = GAME_MEM8(0x75);
    GAME_MEM8(0x78) = GAME_MEM8(0x76);
    lo = GAME_MEM8(0x77);
    hi = GAME_MEM8(0x78);


    ptr = (u16)(lo | (hi << 8));
    for (i = 0; i < 256; i++)
        GAME_MEM8((u16)(0x0500 + i)) = GAME_MEM8((u16)(ptr + i));
    hi++;
    GAME_MEM8(0x78) = hi;


    ptr = (u16)(lo | (hi << 8));
    for (i = 0; i < 256; i++)
        GAME_MEM8((u16)(0x0600 + i)) = GAME_MEM8((u16)(ptr + i));
    hi++;
    GAME_MEM8(0x78) = hi;


    ptr = (u16)(lo | (hi << 8));
    for (i = 0; i < 256; i++)
        GAME_MEM8((u16)(0x0700 + i)) = GAME_MEM8((u16)(ptr + i));
    hi++;
    GAME_MEM8(0x78) = hi;

    r->offset = 0;
}
