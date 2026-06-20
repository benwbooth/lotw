








#include "game_memory.h"
#include "routine_context.h"

void routine_0146(RoutineContext *r);

void routine_0173(RoutineContext *r)
{
    u8 v49 = GAME_MEM8(0x49);
    GAME_MEM8(0x49) = 0x00;
    GAME_MEM8(0x4A) = 0x00;
    if (v49 == 0)
        goto flow_0409;


    {
        u8 a = (u8)(GAME_MEM8(0x45) & 0x0F);
        if (a == 0)
            goto flow_0413;
        if (a < 0x06) {
            if (GAME_MEM8(0x20) & 0x04)
                goto flow_0413;
            GAME_MEM8(0x4B) = 0xFF;
            GAME_MEM8(0x4C) = 0xFF;
            goto flow_0412;
        }
        if (a >= 0x0B) {
            if (GAME_MEM8(0x20) & 0x08)
                goto flow_0413;
            GAME_MEM8(0x4B) = 0x01;
            GAME_MEM8(0x4C) = 0x00;
            goto flow_0412;
        }
        goto flow_0413;
    }

flow_0409:
    {
        u8 v4B = GAME_MEM8(0x4B);
        GAME_MEM8(0x4B) = 0x00;
        GAME_MEM8(0x4C) = 0x00;
        if (v4B == 0)
            goto flow_0413;
        u8 a = GAME_MEM8(0x43);
        if (a == 0)
            goto flow_0413;
        if (a < 0x06) {
            if (GAME_MEM8(0x20) & 0x01)
                goto flow_0413;
            GAME_MEM8(0x49) = 0x0F;
            GAME_MEM8(0x4A) = 0xFF;
            goto flow_0412;
        }
        if (a >= 0x0B) {
            if (GAME_MEM8(0x20) & 0x02)
                goto flow_0413;
            GAME_MEM8(0x49) = 0x01;
            GAME_MEM8(0x4A) = 0x00;
            goto flow_0412;
        }
        goto flow_0413;
    }

flow_0412:
    routine_0146(r);
    return;
flow_0413:
    r->carry = 1;
}
