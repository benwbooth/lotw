







#include "game_memory.h"
#include "routine_context.h"

void routine_0052(RoutineContext *r); void routine_0130(RoutineContext *r); void routine_0095(RoutineContext *r);
void routine_0096(RoutineContext *r); void routine_0081(RoutineContext *r); void routine_0060(RoutineContext *r);
void routine_0201(RoutineContext *r);

void routine_0178(RoutineContext *r)
{
    GAME_MEM8(0x0E) = 0x77;
    GAME_MEM8(0x0F) = 0xB5;
    routine_0052(r);
    if (r->carry)
        return;


    GAME_MEM8(0x8F) = 0x10;
    routine_0130(r);
    routine_0095(r);
    routine_0096(r);



    GAME_MEM8(0x7C) = 0x20;
    routine_0081(r);
    routine_0060(r);
    routine_0201(r);
}
