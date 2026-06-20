


#include "game_memory.h"
#include "routine_context.h"

void routine_0118(RoutineContext *r);

void routine_0117(RoutineContext *r)
{
    int x;
    for (x = 0x0F; x >= 0; --x) {
        r->index = (u8)x;
        r->offset = GAME_MEM8((u16)(0x0060 + x));
        routine_0118(r);
        r->index = (u8)x;
    }
    r->index = 0xFF;
}
