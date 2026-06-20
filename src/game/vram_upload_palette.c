


#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_upload_palette(RoutineContext *r)
{
    int y;
    REG_W(0x2006, 0x3F); REG_W(0x2006, 0x00);
    for (y = 0; y < 0x20; y++)
        REG_W(0x2007, GAME_MEM8((u16)(0x0180 + y)));
    REG_W(0x2006, 0x3F); REG_W(0x2006, 0x00);
    REG_W(0x2006, 0x00); REG_W(0x2006, 0x00);
    vblank_commit_tail(r);
}
