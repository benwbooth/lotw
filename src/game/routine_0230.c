










#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0255(RoutineContext *r);

void routine_0230(RoutineContext *r)
{
    if ((GAME_MEM8(0x0A) & 0x0F) != 0) { r->carry = 0; return; }

    GAME_MEM8(0x0C) = GAME_MEM8(0x0F);
    GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0A) - 0x10);

    routine_0090(r);

    r->offset = 0x00;
    routine_0255(r);
    if (r->carry == 0) return;

    if (GAME_MEM8(0x0E) == 0) return;

    r->offset = 0x0C;
    routine_0255(r);
    if (r->carry == 0) return;

}
