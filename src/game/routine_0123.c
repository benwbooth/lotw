


#include "game_memory.h"
#include "routine_context.h"

void song_init(RoutineContext *r);

void routine_0123(RoutineContext *r)
{
    if (r->value == GAME_MEM8(0x8E))
        return;
    GAME_MEM8(0x8E) = r->value;
    song_init(r);
}
