






#include "game_memory.h"
#include "routine_context.h"

void routine_0015(RoutineContext *r)
{
    u8 y, x, a, c;

    GAME_MEM8(0x09) = r->offset;
    x = (u8)((r->value & 0x0F) << 1);

    a = 0x00;
    y = r->offset;
    do {
        a = (u8)(a + GAME_MEM8((u16)(0xFE8B + x)));
        y--;
    } while (y != 0);
    GAME_MEM8(0xF5) = a;

    y = GAME_MEM8(0x09);
    a = 0x00;
    do {
        a = (u8)(a + GAME_MEM8((u16)(0xFE8C + x)));
        y--;
    } while (y != 0);
    GAME_MEM8(0xF7) = a;
    (void)c;
}
