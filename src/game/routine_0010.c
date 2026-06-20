



#include "game_memory.h"
#include "routine_context.h"

void routine_0010(RoutineContext *r)
{
    if (GAME_MEM8(0x0A) >= 0xA1) { r->carry = 1; return; }
    if (GAME_MEM8(0x0E) < 0xF1)  { r->carry = 0; return; }
    if (GAME_MEM8(0x0E) == 0x00) { r->carry = 0; return; }
    r->carry = 1;
}
