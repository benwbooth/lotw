


#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_fill_run(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x1A);
    u8 a = GAME_MEM8(0x18);
    do { REG_W(0x2007, a); } while (--x != 0);
    vblank_commit_tail(r);
}
