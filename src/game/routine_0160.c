







#include "game_memory.h"
#include "routine_context.h"

void routine_0160(RoutineContext *r)
{
    u8 x = 0x1E;
    u8 a;
    GAME_MEM8(0x8F) = 0x13;
    a = GAME_MEM8(0x88);
    if (a != 0) {
        a = GAME_MEM8(0x89);
        if (a != 0) {
            GAME_MEM8(0x8A) = x;
        }
        GAME_MEM8(0x89) = x;
    }
    GAME_MEM8(0x88) = x;
    r->value = a;
    r->index = x;
}
