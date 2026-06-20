
















#include "game_memory.h"
#include "routine_context.h"

void farcall_bank_09_r7(RoutineContext *r);

void routine_0003(RoutineContext *r)
{
    if (GAME_MEM8(0xFA) == 0) {
        GAME_MEM8(0x16) = 0x0E;
        GAME_MEM8(0x17) = 0x20;
        GAME_MEM8(0x17) = (u8)(((u8)((GAME_MEM8(0x1D) ^ 0x01) << 2)) | GAME_MEM8(0x17));
        GAME_MEM8(0xF9) = (u8)(((u8)((((GAME_MEM8(0x1D) ^ 0x01) << 4) + 0x07)) ) | GAME_MEM8(0x7C));
        GAME_MEM8(0xFA) = 0x09;
    }

    GAME_MEM8(0x0C) = GAME_MEM8(0xF9);
    farcall_bank_09_r7(r);

    GAME_MEM8(0x16) = (u8)(GAME_MEM8(0x16) + 1);
    GAME_MEM8(0x16) = (u8)(GAME_MEM8(0x16) + 1);
    GAME_MEM8(0xF9) = (u8)(GAME_MEM8(0xF9) + 1);

    GAME_MEM8(0xFA) = (u8)(GAME_MEM8(0xFA) - 1);
    if (GAME_MEM8(0xFA) == 0)
        GAME_MEM8(0x1D) ^= 0x01;
}
