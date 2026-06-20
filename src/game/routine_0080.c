











#include "game_memory.h"
#include "routine_context.h"

void routine_0106(RoutineContext *r);
void farcall_bank_09_r7(RoutineContext *r);

void routine_0080(RoutineContext *r)
{
    u8 sx;

    routine_0106(r);

    sx = GAME_MEM8(0x7C);
    GAME_MEM8(0x16) = (u8)((sx << 1) & 0x1F);
    GAME_MEM8(0x17) = (u8)((sx & 0x10) >> 2);
    GAME_MEM8(0x16) = (u8)(0x00 + GAME_MEM8(0x16));
    GAME_MEM8(0x17) = (u8)(0x20 + GAME_MEM8(0x17));

    GAME_MEM8(0x08) = sx;
    GAME_MEM8(0x09) = 0x10;

    do {
        GAME_MEM8(0x0C) = GAME_MEM8(0x08);
        farcall_bank_09_r7(r);
        GAME_MEM8(0x16) = (u8)(GAME_MEM8(0x16) + 2);
        if (GAME_MEM8(0x16) & 0x20) {
            GAME_MEM8(0x16) = 0x00;
            GAME_MEM8(0x17) ^= 0x04;
        }
        GAME_MEM8(0x08) = (u8)(GAME_MEM8(0x08) + 1);
        GAME_MEM8(0x09) = (u8)(GAME_MEM8(0x09) - 1);
    } while (GAME_MEM8(0x09) != 0);
}
