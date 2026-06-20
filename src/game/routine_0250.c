



















#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0111(RoutineContext *r);
void routine_0252(RoutineContext *r);

static void f179_fail(RoutineContext *r)
{

    if (GAME_MEM8(0xF0) >= 0x0C)
        GAME_MEM8(0xF1) = (u8)(GAME_MEM8(0xF0) - 0x04);

    GAME_MEM8(0xF0) = 0x00;
    r->carry = 1;
}

static void f179_ok(RoutineContext *r)
{
    GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
    r->carry = 0;
}

void routine_0250(RoutineContext *r)
{
    u8 x, y;

    if (GAME_MEM8(0xF1) != 0) { f179_fail(r); return; }

    GAME_MEM8(0x0C) = GAME_MEM8(0xFA);
    GAME_MEM8(0x0F) = GAME_MEM8(0xFA);
    GAME_MEM8(0x0E) = GAME_MEM8(0xF9);

    x = GAME_MEM8(0xFB);
    y = (u8)(GAME_MEM8(0xEE) - 1);
    if (y == 0) {

        if (x >= 0xB0) { f179_ok(r); return; }
        GAME_MEM8(0x0D) = x;
        x = (u8)(x + 1);
        GAME_MEM8(0x0A) = x;
        routine_0111(r);
        if (r->carry) { f179_fail(r); return; }
    } else {
        if (x != 0xEF) {

        } else {
            x = GAME_MEM8(0xFC);
        }
        GAME_MEM8(0x0D) = x;
    }


    routine_0090(r);

    if (GAME_MEM8(0xF9) == 0) {
        u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
        if ((GAME_MEM8(ptr) & 0x3F) == 0) { f179_fail(r); return; }
        if ((GAME_MEM8((u16)(ptr + 1)) & 0x3F) == 0) { f179_fail(r); return; }
    }


    r->offset = 0x01;
    routine_0252(r);
    if (r->carry) { f179_fail(r); return; }

    if (GAME_MEM8(0xF9) == 0) { f179_ok(r); return; }

    r->offset = 0x0D;
    routine_0252(r);
    if (r->carry) { f179_fail(r); return; }

    f179_ok(r);
}
