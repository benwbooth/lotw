


















#include "game_memory.h"
#include "routine_context.h"

void routine_0042(RoutineContext *r);
void routine_0044(RoutineContext *r);
void routine_0048(RoutineContext *r);

void routine_0041(RoutineContext *r)
{
    u16 ptr;
    u8 y, c, lo;
    int guard;

    routine_0048(r);

    ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
    y = 0x00;




    for (guard = 0; guard < 256; guard++) {
        c = GAME_MEM8((u16)(ptr + y));
        if (c == 0x00) {
            r->carry = 1;
            return;
        }
        if (c == 0x0D) {
            u16 sum;
            y++;
            sum = (u16)(y + GAME_MEM8(0x0C));
            lo = (u8)sum;
            GAME_MEM8(0x0C) = lo;
            if (sum > 0xFF)
                GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0D) + 1);

            r->value = 0x08;
            routine_0042(r);
            r->value = 0x05;
            routine_0044(r);
            r->carry = 0;
            return;
        }
        {
            u8 lonib = c & 0x0F;
            u8 hi;
            u8 v;
            GAME_MEM8(0x08) = lonib;
            hi = (u8)((c & 0xF0) << 1);
            v = (u8)(hi | GAME_MEM8(0x08));
            v = (u8)(v + 0x10);
            GAME_MEM8((u16)(0x0140 + y)) = v;
        }
        y++;
    }

}
