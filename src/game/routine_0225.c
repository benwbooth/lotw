



















#include "game_memory.h"
#include "routine_context.h"

void routine_0108(RoutineContext *r);
void routine_0112(RoutineContext *r);
void routine_0230(RoutineContext *r);
void routine_0233(RoutineContext *r);
void routine_0236(RoutineContext *r);
void routine_0237(RoutineContext *r);
void routine_0238(RoutineContext *r);
void routine_0239(RoutineContext *r);
void routine_0242(RoutineContext *r);
void routine_0247(RoutineContext *r);
void routine_0250(RoutineContext *r);

void routine_0225(RoutineContext *r)
{
    if (GAME_MEM8(0xF0) != 0)
        goto flow_0527;
    if (GAME_MEM8(0xF1) != 0)
        goto flow_0529;

    GAME_MEM8(0x0F) = GAME_MEM8(0xFA);
    GAME_MEM8(0x0E) = GAME_MEM8(0xF9);
    GAME_MEM8(0x0A) = GAME_MEM8(0xFB);

    routine_0230(r);
    if (r->carry)
        goto flow_0524;
    GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
    GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
    goto flow_0527;

flow_0524:
    if ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) == 0)
        routine_0233(r);


    routine_0112(r);
    if (r->carry)
        goto flow_0526;
    {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        r->offset = GAME_MEM8((u16)(ptr + 0x09));
    }
    r->value = GAME_MEM8(0xF4);
    routine_0108(r);
    routine_0247(r);
    if (r->carry)
        goto flow_0530;
    routine_0230(r);
    if (!r->carry)
        goto flow_0530;
    goto flow_0528;

flow_0526:
    GAME_MEM8(0xF5) = 0x00;
    GAME_MEM8(0xF6) = 0x00;
    routine_0250(r);

    if (r->carry)
        goto flow_0530;

flow_0527:
    routine_0236(r);
    routine_0238(r);
    {
        u8 saved_f0 = GAME_MEM8(0xF0);
        routine_0250(r);

        if (!r->carry)
            goto flow_0528;

        GAME_MEM8(0xF1) = (u8)(saved_f0 + 0x05 + 1);
        goto flow_0531;
    }

flow_0528:
    routine_0238(r);
    goto flow_0531;

flow_0529:
    routine_0237(r);
    if (r->carry)
        goto flow_0530;
    routine_0238(r);
    goto flow_0531;

flow_0530:
    routine_0239(r);

flow_0531:
    routine_0242(r);
}
