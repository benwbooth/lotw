

















#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);

void routine_0175(RoutineContext *r);
void routine_0187(RoutineContext *r);
void routine_0137(RoutineContext *r);





int lotw_nonlocal_handoff = 0;

void routine_0167(RoutineContext *r)
{
    u8 a;
    u16 ptr;
    u8 x = GAME_MEM8(0x45);
    if (x == 0)
        return;
    x = (u8)(x - 1);
    GAME_MEM8(0x0D) = x;
    x = GAME_MEM8(0x44);
    GAME_MEM8(0x0C) = x;

    routine_0090(r);
    ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));

    r->offset = 0x00;
    a = GAME_MEM8((u16)(ptr + r->offset)) & 0x3F;
    if (a == 0x05) goto hit_E077;
    if (a == 0x04) goto hit_E424;
    if (a == 0x03) goto hit_D5F3;

    if (GAME_MEM8(0x43) == 0)
        return;

    r->offset = 0x0C;
    a = GAME_MEM8((u16)(ptr + r->offset)) & 0x3F;
    if (a == 0x05) goto hit_E077;
    if (a == 0x04) goto hit_E424;
    if (a == 0x03) goto hit_D5F3;

    return;

hit_E077:

    routine_0175(r);
    lotw_nonlocal_handoff = 1;
    return;

hit_E424:

    routine_0187(r);
    lotw_nonlocal_handoff = 1;
    return;

hit_D5F3:

    {
        u8 ei = GAME_MEM8(0x55);
        if (GAME_MEM8((u16)(0x51 + ei)) != 0x0E)
            return;

        {
            u8 cnt = GAME_MEM8(0x6E);
            int idx;
            for (idx = 2; idx >= 0; idx--) {
                if (GAME_MEM8((u16)(0x51 + idx)) == 0x0E)
                    cnt = (u8)(cnt + 1);
            }
            if (cnt != 0x04)
                return;
        }


        routine_0137(r);
        lotw_nonlocal_handoff = 1;
        return;
    }
}
