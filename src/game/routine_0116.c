







#include "game_memory.h"
#include "routine_context.h"

void routine_0116(RoutineContext *r)
{
    if (GAME_MEM8(0x0A) >= 0xB0) {
        r->carry = 1;
        return;
    }
    if (GAME_MEM8(0x0F) < 0x3F) {
        r->carry = 0;
        return;
    }
    if (GAME_MEM8(0x0E) == 0) {
        r->carry = 0;
        return;
    }
    r->carry = 1;
}
