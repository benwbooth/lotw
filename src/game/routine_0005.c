

























#include "game_memory.h"
#include "routine_context.h"

void routine_0006(RoutineContext *r); void routine_0007(RoutineContext *r); void routine_0012(RoutineContext *r);

void routine_0005(RoutineContext *r)
{
    GAME_MEM8(0xE3) = 0x01;
    GAME_MEM8(0xE5) = 0x10;
    GAME_MEM8(0xE6) = 0x04;

    do {
        u16 p = (u16)(GAME_MEM8(0xE5) | (GAME_MEM8(0xE6) << 8));
        if (GAME_MEM8((u16)(p + 1)) != 0) {
            routine_0007(r);
        } else if ((GAME_MEM8(0x20) & 0x40) && !(GAME_MEM8(0xFD) & 0x40)) {
            routine_0006(r);
        }

        GAME_MEM8(0xE3) = (u8)(GAME_MEM8(0xE3) + 1);
        {
            u16 np = (u16)(GAME_MEM8(0xE5) + 0x10);
            GAME_MEM8(0xE5) = (u8)np;
            GAME_MEM8(0xE6) = (u8)(GAME_MEM8(0xE6) + (np >> 8));
        }
    } while (GAME_MEM8(0xE3) < 0x04);

    routine_0012(r);
}
