







#include "game_memory.h"
#include "routine_context.h"

void routine_0202(RoutineContext *r);

void routine_0165(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
    u8 v = GAME_MEM8((u16)(ptr + r->offset)) & 0x3F;

    if (v != 0x30) {
        r->carry = 0;
        return;
    }
    if (GAME_MEM8(0x4F) == 0)
        GAME_MEM8(0x4F) = 0x0A;
    if (GAME_MEM8(0x85) == 0) {
        routine_0202(r);
        GAME_MEM8(0x8F) = 0x0A;
        GAME_MEM8(0x85) = 0x01;
    }
    r->carry = 1;
}
