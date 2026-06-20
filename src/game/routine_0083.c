


#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void metasprite_build(RoutineContext *r);

void routine_0083(RoutineContext *r)
{
    GAME_MEM8(0x0D) = 0x00;
    routine_0090(r);
    GAME_MEM8(0x0D) = (u8)((u8)(GAME_MEM8(0x0D) - 0x05) + GAME_MEM8(0x76));
    metasprite_build(r);
}
