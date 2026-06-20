




#include "game_memory.h"
#include "routine_context.h"

void routine_0032(RoutineContext *r)
{
    r->index = (u8)((GAME_MEM8(0x20) & 0x0F) << 1);
    GAME_MEM8(0x49) = GAME_MEM8((u16)(0xFE8B + r->index));
    GAME_MEM8(0x4B) = GAME_MEM8((u16)(0xFE8C + r->index));
}
