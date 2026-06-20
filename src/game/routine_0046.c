


#include "game_memory.h"
#include "routine_context.h"

void routine_0075(RoutineContext *r);

void routine_0046(RoutineContext *r)
{
    int x;
    GAME_MEM8(0x0180) = 0x0F;
    GAME_MEM8(0x0181) = 0x0C;
    GAME_MEM8(0x0182) = 0x10;
    GAME_MEM8(0x0183) = 0x30;
    for (x = 0x1B; x >= 0; --x)
        GAME_MEM8((u16)(0x0184 + x)) = 0x0F;
    r->value = 0x0F;
    routine_0075(r);
}
