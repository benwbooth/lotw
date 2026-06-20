




#include "game_memory.h"
#include "routine_context.h"

void sound_restore_game_banks(RoutineContext *r)
{
    REG_W(0x8000, 0x06);
    REG_W(0x8001, GAME_MEM8(0x30));
    REG_W(0x8000, 0x07);
    REG_W(0x8001, GAME_MEM8(0x31));
    (void)r;
}
