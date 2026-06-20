




































#include "game_memory.h"
#include "routine_context.h"

void routine_0241(RoutineContext *r);
void routine_0116(RoutineContext *r);
void routine_0253(RoutineContext *r);
void routine_0111(RoutineContext *r);
void routine_0109(RoutineContext *r);
void routine_0202(RoutineContext *r);
void routine_0090(RoutineContext *r);
void farcall_bank_09_r7(RoutineContext *r);





static void e99a_copy(RoutineContext *r)
{
    int y;
    for (y = 0x0F; y >= 0; --y) {
        u16 ptr = (u16)(GAME_MEM8(0xE5) | (GAME_MEM8(0xE6) << 8));
        GAME_MEM8((u16)(ptr + y)) = GAME_MEM8((u16)(0x00ED + y));
    }
    r->offset = 0xFF;
}

void routine_0272(RoutineContext *r)
{
#ifdef LOTW_HOST






    {
        u16 i;
        for (i = 0x0800; i < 0xA000; ++i)
            GAME_MEM8(i) = 0;
    }
#endif


    if (GAME_MEM8(0xED) & 0x01) {
        if ((GAME_MEM8(0xF3) & 0x03) == 0)
            GAME_MEM8(0xED) ^= 0x04;
        goto done;
    }


    GAME_MEM8(0xE3) = 0x09;
    routine_0241(r);

    routine_0116(r);
    if (r->carry) goto flow_0671;

    routine_0253(r);
    if (r->carry) goto flow_0671;

    routine_0111(r);
    if (r->carry) goto flow_0670;

    routine_0109(r);
    if (r->carry) {
        u8 x = GAME_MEM8(0x09);
        GAME_MEM8((u16)(0x0401 + x)) = 0x80;
    }

    GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
    GAME_MEM8(0xF4) = 0x00;
    goto done;

flow_0670:
    if (GAME_MEM8(0xF4) != 0) goto flow_0674;
    if (GAME_MEM8(0x85) != 0) goto flow_0671;
    routine_0202(r);
    GAME_MEM8(0x8F) = 0x0A;
    GAME_MEM8(0x85) = 0x02;

flow_0671:
    if (GAME_MEM8(0xF4) != 0) goto flow_0674;
    GAME_MEM8(0xF4) = (u8)(GAME_MEM8(0xF4) + 1);
    if (GAME_MEM8(0xF5) != 0) {
        GAME_MEM8(0xF5) = (u8)((0 - GAME_MEM8(0xF5)) & 0x0F);
        GAME_MEM8(0xF6) ^= 0xFF;
    }

    GAME_MEM8(0xF7) = (u8)((u8)(~GAME_MEM8(0xF7)) + 1);
    if (GAME_MEM8(0x8F) == 0)
        GAME_MEM8(0x8F) = 0x06;
    goto done;

flow_0674:
    if ((u8)((GAME_MEM8(0xFB) & 0x0F) | GAME_MEM8(0xF9)) != 0) {
        GAME_MEM8(0xF3) = (u8)(GAME_MEM8(0xF3) + 1);
        goto done;
    }

    {
        GAME_MEM8(0xEE) = 0x00;
        if (GAME_MEM8(0xF0) != 0) {
            u16 ptr;
            u8 diff;
            GAME_MEM8(0x0C) = GAME_MEM8(0xFA);
            GAME_MEM8(0x0D) = GAME_MEM8(0xFB);
            routine_0090(r);
            ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
            GAME_MEM8(ptr) = GAME_MEM8(0xF0);

            diff = (u8)(GAME_MEM8(0xFA) - GAME_MEM8(0x7C));

            if (diff < 0x11 || diff >= 0xFE) {
                u8 fa = GAME_MEM8(0xFA);
                GAME_MEM8(0x0C) = fa;
                GAME_MEM8(0x16) = (u8)((fa << 1) & 0x1F);
                GAME_MEM8(0x17) = (u8)((GAME_MEM8(0xFA) & 0x10) >> 2);
                GAME_MEM8(0x16) = (u8)(0x00 + GAME_MEM8(0x16));
                GAME_MEM8(0x17) = (u8)(0x20 + GAME_MEM8(0x17));
                farcall_bank_09_r7(r);
            }

        }

    }

done:

    e99a_copy(r);
}
