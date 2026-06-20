










#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);

void routine_0051(RoutineContext *r)
{
    int x, y;


    x = 0x0F; y = 0x07;
    do {
        u8 b = GAME_MEM8((u16)(0x0308 + y));
        GAME_MEM8((u16)(0x0322 + x)) = (u8)(b >> 4);
        x--;
        GAME_MEM8((u16)(0x0322 + x)) = (u8)(b & 0x0F);
        x--; y--;
    } while (y >= 0);


    for (x = 0x0F; x >= 0; x--)
        GAME_MEM8((u16)(0x0332 + x)) = (u8)(GAME_MEM8((u16)(0x0310 + x)) & 0x0F);


    {
        u8 a = GAME_MEM8(0x0320);
        for (x = 0x0F; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = GAME_MEM8((u16)(0x0322 + x));
            GAME_MEM8((u16)(0x0322 + x)) = (u8)((c << 1) | cin);
        }
    }

    {
        u8 a = GAME_MEM8(0x0321);
        for (x = 0x0F; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = GAME_MEM8((u16)(0x0332 + x));
            GAME_MEM8((u16)(0x0332 + x)) = (u8)((c << 1) | cin);
        }
    }


    {
        u8 a = 0x00;
        for (x = 0x1F; x >= 0; x--)
            a = (u8)(a + GAME_MEM8((u16)(0x0322 + x)));
        GAME_MEM8(0x0389) = a;
    }

    {
        u8 a = 0x0A;
        for (x = 0x1F; x >= 0; x--)
            a = (u8)(a ^ GAME_MEM8((u16)(0x0322 + x)));
        GAME_MEM8(0x038A) = a;
    }


    {
        u8 a = GAME_MEM8(0x0389);
        for (x = 0x0E; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = GAME_MEM8((u16)(0x0322 + x));
            GAME_MEM8((u16)(0x0322 + x)) = (u8)((c << 1) | cin);
        }
    }

    {
        u8 a = GAME_MEM8(0x038A);
        for (x = 0x0E; x >= 0; x -= 2) {
            u8 cin = (u8)(a & 1); a >>= 1;
            u8 c = GAME_MEM8((u16)(0x0332 + x));
            GAME_MEM8((u16)(0x0332 + x)) = (u8)((c << 1) | cin);
        }
    }


    GAME_MEM8(0x3A) = GAME_MEM8(0x0331);
    GAME_MEM8(0x3B) = GAME_MEM8(0x0341);
    for (x = 0x0E; x >= 0; x--) {
        GAME_MEM8(0x08) = (u8)x;
        r->value = 0x20; rng_update(r);
        x = GAME_MEM8(0x08);
        GAME_MEM8((u16)(0x0322 + x)) = (u8)(r->value ^ GAME_MEM8((u16)(0x0322 + x)));
        r->value = 0x20; rng_update(r);
        x = GAME_MEM8(0x08);
        GAME_MEM8((u16)(0x0332 + x)) = (u8)(r->value ^ GAME_MEM8((u16)(0x0332 + x)));
    }
}
