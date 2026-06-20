






#include "game_memory.h"
#include "routine_context.h"

void routine_0234(RoutineContext *);
void routine_0108(RoutineContext *);
void routine_0248(RoutineContext *);
void routine_0239(RoutineContext *);
void routine_0238(RoutineContext *);
void routine_0242(RoutineContext *);

void routine_0221(RoutineContext *r)
{
    if ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) == 0)
        routine_0234(r);


    {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        u8 v = GAME_MEM8((u16)(ptr + 0x09));
        r->offset = v;
        r->value = GAME_MEM8(0xF4);
        routine_0108(r);
    }

    routine_0248(r);
    if (r->carry)
        routine_0239(r);
    else
        routine_0238(r);

    routine_0242(r);

}
