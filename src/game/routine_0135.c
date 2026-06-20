










#include "game_memory.h"
#include "routine_context.h"

void routine_0204(RoutineContext *r); void routine_0146(RoutineContext *r); void routine_0173(RoutineContext *r);
void routine_0163(RoutineContext *r); void routine_0144(RoutineContext *r); void routine_0145(RoutineContext *r);
extern int lotw_nonlocal_handoff;

void routine_0135(RoutineContext *r)
{
    if (GAME_MEM8(0x4F) != 0)
        goto flow_0299;
    if (GAME_MEM8(0x22) != 0)
        return;


    GAME_MEM8(0x8F) = 0x1B;
    GAME_MEM8(0x4F) = GAME_MEM8(0x5C);
    {
        u8 x = GAME_MEM8(0x55);
        if (GAME_MEM8((u16)(0x51 + x)) == 0x06) {
            routine_0204(r);
            if (!r->carry) {
                u8 f = GAME_MEM8(0x4F);
                GAME_MEM8(0x4F) = (u8)((f >> 2) + f);
            }
        }
    }

flow_0299:


    lotw_nonlocal_handoff = 1;
    GAME_MEM8(0x22) = 0x01;
    {
        u8 old4f = GAME_MEM8(0x4F);
        GAME_MEM8(0x4F) = (u8)(old4f - 1);
        u8 t = (u8)(old4f >> 2);
        GAME_MEM8(0x4B) = (u8)((t ^ 0xFF) + 1);
    }
    routine_0146(r);
    if (!r->carry)
        goto flow_0301;


    GAME_MEM8(0x49) = 0x00; GAME_MEM8(0x4A) = 0x00;
    routine_0146(r);
    if (!r->carry)
        goto flow_0301;
    GAME_MEM8(0x4F)++;
    routine_0173(r);
    if (!r->carry)
        goto flow_0301;
    goto flow_0303;

flow_0301:
    GAME_MEM8(0x43) = GAME_MEM8(0x0E);
    GAME_MEM8(0x44) = GAME_MEM8(0x0F);
    {
        u8 y = GAME_MEM8(0x0A);
        if (y >= 0xEF) y = 0x00;
        GAME_MEM8(0x45) = y;
    }
    routine_0163(r);
    goto flow_0344;

flow_0303:
    GAME_MEM8(0x4F) = 0x00; GAME_MEM8(0x4E) = 0x00;
    routine_0163(r);

flow_0344:
    routine_0144(r);
    routine_0145(r);
}
