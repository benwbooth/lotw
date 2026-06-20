







#include "game_memory.h"
#include "routine_context.h"

void routine_0232(RoutineContext *);
void routine_0108(RoutineContext *);
void routine_0248(RoutineContext *);
void routine_0256(RoutineContext *);
void routine_0239(RoutineContext *);
void routine_0238(RoutineContext *);
void routine_0242(RoutineContext *);

void routine_0224(RoutineContext *r)
{

    int skip = ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) != 0) && (GAME_MEM8(0xF3) < 0x20);
    if (!skip)
        routine_0232(r);


    {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        r->offset = GAME_MEM8((u16)(ptr + 0x09));
        r->value = GAME_MEM8(0xF4);
        routine_0108(r);
    }

    routine_0248(r);
    if (r->carry) {
        routine_0256(r);
        if (r->carry) {
            routine_0239(r);
            routine_0242(r);
            return;
        }
    }


    routine_0238(r);
    routine_0242(r);

}
