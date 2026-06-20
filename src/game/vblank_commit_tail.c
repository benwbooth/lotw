


#include "game_memory.h"
#include "routine_context.h"
void ppu_commit_banks(RoutineContext *r); void statusbar_split(RoutineContext *r); void frame_counters(RoutineContext *r);
void vblank_commit_tail(RoutineContext *r)
{
    ppu_commit_banks(r);

    statusbar_split(r);
    if (GAME_MEM8(0x36) != 0)
        GAME_MEM8(0x36)--;
    frame_counters(r);
    REG_W(0x8000, GAME_MEM8(0x25));
}
