












#include "game_memory.h"
#include "routine_context.h"

void routine_0064(RoutineContext *r);

void routine_0063(RoutineContext *r)
{
    u8 x, y;
    GAME_MEM8(0x0A) = 0x10;
    x = GAME_MEM8(0x3F);
    y = GAME_MEM8(0x3E);

    do {
        r->index = x;
        r->offset = y;
        routine_0064(r);

        x = (u8)(((u8)(x + 0x08)) | 0x80);
        y = (u8)(y + 0x30);

        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x0A) - 1);
    } while (GAME_MEM8(0x0A) != 0);

    GAME_MEM8(0x3F) = (u8)(((u8)(x + 0x38)) | 0x80);
    GAME_MEM8(0x3E) = (u8)(y + 0x10);
}
