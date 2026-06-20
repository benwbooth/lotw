



#include "game_memory.h"
#include "routine_context.h"

void routine_0106(RoutineContext *r);

void routine_0076(RoutineContext *r)
{
    u8 saved_ctrl, saved_mask;
    int i;

    routine_0106(r);

    saved_ctrl = GAME_MEM8(0x23);
    REG_W(0x2000, saved_ctrl & 0x7B);
    GAME_MEM8(0x29) = 0x00;

    saved_mask = GAME_MEM8(0x24);
    REG_W(0x2001, saved_mask & 0xE7);

    REG_W(0x2006, 0x23);
    REG_W(0x2006, 0x20);
    for (i = 0; i < 0xA0; ++i)
        REG_W(0x2007, GAME_MEM8((u16)(0xFECB + i)));

    REG_W(0x2006, 0x23);
    REG_W(0x2006, 0xF0);
    for (i = 0; i < 0x10; ++i)
        REG_W(0x2007, 0x00);

    GAME_MEM8(0x29) += 1;

    GAME_MEM8(0x24) = saved_mask;
    GAME_MEM8(0x23) = saved_ctrl;
    REG_W(0x2000, saved_ctrl);

    r->value = saved_ctrl;
    r->offset = 0x00;
}
