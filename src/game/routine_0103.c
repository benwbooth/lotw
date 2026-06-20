



#include "game_memory.h"
#include "routine_context.h"
void routine_0104(RoutineContext *r); void routine_0105(RoutineContext *r);
void routine_0103(RoutineContext *r)
{
    routine_0104(r);
    routine_0105(r);
    {
        u8 btn = r->value;
        routine_0104(r);
        r->value = btn;
        GAME_MEM8(0x20) = btn;
    }
}
