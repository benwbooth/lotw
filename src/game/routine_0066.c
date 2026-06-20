









#include "game_memory.h"
#include "routine_context.h"

void routine_0066(RoutineContext *r)
{
    u8 ctrl = GAME_MEM8(0x23);
    u8 mask = GAME_MEM8(0x24);
    int i;

    REG_W(0x2000, ctrl & 0x7B);
    GAME_MEM8(0x29) = 0x00;
    REG_W(0x2001, mask & 0xE7);
    REG_W(0x2006, 0x20);
    REG_W(0x2006, 0x00);

    for (i = 0; i < 5 * 0xC0; i++) REG_W(0x2007, 0xC0);
    for (i = 0; i < 0x40; i++)     REG_W(0x2007, 0x00);
    for (i = 0; i < 5 * 0xC0; i++) REG_W(0x2007, 0xC0);
    for (i = 0; i < 0x40; i++)     REG_W(0x2007, 0x00);

    GAME_MEM8(0x24) = mask;
    GAME_MEM8(0x23) = ctrl;
    REG_W(0x2000, ctrl);

    r->value = ctrl;
    r->index = 0;
    r->offset = 0;
}
