

#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_copy_indirect(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x1A);
    u16 src = (u16)(GAME_MEM8(0x18) | (GAME_MEM8(0x19) << 8));
    u8 y = 0;
    do { REG_W(0x2007, GAME_MEM8((u16)(src + y))); y++; } while (--x != 0);
    vblank_commit_tail(r);
}
