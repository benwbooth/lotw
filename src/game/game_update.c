






























#include "game_memory.h"
#include "routine_context.h"


void routine_0140(RoutineContext *r);
void routine_0136(RoutineContext *r);
void routine_0167(RoutineContext *r);
extern int lotw_nonlocal_handoff;
void routine_0107(RoutineContext *r);
void routine_0135(RoutineContext *r);
void routine_0146(RoutineContext *r);
void routine_0173(RoutineContext *r);
void routine_0163(RoutineContext *r);
void routine_0103(RoutineContext *r);
void routine_0144(RoutineContext *r);
void routine_0145(RoutineContext *r);





void routine_0139(RoutineContext *r);
void routine_0174(RoutineContext *r);

void game_update(RoutineContext *r)
{
    u8 a, y;

    GAME_MEM8(0xE3) = 0xFF;
    if (GAME_MEM8(0xEB) != 0) {
        routine_0139(r);
        return;
    }

flow_0280:
    routine_0140(r);
    if (GAME_MEM8(0x20) & 0x10) {
        routine_0174(r);
        return;
    }


    routine_0136(r);
    if (GAME_MEM8(0x46) != 0) {
        GAME_MEM8(0x46)--;
        GAME_MEM8(0x20) = 0x00;
    }


    {
        int clear_hi = 1;
        if (GAME_MEM8(0x40)  == 0x04) {
            if ((GAME_MEM8(0x84) & 0x07) == 0)
                clear_hi = 1;
            else
                clear_hi = (GAME_MEM8(0x20) & 0x40) ? 0 : 1;
        } else {

            clear_hi = (GAME_MEM8(0x20) & 0x40) ? 0 : 1;
        }
        if (clear_hi)
            GAME_MEM8(0xFD) &= 0x0F;
    }


    a = GAME_MEM8(0x20) & 0x0F;
    if (a != 0) {
        GAME_MEM8(0x08) = a;
        GAME_MEM8(0xFD) = (u8)((GAME_MEM8(0xFD) & 0xF0) | a);
    }


    if (GAME_MEM8(0x20) & 0x20)
        goto flow_0304;


    if (GAME_MEM8(0x20) & 0x08) {
        routine_0167(r);



        if (lotw_nonlocal_handoff) { lotw_nonlocal_handoff = 0; return; }
    }


    y = 0x01;
    while (GAME_MEM8((u16)(0x0087 + y)) != 0) {
        y++;
        if (y >= 0x05) {
            y = 0x06;
            break;
        }
    }
    r->offset = y;


    routine_0107(r);

    if (GAME_MEM8(0x4E) != 0) {

        GAME_MEM8(0x4B) = (u8)((GAME_MEM8(0x4E) >> 2) + 1);
        routine_0146(r);
        if (!r->carry)
            goto flow_0301;

        GAME_MEM8(0x49) = 0x00; GAME_MEM8(0x4A) = 0x00;
        routine_0146(r);
        if (!r->carry)
            goto flow_0301;
        goto flow_0303;
    }


    if (GAME_MEM8(0x4F) != 0) {

        routine_0135(r);



        if (lotw_nonlocal_handoff) { lotw_nonlocal_handoff = 0; return; }
        GAME_MEM8(0x4F) = 0x00;
    } else if (GAME_MEM8(0x20) & 0x80) {
        routine_0135(r);
        if (lotw_nonlocal_handoff) { lotw_nonlocal_handoff = 0; return; }
        GAME_MEM8(0x4F) = 0x00;
    } else {

        GAME_MEM8(0x22) = 0x00;
        GAME_MEM8(0x4F) = 0x00;
    }


    routine_0146(r);
    if (!r->carry)
        goto flow_0294;
    routine_0173(r);
    if (!r->carry)
        goto flow_0294;
    goto flow_0303;
flow_0294:
    goto flow_0301;

flow_0301:

    GAME_MEM8(0x43)  = GAME_MEM8(0x0E);
    GAME_MEM8(0x44)  = GAME_MEM8(0x0F);
    a = GAME_MEM8(0x0A);
    if (a >= 0xEF)
        a = 0x00;
    GAME_MEM8(0x45)  = a;
    routine_0163(r);
    goto flow_0344;

flow_0303:
    GAME_MEM8(0x4F) = 0x00;
    GAME_MEM8(0x4E) = 0x00;
    routine_0163(r);
    goto flow_0344;

flow_0304:



    GAME_MEM8(0x8F) = 0x10;
    for (;;) {
        routine_0103(r);
        if (r->value & 0xF0)
            break;
        if ((GAME_MEM8(0x20) & 0x03) == 0)
            continue;
        GAME_MEM8(0x20) <<= 1;
        GAME_MEM8(0x20) <<= 1;
        r->offset = 0x01;
        routine_0107(r);
        {
            u8 t = (u8)(GAME_MEM8(0x4B) + GAME_MEM8(0x55) );
            u8 ni;
            if (t & 0x80)
                ni = 0x03;
            else if (t < 0x04)
                ni = t;
            else
                ni = 0x00;
            GAME_MEM8(0x55)  = ni;
        }
        GAME_MEM8(0x8F) = 0x0C;

    }


    GAME_MEM8(0x8F) = 0x10;


flow_0344:

    routine_0144(r);
    routine_0145(r);
}
