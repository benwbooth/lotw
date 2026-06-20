











#include "game_memory.h"
#include "routine_context.h"

void routine_0262(RoutineContext *r);

void routine_0261(RoutineContext *r)
{
    u8 x = GAME_MEM8(0xF1);
    if (x == 0)
        x = 0x19;
    x = (u8)(x - 1);
    GAME_MEM8(0xF1) = x;
    r->index = x;

    GAME_MEM8(0xF7) = (u8)(((x >> 2) ^ 0xFF) + 1);

    routine_0262(r);
    if (!r->carry)
        return;

    GAME_MEM8(0xF5) = 0x00;
    GAME_MEM8(0xF6) = 0x00;
    routine_0262(r);
}
