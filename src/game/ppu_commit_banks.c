








#include "game_memory.h"
#include "routine_context.h"

void ppu_commit_banks(RoutineContext *r)
{
    int x;
    for (x = 7; x >= 0; x--) {
        REG_W(0x8000, (u8)x);
        REG_W(0x8001, GAME_MEM8((u16)(0x2A + x)));
    }
    r->index = 0xFF;
}
