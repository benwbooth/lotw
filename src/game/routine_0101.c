




#include "game_memory.h"
#include "routine_context.h"

void routine_0102(RoutineContext *r);

void routine_0101(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x09);
    u8 full = r->index;
    GAME_MEM8((u16)(0x0259 + x)) = full;
    GAME_MEM8((u16)(0x025D + x)) = full;
    GAME_MEM8((u16)(0x0261 + x)) = full;
    GAME_MEM8((u16)(0x0265 + x)) = full;
    GAME_MEM8((u16)(0x0269 + x)) = full;
    {
        u8 empty = r->offset;
        GAME_MEM8((u16)(0x026D + x)) = empty;
        GAME_MEM8((u16)(0x0271 + x)) = empty;
        GAME_MEM8((u16)(0x0275 + x)) = empty;
        GAME_MEM8((u16)(0x0279 + x)) = empty;
        GAME_MEM8((u16)(0x027D + x)) = empty;
    }

    routine_0102(r);

    {
        u8 y = r->offset;
        u8 xx = (u8)(GAME_MEM8(0x09) + 0x18);
        for (;;) {
            if (--y == 0) break;
            GAME_MEM8((u16)(0x0241 + xx)) -= 2;
            if (--y == 0) break;
            GAME_MEM8((u16)(0x0241 + xx)) -= 2;
            xx = (u8)(xx + 4);
        }
    }
    {
        u8 xx = (u8)(GAME_MEM8(0x09) + 0x2C);
        u8 y = GAME_MEM8(0x08);
        for (;;) {
            if (--y == 0) break;
            GAME_MEM8((u16)(0x0241 + xx)) -= 2;
            if (--y == 0) break;
            GAME_MEM8((u16)(0x0241 + xx)) -= 2;
            xx = (u8)(xx + 4);
        }
    }
}
