











#include "game_memory.h"
#include "routine_context.h"

void routine_0054(RoutineContext *r)
{
    u8 ctrl = GAME_MEM8(0x23);
    u8 mask = GAME_MEM8(0x24);
    int i;

    REG_W(0x2000, ctrl & 0x7B);
    GAME_MEM8(0x29) = 0x00;
    REG_W(0x2001, mask & 0xE7);
    REG_W(0x2006, 0x20);
    REG_W(0x2006, 0x00);

    for (i = 0; i < 0x100; i++) REG_W(0x2007, GAME_MEM8((u16)(0x9EC9 + i)));
    for (i = 0; i < 0x100; i++) REG_W(0x2007, GAME_MEM8((u16)(0x9FC9 + i)));
    for (i = 0; i < 0x100; i++) REG_W(0x2007, GAME_MEM8((u16)(0xA0C9 + i)));
    for (i = 0; i < 0x100; i++) REG_W(0x2007, GAME_MEM8((u16)(0xA1C9 + i)));

    GAME_MEM8(0x2A) = GAME_MEM8(0xA2E9);
    GAME_MEM8(0x2B) = GAME_MEM8(0xA2EA);

    GAME_MEM8(0x24) = mask;
    GAME_MEM8(0x23) = ctrl;
    REG_W(0x2000, ctrl);

    r->value = ctrl;
    r->index = 0;
}
