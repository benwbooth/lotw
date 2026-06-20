








#include "game_memory.h"
#include "routine_context.h"

void routine_0013(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x0F);
    u8 y = GAME_MEM8(0x0E);

    if (GAME_MEM8((u16)(0x0401 + y)) == 0)
        goto hide;
    if (GAME_MEM8((u16)(0x040E + y)) >= 0xBF)
        goto hide;

    {
        u8 attr = GAME_MEM8((u16)(0x0402 + y));
        GAME_MEM8((u16)(0x0202 + x)) = attr;
        GAME_MEM8((u16)(0x0206 + x)) = attr;
        if (attr & 0x40) {
            u8 t = GAME_MEM8((u16)(0x0400 + y));
            GAME_MEM8((u16)(0x0205 + x)) = t;
            GAME_MEM8((u16)(0x0201 + x)) = (u8)(t + 2);
        } else {
            u8 t = GAME_MEM8((u16)(0x0400 + y));
            GAME_MEM8((u16)(0x0201 + x)) = t;
            GAME_MEM8((u16)(0x0205 + x)) = (u8)(t + 2);
        }
    }
    {
        u8 px = GAME_MEM8((u16)(0x040C + y));
        GAME_MEM8((u16)(0x0203 + x)) = px;
        GAME_MEM8((u16)(0x0207 + x)) = (u8)(px + 8);
        {
            u8 py = (u8)(GAME_MEM8((u16)(0x040E + y)) + 0x2B);
            GAME_MEM8((u16)(0x0200 + x)) = py;
            GAME_MEM8((u16)(0x0204 + x)) = py;
        }
    }
    (void)r;
    return;

hide:
    GAME_MEM8((u16)(0x0200 + x)) = 0xEF;
    GAME_MEM8((u16)(0x0204 + x)) = 0xEF;
    (void)r;
}
