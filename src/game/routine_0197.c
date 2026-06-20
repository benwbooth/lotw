



















#include "game_memory.h"
#include "routine_context.h"

void routine_0197(RoutineContext *r)
{
    int x;
    u8 y = 0x10;
    u8 a;

    GAME_MEM8(0x08) = 0x58;

    for (x = 2; x >= 0; --x) {
        u8 item = GAME_MEM8((u16)(0x0051 + x));
        if (item & 0x80) {
            a = 0xEF;
        } else {
            u8 t = (u8)(((u8)(item << 2)) + 0xA1);
            GAME_MEM8((u16)(0x0241 + y)) = t;
            GAME_MEM8((u16)(0x0245 + y)) = (u8)(t + 0x02);
            a = 0xBB;
        }
        GAME_MEM8((u16)(0x0240 + y)) = a;
        GAME_MEM8((u16)(0x0244 + y)) = a;

        GAME_MEM8((u16)(0x0243 + y)) = GAME_MEM8(0x08);
        GAME_MEM8((u16)(0x0247 + y)) = (u8)(GAME_MEM8(0x08) + 0x08);
        GAME_MEM8(0x08) = (u8)((u8)(GAME_MEM8(0x08) + 0x08) - 0x28);

        GAME_MEM8((u16)(0x0242 + y)) = 0x01;
        GAME_MEM8((u16)(0x0246 + y)) = 0x01;

        y = (u8)(y - 0x08);
    }
    r->index = 0xFF;
    r->offset = y;
}
