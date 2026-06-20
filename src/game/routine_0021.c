
























#include "game_memory.h"
#include "routine_context.h"

void routine_0030(RoutineContext *r); void routine_0032(RoutineContext *r); void routine_0027(RoutineContext *r);
void routine_0022(RoutineContext *r); void routine_0028(RoutineContext *r); void routine_0024(RoutineContext *r);
void routine_0025(RoutineContext *r); void routine_0026(RoutineContext *r); void routine_0100(RoutineContext *r);
void routine_0029(RoutineContext *r);


static void tail_acbb(RoutineContext *r)
{
    routine_0024(r);
    routine_0025(r);
    routine_0026(r);
}
static void tail_aca1(RoutineContext *r)
{
    GAME_MEM8(0x43) = GAME_MEM8(0x0E);
    GAME_MEM8(0x45) = GAME_MEM8(0x0A);
    routine_0028(r);
    tail_acbb(r);
}
static void tail_acaf(RoutineContext *r)
{
    GAME_MEM8(0x4F) = 0x00;
    GAME_MEM8(0x4E) = 0x00;
    routine_0028(r);
    tail_acbb(r);
}

void routine_0021(RoutineContext *r)
{

    r->value = GAME_MEM8(0x20);
    if (r->value & 0x10) {
        routine_0029(r);
        return;
    }

    if (!(GAME_MEM8(0x20) & 0x40)) {
        GAME_MEM8(0xFD) = (u8)(GAME_MEM8(0xFD) & 0x0F);
    }

    r->value = (u8)(GAME_MEM8(0x20) & 0x0F);
    if (r->value != 0) {
        GAME_MEM8(0x08) = r->value;
        GAME_MEM8(0xFD) = (u8)((GAME_MEM8(0xFD) & 0xF0) | GAME_MEM8(0x08));
    }


    if (GAME_MEM8(0x85) == 0) {

        if ((GAME_MEM8(0x26) & 0x40) == 0)
            goto flow_0092;
        r->index = (u8)(GAME_MEM8(0x3E) + 1);
        if (((r->index) & 0x06) != 0)
            goto flow_0092;
        {
            u8 sum = (u8)(GAME_MEM8(0x1C) + GAME_MEM8((u16)(0x040C + r->index)));
            r->value = (sum < 0xB0) ? 0x0A : 0x05;
        }
        routine_0030(r);
        GAME_MEM8(0x4F) = 0x0A;
        GAME_MEM8(0x8F) = 0x21;
        GAME_MEM8(0x90) = 0x02;
        GAME_MEM8(0x85) = 0x01;
        routine_0100(r);
    }


    if (GAME_MEM8(0x4F) == 0 && GAME_MEM8(0x4E) == 0) {
        GAME_MEM8(0x85) = 0x00;
        goto flow_0092;
    }

    GAME_MEM8(0x20) = (u8)((GAME_MEM8(0x20) & 0xF0) | 0x02);

flow_0092:
    routine_0032(r);
    if (GAME_MEM8(0x4E) != 0) {

        r->value = (u8)(GAME_MEM8(0x4E) >> 2);
        r->value = (u8)(r->value + 1);
        GAME_MEM8(0x4B) = r->value;
        routine_0027(r);
        if (!r->carry) {
            tail_aca1(r);
            return;
        }

        GAME_MEM8(0x49) = 0x00;
        routine_0027(r);
        if (!r->carry) { tail_aca1(r); return; }
        tail_acaf(r);
        return;
    }


    if (GAME_MEM8(0x4F) != 0) {
        goto flow_0093;
    }
    if (!(GAME_MEM8(0x20) & 0x80)) {

        GAME_MEM8(0x22) = 0x00;
        r->value = 0x00;
        goto flow_0095;
    }
flow_0093:
    routine_0022(r);
    r->value = 0x00;
flow_0095:
    GAME_MEM8(0x4F) = r->value;
    routine_0027(r);
    if (r->carry) {
        tail_acaf(r);
        return;
    }

    tail_aca1(r);
}
