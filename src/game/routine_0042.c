








#include "game_memory.h"
#include "routine_context.h"

void routine_0042(RoutineContext *r)
{
    u8 hi = 0x08;
    u8 a = GAME_MEM8(0x0A);
    u8 carry;

    carry = a >> 7;  a = (u8)(a << 1);  hi = (u8)((hi << 1) | carry);
    carry = a >> 7;  a = (u8)(a << 1);  hi = (u8)((hi << 1) | carry);

    GAME_MEM8(0x17) = hi;
    GAME_MEM8(0x16) = a;
    r->value = a;
}
