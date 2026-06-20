

#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_poke2(RoutineContext *r)
{
    REG_W(0x2007, GAME_MEM8(0x19));
    REG_W(0x2007, GAME_MEM8(0x18));
    vblank_commit_tail(r);
}
