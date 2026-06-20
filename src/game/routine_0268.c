


























#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r);
void routine_0241(RoutineContext *r);
void routine_0115(RoutineContext *r);
void routine_0109(RoutineContext *r);
void routine_0270(RoutineContext *r);
void routine_0214(RoutineContext *r);

static void store_pos(void)
{
    GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
}

static void finish(RoutineContext *r)
{
    if (GAME_MEM8(0xEE) != 0)
        routine_0270(r);
    routine_0214(r);
}

void routine_0268(RoutineContext *r)
{
    routine_0213(r);

    GAME_MEM8(0xEE) = (u8)(GAME_MEM8(0xEE) - 1);
    if (GAME_MEM8(0xEE) == 0) { finish(r); return; }

    routine_0241(r);

    routine_0115(r);
    if (r->carry) {
        GAME_MEM8(0xEE) = 0x00;
        finish(r);
        return;
    }

    routine_0109(r);
    if (!r->carry) {
        store_pos();
        finish(r);
        return;
    }


    if (GAME_MEM8(0x2D) >= 0x30 && GAME_MEM8(0x08) >= 0x04) {
        u8 x = GAME_MEM8(0x09);
        GAME_MEM8((u16)(0x0401 + x)) = 0x80;
        GAME_MEM8(0xEE) = 0x01;
        GAME_MEM8(0x8F) = 0x0C;
        store_pos();
        finish(r);
        return;
    }


    {
        u8 x = GAME_MEM8(0x09);
        if ((u8)(GAME_MEM8((u16)(0x0401 + x)) - 1) != 0) {
            store_pos();
            finish(r);
            return;
        }

        x = GAME_MEM8(0x09);
        {
            u8 yv = (GAME_MEM8(0xEE) & 0x01) ? 0x02 : 0xFE;
            GAME_MEM8((u16)(0x040F + x)) = yv;
        }
        {
            u8 cur = GAME_MEM8((u16)(0x0405 + x));
            u8 sub = GAME_MEM8(0xF8);
            GAME_MEM8((u16)(0x0405 + x)) = (u8)(cur - sub);
            if (cur >= sub) {
                GAME_MEM8(0x8F) = 0x06;
            } else {
                GAME_MEM8((u16)(0x0401 + x)) = 0x80;
                GAME_MEM8((u16)(0x0405 + x)) = 0x00;
            }
        }
        store_pos();
        finish(r);
    }
}
