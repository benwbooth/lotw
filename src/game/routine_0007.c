




#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r);
void routine_0011(RoutineContext *r);
void routine_0010(RoutineContext *r);
void routine_0009(RoutineContext *r);
void routine_0214(RoutineContext *r);

void routine_0007(RoutineContext *r)
{
    routine_0213(r);

    GAME_MEM8(0xEE) = (u8)(GAME_MEM8(0xEE) - 1);
    if (GAME_MEM8(0xEE) != 0) {
        routine_0011(r);
        routine_0010(r);
        if (r->carry) {
            GAME_MEM8(0xEE) = 0x00;

        } else {
            GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
            GAME_MEM8(0xFB) = GAME_MEM8(0x0A);

        }
    }


    if (GAME_MEM8(0xEE) != 0)
        routine_0009(r);
    routine_0214(r);
}
