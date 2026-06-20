











#include "game_memory.h"
#include "routine_context.h"

void routine_0203(RoutineContext *r);

void routine_0249(RoutineContext *r)
{
    if (GAME_MEM8(0x85) != 0)
        return;
    if ((u8)(GAME_MEM8(0xEE) - 1) != 0)
        return;

    if (GAME_MEM8(0x2D) >= 0x30) {
        if (GAME_MEM8(0xE3) != 0) {
            u8 x = GAME_MEM8(0x55);
            if (GAME_MEM8((u16)(0x0051 + x)) == 0x0A) {
                GAME_MEM8(0x8F) = 0x01;
                return;
            }

        }

    } else {

        if (GAME_MEM8(0x40) == 0x04)
            return;

    }


    r->value = GAME_MEM8(0xF8);
    routine_0203(r);
    GAME_MEM8(0x8F) = 0x21;
    GAME_MEM8(0x90) = 0x01;
    GAME_MEM8(0x85) = 0x01;
    GAME_MEM8(0xEF) = (u8)(GAME_MEM8(0xEF) & 0xDF);
}
