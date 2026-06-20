








#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);

void routine_0164(RoutineContext *r)
{
    if ((GAME_MEM8(0x86) | GAME_MEM8(0x4F)) != 0) { r->carry = 1; return; }
    if (GAME_MEM8(0x0E) != 0)                { r->carry = 0; return; }

    GAME_MEM8(0x0C) = GAME_MEM8(0x0F);
    GAME_MEM8(0x0D) = 0x00;

    routine_0090(r);

    {
        u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
        u8 v = GAME_MEM8(ptr) & 0x3F;
        r->carry = (v == 0) ? 1 : 0;
    }
}
