









#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);

void routine_0052(RoutineContext *r)
{
    int x, y;

    for (x = 0x1F; x >= 0; x--)
        GAME_MEM8((u16)(0x0342 + x)) = GAME_MEM8((u16)(0x0322 + x));

    GAME_MEM8(0x3A) = GAME_MEM8(0x0351);
    GAME_MEM8(0x3B) = GAME_MEM8(0x0361);

    for (x = 0x0E; x >= 0; x--) {
        GAME_MEM8(0x08) = (u8)x;
        r->value = 0x20; rng_update(r); x = GAME_MEM8(0x08);
        GAME_MEM8((u16)(0x0342 + x)) ^= r->value;
        r->value = 0x20; rng_update(r); x = GAME_MEM8(0x08);
        GAME_MEM8((u16)(0x0352 + x)) ^= r->value;
    }


    { u8 a = 0; for (x = 0x0E; x >= 0; x -= 2) {
        u8 c = GAME_MEM8((u16)(0x0352 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        GAME_MEM8((u16)(0x0352 + x)) = (u8)(c >> 1);
    } GAME_MEM8(0x038A) = a; }

    { u8 a = 0; for (x = 0x0E; x >= 0; x -= 2) {
        u8 c = GAME_MEM8((u16)(0x0342 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        GAME_MEM8((u16)(0x0342 + x)) = (u8)(c >> 1);
    } GAME_MEM8(0x0389) = a; }


    { u8 a = 0; for (x = 0x1F; x >= 0; x--) a = (u8)(a + GAME_MEM8((u16)(0x0342 + x)));
      if (a != GAME_MEM8(0x0389)) goto fail; }

    { u8 a = 0x0A; for (x = 0x1F; x >= 0; x--) a ^= GAME_MEM8((u16)(0x0342 + x));
      if (a != GAME_MEM8(0x038A)) goto fail; }


    { u8 a = 0; for (x = 0x0F; x >= 0; x -= 2) {
        u8 c = GAME_MEM8((u16)(0x0342 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        GAME_MEM8((u16)(0x0342 + x)) = (u8)(c >> 1);
    } GAME_MEM8(0x0320) = a; }

    { u8 a = 0; for (x = 0x0F; x >= 0; x -= 2) {
        u8 c = GAME_MEM8((u16)(0x0352 + x));
        a = (u8)((a >> 1) | ((c & 1) << 7));
        GAME_MEM8((u16)(0x0352 + x)) = (u8)(c >> 1);
    } GAME_MEM8(0x0321) = a; }


    x = 0x0F; y = 0x07;
    do {
        u8 hi = GAME_MEM8((u16)(0x0342 + x)); x--;
        u8 lo = GAME_MEM8((u16)(0x0342 + x)); x--;
        GAME_MEM8((u16)(0x0308 + y)) = (u8)((hi << 4) | lo);
        y--;
    } while (y >= 0);

    for (x = 0x0F; x >= 0; x--)
        GAME_MEM8((u16)(0x0310 + x)) = GAME_MEM8((u16)(0x0352 + x));

    r->carry = 0;
    return;
fail:
    GAME_MEM8(0x8F) = 0x1C;
    GAME_MEM8(0x90) = 0x1C;
    r->carry = 1;
}
