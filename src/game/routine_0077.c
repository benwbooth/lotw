






#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0079(RoutineContext *r);

void routine_0077(RoutineContext *r)
{
    GAME_MEM8(0x0C) = GAME_MEM8(0x7C) & 0xFE;
    GAME_MEM8(0x0D) = 0x00;
    routine_0090(r);
    routine_0079(r);
}
