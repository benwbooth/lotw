













#include "game_memory.h"
#include "routine_context.h"

void main_init(RoutineContext *r);

void reset(RoutineContext *r)
{

    REG_W(0x8000, 0x00);
    REG_W(0xA001, 0x00);
    REG_W(0xE000, 0x00);
    main_init(r);
}
