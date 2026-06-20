







#include "game_memory.h"
#include "routine_context.h"

void routine_0088(RoutineContext *r)
{
    u8 ms_y = GAME_MEM8(0x48);
    u8 ms_x = GAME_MEM8(0x47);
    u8 idx = (u8)(((ms_y << 2) & 0x04) | ms_x);
    u8 a = GAME_MEM8((u16)(0x0300 + idx));
    u8 cnt = (u8)((ms_y >> 1) + 1);
    do {
        a = (u8)(a << 1);
    } while (--cnt != 0);
    r->value = a;
}
