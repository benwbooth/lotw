



#include "game_memory.h"
#include "routine_context.h"

void routine_0100(RoutineContext *r);

void routine_0018(RoutineContext *r)
{
    int x;
    for (x = 0x3F; x >= 0; x--)
        GAME_MEM8((u16)(0x02C0 + x)) = GAME_MEM8((u16)(0xAB7C + x));
    routine_0100(r);
}
