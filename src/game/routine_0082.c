












#include "game_memory.h"
#include "routine_context.h"

void routine_0106(RoutineContext *r);
void farcall_bank_09_r7(RoutineContext *r);

void routine_0082(RoutineContext *r)
{
    u8 col;

    routine_0106(r);

    if (GAME_MEM8(0x7F) & 0x80)
        col = GAME_MEM8(0x7C);
    else
        col = (u8)(GAME_MEM8(0x7C) + 0x10);
    GAME_MEM8(0x0C) = col;

    GAME_MEM8(0x16) = (u8)((col << 1) & 0x1F);
    GAME_MEM8(0x17) = (u8)((col & 0x10) >> 2);
    GAME_MEM8(0x16) = (u8)(0x00 + GAME_MEM8(0x16));
    GAME_MEM8(0x17) = (u8)(0x20 + GAME_MEM8(0x17));

    farcall_bank_09_r7(r);
}
