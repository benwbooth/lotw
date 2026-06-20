


#include "game_memory.h"
#include "routine_context.h"

void routine_0098(RoutineContext *r);

void routine_0017(RoutineContext *r)
{
    int x;
    for (x = 0x3F; x >= 0; x--)
        GAME_MEM8((u16)(0x0240 + x)) = GAME_MEM8((u16)(0xAB3C + x));
    routine_0098(r);
}
