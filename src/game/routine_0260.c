












#include "game_memory.h"
#include "routine_context.h"

void routine_0262(RoutineContext *r);
void routine_0247(RoutineContext *r);

void routine_0260(RoutineContext *r)
{
    u8 a = (u8)(GAME_MEM8(0xF0) >> 2);
    a = (u8)(a + 1);
    GAME_MEM8(0xF7) = a;
    r->value = a;

    routine_0262(r);
    if (!r->carry)
        return;


    GAME_MEM8(0xF5) = 0x00;
    GAME_MEM8(0xF6) = 0x00;
    r->value = 0x00;
    routine_0247(r);
    if (!r->carry)
        return;


    GAME_MEM8(0xF7) = 0x00;
    r->value = 0x00;
}
