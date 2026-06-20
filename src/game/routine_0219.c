





















#include "game_memory.h"
#include "routine_context.h"

void routine_0237(RoutineContext *r);
void routine_0236(RoutineContext *r);
void routine_0238(RoutineContext *r);
void routine_0250(RoutineContext *r);

void routine_0219(RoutineContext *r)
{
    u8 x, a;

    if (GAME_MEM8(0xF0) == 0) {
        if (GAME_MEM8(0xF1) == 0)
            goto flow_0489;
        routine_0237(r);
        if (r->carry)
            goto flow_0489;
        routine_0238(r);
    }


    routine_0236(r);
    if (r->carry)
        goto flow_0489;
    routine_0238(r);

flow_0489:
    x = (u8)(GAME_MEM8(0xF3) - 1);
    if (x == 0) {
        GAME_MEM8(0xEE) = 0x00;
        GAME_MEM8(0xF3) = 0xF0;
        r->index = x;
        return;
    }


    GAME_MEM8(0xF3) = x;
    if (x < 0x3C) {
        x = 0xEF;
        a = GAME_MEM8(0xFB);
        if (a == 0xEF)
            x = GAME_MEM8(0xFC);

        GAME_MEM8(0xFB) = x;
        GAME_MEM8(0xFC) = a;
    }


    routine_0250(r);
}
