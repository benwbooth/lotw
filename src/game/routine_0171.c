









#include "game_memory.h"
#include "routine_context.h"

void farcall_bank_09_r7(RoutineContext *r);

void routine_0171(RoutineContext *r)
{
    u8 fa = GAME_MEM8(0xFA);

    GAME_MEM8(0x0C) = fa;
    GAME_MEM8(0x16) = (u8)((fa << 1) & 0x1F);
    GAME_MEM8(0x17) = (u8)((GAME_MEM8(0xFA) & 0x10) >> 2);

    GAME_MEM8(0x16) = (u8)(0x00 + GAME_MEM8(0x16));
    GAME_MEM8(0x17) = (u8)(0x20 + GAME_MEM8(0x17));

    farcall_bank_09_r7(r);
}
