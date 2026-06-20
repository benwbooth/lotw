







#include "game_memory.h"
#include "routine_context.h"

void routine_0122(RoutineContext *r)
{
    u8 in = r->value;
    u8 x = (u8)(cur_character << 1);
    if (in >= 0x08)
        x++;
    u8 y = (u8)((in & 0x07) + 1);
    u8 a = GAME_MEM8((u16)(0xFFBB + x));
    do {
        a = (u8)(a << 1);
    } while (--y != 0);
    r->value = a;
}
