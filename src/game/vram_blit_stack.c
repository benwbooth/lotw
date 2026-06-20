



#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_blit_stack(RoutineContext *r)
{




    for (int i = 0; i < 0x40; i++)
        REG_W(0x2007, GAME_MEM8((u16)(0x0100 + i)));
    vblank_commit_tail(r);
}
