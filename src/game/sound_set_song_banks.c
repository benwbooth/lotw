




#include "game_memory.h"
#include "routine_context.h"

void sound_set_song_banks(RoutineContext *r)
{
    REG_W(0x8000, 0x06);
    REG_W(0x8001, GAME_MEM8(0x34));
    REG_W(0x8000, 0x07);
    REG_W(0x8001, GAME_MEM8(0x35));
    (void)r;
}
