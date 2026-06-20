













#include "game_memory.h"
#include "routine_context.h"

void routine_0061(RoutineContext *r)
{
    u8 a, x, world_x;

    if (GAME_MEM8(0x85) != 0 && (GAME_MEM8(0x84) & 0x01) == 0) {
        GAME_MEM8(0x0210) = 0xEF;
        GAME_MEM8(0x0214) = 0xEF;
        return;
    }


    a = (u8)(GAME_MEM8(0x45) + 0x2B);
    GAME_MEM8(0x0210) = a;
    GAME_MEM8(0x0214) = a;


    world_x = (u8)((GAME_MEM8(0x7C) << 4) | GAME_MEM8(0x7B));
    GAME_MEM8(0x08) = world_x;


    a = (u8)((GAME_MEM8(0x44) << 4) | GAME_MEM8(0x43));
    a = (u8)(a - world_x);
    GAME_MEM8(0x0213) = a;
    GAME_MEM8(0x0217) = (u8)(a + 0x08);


    GAME_MEM8(0x0212) = GAME_MEM8(0x57);
    GAME_MEM8(0x0216) = GAME_MEM8(0x57);


    x = GAME_MEM8(0x56);
    if (GAME_MEM8(0x57) & 0x40) {
        GAME_MEM8(0x0215) = x;
        GAME_MEM8(0x0211) = (u8)(x + 2);
    } else {
        GAME_MEM8(0x0211) = x;
        GAME_MEM8(0x0215) = (u8)(x + 2);
    }
}
