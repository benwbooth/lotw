






#include "game_memory.h"
#include "routine_context.h"

void routine_0227(RoutineContext *r);
void routine_0223(RoutineContext *r);

void routine_0226(RoutineContext *r)
{
    if (GAME_MEM8(0xF4) != 0) {

        routine_0223(r);
        return;
    }


    r->value = 0x01; routine_0227(r); if (r->carry) goto hit;
    r->value = 0x02; routine_0227(r); if (r->carry) goto hit;
    r->value = 0x04; routine_0227(r); if (r->carry) goto hit;
    r->value = 0x08; routine_0227(r); if (r->carry) goto hit;

    {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        u8 v = GAME_MEM8((u16)(ptr + 4));
        GAME_MEM8(0x00F2) = v;
        r->value = 0x00;
        GAME_MEM8(0xFC) = 0x00;
    }
    return;

hit:

    r->value = 0x01;
    GAME_MEM8(0xF4) = 0x01;
}
