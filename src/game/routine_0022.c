



























#include "game_memory.h"
#include "routine_context.h"

void routine_0027(RoutineContext *r); void routine_0028(RoutineContext *r); void routine_0024(RoutineContext *r);
void routine_0025(RoutineContext *r); void routine_0026(RoutineContext *r);

void routine_0022(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x4F);
    if (x == 0) {
        if (GAME_MEM8(0x22) != 0)
            return;

        GAME_MEM8(0x8F) = 0x1B;
        GAME_MEM8(0x4F) = GAME_MEM8(0x5C);
    }


    GAME_MEM8(0x22) = 0x01;
    GAME_MEM8(0x4F) = (u8)(GAME_MEM8(0x4F) - 1);
    GAME_MEM8(0x4B) = (u8)(((u8)(x >> 2) ^ 0xFF) + 1);

    routine_0027(r);
    if (r->carry) {
        GAME_MEM8(0x49) = 0x00;
        routine_0027(r);
    }

    if (!r->carry) {
        GAME_MEM8(0x43) = GAME_MEM8(0x0E);
        GAME_MEM8(0x45) = GAME_MEM8(0x0A);
        routine_0028(r);
    } else {
        GAME_MEM8(0x4F) = 0x00;
        GAME_MEM8(0x4E) = 0x00;
        routine_0028(r);
    }


    routine_0024(r);
    routine_0025(r);
    routine_0026(r);
}
