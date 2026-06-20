














#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r);
void routine_0214(RoutineContext *r);
void routine_0272(RoutineContext *r);
void routine_0090(RoutineContext *r);
void farcall_bank_09_r7(RoutineContext *r);

void routine_0271(RoutineContext *r)
{
    if (GAME_MEM8(0x0491) == 0)
        return;

    GAME_MEM8(0xE5) = 0x90;
    GAME_MEM8(0xE6) = 0x04;
    routine_0213(r);

    GAME_MEM8(0xF3) = (u8)(GAME_MEM8(0xF3) - 1);
    if (GAME_MEM8(0xF3) != 0) {
        routine_0272(r);
        return;
    }


    if ((GAME_MEM8(0xED) & 0x01) == 0) {

        if ((u8)((GAME_MEM8(0xFB) & 0x0F) | GAME_MEM8(0xF9)) != 0) {
            GAME_MEM8(0xF3) = (u8)(GAME_MEM8(0xF3) + 1);
            routine_0272(r);
            return;
        }

    }


    GAME_MEM8(0xEE) = 0x00;
    if (GAME_MEM8(0xF0) != 0) {
        u16 ptr;
        u8 diff;
        GAME_MEM8(0x0C) = GAME_MEM8(0xFA);
        GAME_MEM8(0x0D) = GAME_MEM8(0xFB);
        routine_0090(r);
        ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
        GAME_MEM8(ptr) = GAME_MEM8(0xF0);

        diff = (u8)(GAME_MEM8(0xFA) - GAME_MEM8(0x7C));

        if (diff < 0x11 || diff >= 0xFE) {
            u8 fa = GAME_MEM8(0xFA);
            GAME_MEM8(0x0C) = fa;
            GAME_MEM8(0x16) = (u8)((fa << 1) & 0x1F);
            GAME_MEM8(0x17) = (u8)((GAME_MEM8(0xFA) & 0x10) >> 2);
            GAME_MEM8(0x16) = (u8)(0x00 + GAME_MEM8(0x16));
            GAME_MEM8(0x17) = (u8)(0x20 + GAME_MEM8(0x17));
            farcall_bank_09_r7(r);
        }

    }


    routine_0214(r);
}
