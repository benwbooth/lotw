




#include "game_memory.h"
#include "routine_context.h"

void routine_0053(RoutineContext *r)
{
    u8 x = 0x40;
    do {
        GAME_MEM8((u16)(0x00 + x)) = GAME_MEM8((u16)(0x9B9F + x));
        x++;
    } while (x != 0x8C);

    for (x = 0x1F; (x & 0x80) == 0; x--)
        GAME_MEM8((u16)(0x0180 + x)) = 0x0F;

    r->value = 0x0F;
    r->index = 0xFF;
}
