


#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_upload_hud(RoutineContext *r)
{
    int x;
    REG_W(0x2000, (u8)(GAME_MEM8(0x23) | 0x04));
    for (x = 0x17; x >= 0; x--)
        REG_W(0x2007, GAME_MEM8((u16)(0x0140 + x)));
    REG_W(0x2006, GAME_MEM8(0x17));
    REG_W(0x2006, (u8)(GAME_MEM8(0x16) + 1));
    for (x = 0x17; x >= 0; x--)
        REG_W(0x2007, GAME_MEM8((u16)(0x0158 + x)));
    for (x = 0x0A; x >= 0; x -= 2) {
        REG_W(0x2006, GAME_MEM8(0x19)); REG_W(0x2006, GAME_MEM8((u16)(0x0170 + x)));
        (void)REG_R(0x2007);
        {
            u8 v = (u8)((REG_R(0x2007) & GAME_MEM8(0x18)) | GAME_MEM8((u16)(0x0171 + x)));
            REG_W(0x2006, GAME_MEM8(0x19)); REG_W(0x2006, GAME_MEM8((u16)(0x0170 + x)));
            REG_W(0x2007, v);
        }
    }
    vblank_commit_tail(r);
}
