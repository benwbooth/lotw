


















#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0114(RoutineContext *r);
void routine_0252(RoutineContext *r);

static void bail(void)
{
    u8 f0 = GAME_MEM8(0xF0);
    if (f0 >= 0x0C)
        GAME_MEM8(0xF1) = (u8)(f0 - 0x04);
    GAME_MEM8(0xF0) = 0x00;
}

void routine_0251(RoutineContext *r)
{
    if (GAME_MEM8(0xF1) != 0) { bail(); return; }

    GAME_MEM8(0x0C) = GAME_MEM8(0xFA);
    GAME_MEM8(0x0F) = GAME_MEM8(0xFA);
    GAME_MEM8(0x0E) = GAME_MEM8(0xF9);
    GAME_MEM8(0x0D) = GAME_MEM8(0xFB);
    GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0xFB) + 1);

    routine_0090(r);

    if (GAME_MEM8(0xFB) >= 0xA0) {
        GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
        return;
    }

    routine_0114(r);
    if (r->carry) { bail(); return; }

    r->offset = 0x02; routine_0252(r);
    if (r->carry) { bail(); return; }

    r->offset = 0x0E; routine_0252(r);
    if (r->carry) { bail(); return; }

    if (GAME_MEM8(0xF9) != 0) {
        r->offset = 0x1A; routine_0252(r);
        if (r->carry) { bail(); return; }
    }


    GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
}
