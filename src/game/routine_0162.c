






#include "game_memory.h"
#include "routine_context.h"

void routine_0074(RoutineContext *r);

void routine_0162(RoutineContext *r)
{
    u8 x = 0x09;
    u8 y = 0x00;
    do {
        if (GAME_MEM8((u16)(0x0401 + y)) == 0x01)
            GAME_MEM8((u16)(0x0401 + y)) = 0x80;
        y = (u8)(y + 0x10);
    } while (--x != 0);
    GAME_MEM8(0x8F) = 0x18;
    GAME_MEM8(0x90) = 0xFF;
    r->index = 0x02;
    routine_0074(r);
}
