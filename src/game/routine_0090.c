






#include "game_memory.h"
#include "routine_context.h"

void routine_0091(RoutineContext *r);

void routine_0090(RoutineContext *r)
{
    u8 saved_0d = GAME_MEM8(0x0D);

    routine_0091(r);

    GAME_MEM8(0x11) = GAME_MEM8(0x0D);

    {
        u8 a = (u8)(saved_0d >> 4);
        u16 s = (u16)(a + GAME_MEM8(0x0C));
        GAME_MEM8(0x0C) = (u8)s;
        GAME_MEM8(0x10) = (u8)s;
        if (s & 0x100) {
            GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0D) + 1);
            GAME_MEM8(0x11) = (u8)(GAME_MEM8(0x11) + 1);
        }
    }

    GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0D) + 0x05);

    {
        u16 lo = (u16)(GAME_MEM8(0x10) + GAME_MEM8(0x75));
        u8 carry = (u8)(lo >> 8);
        GAME_MEM8(0x10) = (u8)lo;
        GAME_MEM8(0x11) = (u8)(GAME_MEM8(0x11) + GAME_MEM8(0x76) + carry);
    }
}
