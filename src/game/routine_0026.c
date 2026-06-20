




#include "game_memory.h"
#include "routine_context.h"

void routine_0026(RoutineContext *r)
{
    if (GAME_MEM8(0x85) != 0) {
        if ((GAME_MEM8(0x84) & 0x01) == 0) {
            GAME_MEM8(0x0210) = 0xEF;
            GAME_MEM8(0x0214) = 0xEF;
            return;
        }
    }


    GAME_MEM8(0x0210) = (u8)(GAME_MEM8(0x45) + 0x2B);
    GAME_MEM8(0x0214) = (u8)(GAME_MEM8(0x45) + 0x2B);
    GAME_MEM8(0x0213) = GAME_MEM8(0x43);
    GAME_MEM8(0x0217) = (u8)(GAME_MEM8(0x43) + 0x08);
    GAME_MEM8(0x0212) = (u8)(GAME_MEM8(0x57) | 0x20);
    GAME_MEM8(0x0216) = (u8)(GAME_MEM8(0x57) | 0x20);

    if (GAME_MEM8(0x57) & 0x40) {
        r->index = GAME_MEM8(0x56);
        GAME_MEM8(0x0215) = r->index;
        r->index = (u8)(r->index + 2);
        GAME_MEM8(0x0211) = r->index;
    } else {
        r->index = GAME_MEM8(0x56);
        GAME_MEM8(0x0211) = r->index;
        r->index = (u8)(r->index + 2);
        GAME_MEM8(0x0215) = r->index;
    }
}
