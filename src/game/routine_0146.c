















#include "game_memory.h"
#include "routine_context.h"

void routine_0143(RoutineContext *r); void routine_0115(RoutineContext *r); void routine_0142(RoutineContext *r);
void routine_0168(RoutineContext *r); void routine_0110(RoutineContext *r); void routine_0148(RoutineContext *r);
void routine_0149(RoutineContext *r); void routine_0089(RoutineContext *r); void routine_0150(RoutineContext *r);
void routine_0147(RoutineContext *r);

void routine_0146(RoutineContext *r)
{
    u8 save4B = GAME_MEM8(0x4B);
    u8 save49 = GAME_MEM8(0x49);
    u8 a, x, v;

flow_0353:
    routine_0143(r);
    routine_0115(r);
    if (r->carry) {
        routine_0142(r);
        if (r->carry)
            goto flow_0363;
        goto flow_0359;
    }


    routine_0168(r);
    if (r->carry)
        goto flow_0359;
    routine_0110(r);
    if (!r->carry)
        goto flow_0364;
    a = GAME_MEM8(0x08);
    if (a == 0x09)
        goto flow_0359;
    if (a < 0x09)
        goto flow_0356;

    x = GAME_MEM8(0x09);
    r->index = x;
    v = GAME_MEM8((u16)(0x0401 + x));
    r->value = v;
    if (v == 0x01) {
        routine_0148(r);
        goto flow_0364;
    }

    routine_0149(r);
    routine_0089(r);
    goto flow_0363;

flow_0356:
    x = GAME_MEM8(0x09);
    r->index = x;
    v = GAME_MEM8((u16)(0x0401 + x));
    r->value = v;
    if (v == 0x01)
        goto flow_0357;
    if (v >= 0x1A)
        goto flow_0358;
    routine_0150(r);
    goto flow_0363;
flow_0357:
    routine_0147(r);
flow_0358:
    r->carry = 0;
    goto flow_0364;

flow_0359:
    if (GAME_MEM8(0x88) == 0)
        goto flow_0361;
    a = GAME_MEM8(0x49);
    if (a == 0)
        goto flow_0361;
    x = a;
    if (!(a & 0x08))
        x = (u8)(x - 2);

    x = (u8)(x + 1);
    a = (u8)(x & 0x0F);
    GAME_MEM8(0x49) = a;
    if (a != 0)
        goto flow_0353;

flow_0361:
    GAME_MEM8(0x49) = save49;
    x = GAME_MEM8(0x4B);
    if (x == 0)
        goto flow_0363;
    if (!(x & 0x80))
        x = (u8)(x - 2);

    x = (u8)(x + 1);
    GAME_MEM8(0x4B) = x;
    if (x != 0)
        goto flow_0353;

flow_0363:
    r->carry = 1;
flow_0364:
    GAME_MEM8(0x49) = save49;
    GAME_MEM8(0x4B) = save4B;



}
