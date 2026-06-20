



#include "game_memory.h"
#include "routine_context.h"

void routine_0096(RoutineContext *r);

void routine_0208(RoutineContext *r)
{
    GAME_MEM8(0x08) = r->value;
    u16 res = (u16)gold - (u16)GAME_MEM8(0x08);
    r->value = (u8)res;
    if (res & 0x100) {

        r->carry = 0;
        return;
    }
    gold = r->value;
    routine_0096(r);
    r->carry = 1;
}
