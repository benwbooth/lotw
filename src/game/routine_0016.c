


#include "game_memory.h"
#include "routine_context.h"

void routine_0099(RoutineContext *r);

void routine_0016(RoutineContext *r)
{
    int x;
    for (x = 0x3F; x >= 0; x--)
        GAME_MEM8((u16)(0x0240 + x)) = GAME_MEM8((u16)(0xAAFC + x));
    routine_0099(r);
}
