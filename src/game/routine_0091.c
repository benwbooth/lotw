








#include "game_memory.h"
#include "routine_context.h"

void routine_0091(RoutineContext *r)
{
    u16 four = (u16)(GAME_MEM8(0x0C) << 2);
    u16 eight = (u16)(GAME_MEM8(0x0C) << 3);
    u16 result = (u16)(four + eight);

    u8 x = (u8)(four >> 8);
    u8 y = (u8)four;

    GAME_MEM8(0x0C) = (u8)result;
    GAME_MEM8(0x0D) = (u8)(result >> 8);
    r->index = x;
    r->offset = y;
    r->value = (u8)(result >> 8);
}
