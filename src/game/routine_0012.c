



#include "game_memory.h"
#include "routine_context.h"

void routine_0013(RoutineContext *r);

void routine_0012(RoutineContext *r)
{
    u8 count;

    GAME_MEM8(0x0F) = 0x88;
    GAME_MEM8(0x0E) = 0x10;
    count = 0x03;
    do {
        routine_0013(r);
        GAME_MEM8(0x0F) = (u8)(GAME_MEM8(0x0F) + 0x08);
        GAME_MEM8(0x0E) = (u8)(GAME_MEM8(0x0E) + 0x10);
        count = (u8)(count - 0x01);
    } while (count != 0);
}
