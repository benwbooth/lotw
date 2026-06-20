






















#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);
void routine_0108(RoutineContext *r);
void routine_0235(RoutineContext *r);
void routine_0236(RoutineContext *r);
void routine_0237(RoutineContext *r);
void routine_0238(RoutineContext *r);
void routine_0239(RoutineContext *r);
void routine_0242(RoutineContext *r);
void routine_0247(RoutineContext *r);
void routine_0250(RoutineContext *r);

void routine_0220(RoutineContext *r)
{
    u8 saved_f6;
    int do_place = 0;

    if (GAME_MEM8(0xF3) >= 0x20) {

    } else if (GAME_MEM8(0xF1) != 0) {
        do_place = 1;
    } else if ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) != 0) {
        do_place = 1;
    }

    if (!do_place) {

        GAME_MEM8(0xF3) = 0x00;
        routine_0235(r);
        r->value = 0x06;
        rng_update(r);
        GAME_MEM8(0xF6) = (u8)(r->value + 1);
        r->value = 0x04;
        rng_update(r);
        r->index = r->value;
        if (r->value == 0) {
            GAME_MEM8(0xF4) = (u8)(0x80 | GAME_MEM8(0xF4));
        }
    }


    saved_f6 = GAME_MEM8(0xF6);
    r->offset = GAME_MEM8(0xF6);
    r->value = GAME_MEM8(0xF4);
    routine_0108(r);

    if (GAME_MEM8(0xF0) != 0) {

        routine_0236(r);
        if (r->carry) goto flow_0499;
        goto flow_0498;
    }

    if (GAME_MEM8(0xF1) != 0)
        goto flow_0495;
    if (!(GAME_MEM8(0xF4) & 0x80))
        goto flow_0496;

flow_0495:
    routine_0237(r);
    if (!r->carry) goto flow_0498;


flow_0496:
    GAME_MEM8(0xF1) = 0x00;
    routine_0247(r);
    if (!r->carry) goto flow_0498;
    routine_0239(r);
    goto flow_0499;

flow_0498:
    routine_0238(r);

flow_0499:
    routine_0250(r);
    routine_0242(r);
    GAME_MEM8(0xF6) = saved_f6;
}
