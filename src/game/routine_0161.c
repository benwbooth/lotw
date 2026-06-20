









#include "game_memory.h"
#include "routine_context.h"

void routine_0161(RoutineContext *r)
{
    u8 x = 0x3C;
    u8 a;
    GAME_MEM8(0x8F) = 0x13;
    a = GAME_MEM8(0x88);
    if (a != 0) {
        a = GAME_MEM8(0x89);
        if (a != 0) {
            a = GAME_MEM8(0x8A);
            if (a != 0) {
                GAME_MEM8(0x8B) = x;
            }
            GAME_MEM8(0x8A) = x;
        }
        GAME_MEM8(0x89) = x;
    }
    GAME_MEM8(0x88) = x;
    r->value = a;
    r->index = x;
}
