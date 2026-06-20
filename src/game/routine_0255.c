

#include "game_memory.h"
#include "routine_context.h"

void routine_0255(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
    u8 v = GAME_MEM8((u16)(ptr + r->offset)) & 0x3F;
    r->carry = (u8)(v >= 0x30);
}
