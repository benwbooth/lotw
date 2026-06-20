



#include "game_memory.h"
#include "routine_context.h"

void routine_0014(RoutineContext *r)
{
    (void)r;
    u8 c = (u8)(GAME_MEM8(0x3E) - 1);
    if (c & 0x80)
        c = 0x07;
    GAME_MEM8(0x3E) = c;

    u8 x = (u8)(c << 2);
    u16 base = (c & 0x06) ? 0x0280 : 0x0210;

    GAME_MEM8(0x0200) = GAME_MEM8((u16)(base + 0 + x));
    GAME_MEM8(0x0201) = GAME_MEM8((u16)(base + 1 + x));
    GAME_MEM8(0x0202) = GAME_MEM8((u16)(base + 2 + x));
    GAME_MEM8(0x0203) = GAME_MEM8((u16)(base + 3 + x));
    GAME_MEM8((u16)(base + x)) = 0xEF;
}
