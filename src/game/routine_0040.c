






#include "game_memory.h"
#include "routine_context.h"

void routine_0048(RoutineContext *r);
void routine_0042(RoutineContext *r);
void routine_0044(RoutineContext *r);

void routine_0040(RoutineContext *r)
{
    u16 ptr;
    u8 y;

    routine_0048(r);

    ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));




    {
        int i;
        for (i = 0, y = 0; i < 256; ++i, ++y) {
            u8 b = GAME_MEM8((u16)(ptr + y));
            if (b == 0x00) {
                r->carry = 1;
                return;
            }
            if (b == 0x0D) {
                routine_0042(r);
                r->value = 0x05;
                routine_0044(r);
                r->carry = 0;
                return;
            }
            GAME_MEM8(0x08) = b & 0x0F;
            GAME_MEM8((u16)(0x0140 + y)) = (u8)(((b & 0xF0) << 1) | GAME_MEM8(0x08));
        }
    }
}
