

#include "game_memory.h"
#include "routine_context.h"

void routine_0206(RoutineContext *r);

void routine_0152(RoutineContext *r)
{
    GAME_MEM8(0x8F) = 0x11;
    r->value = 0x05;
    routine_0206(r);
}
