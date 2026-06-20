



#include "game_memory.h"
#include "routine_context.h"

void routine_0093(RoutineContext *r);

void routine_0203(RoutineContext *r)
{
    u8 dmg = r->value;
    u16 res;
    u8 carry;

    GAME_MEM8(0x08) = dmg;
    res = (u16)health - dmg;
    health = (u8)res;
    carry = (res < 0x100);
    if (carry == 0)
        health = 0x00;
    routine_0093(r);
    r->carry = carry;
}
