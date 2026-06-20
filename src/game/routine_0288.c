










#include "game_memory.h"
#include "routine_context.h"

void routine_0284(RoutineContext *r);

void routine_0288(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x02);
    u8 idx = GAME_MEM8((u8)(0x9B + x));
    GAME_MEM8((u8)(0x9D + x)) = GAME_MEM8((u16)(0xFDCC + idx));

    {
        u8 v = GAME_MEM8((u8)(0x9C + x));
        u8 a = (u8)(v + GAME_MEM8((u8)(0x9F + x)));
        if (v & 0x80) {
            if (a >= 0x10)
                a = 0x00;
        } else {
            if (a >= 0x10)
                a = 0x0F;
        }
        GAME_MEM8((u8)(0x9F + x)) = a;
        GAME_MEM8(0x00) = a;
    }

    r->offset = GAME_MEM8((u8)(0xA0 + x));
    routine_0284(r);

    {
        u8 result = (u8)((GAME_MEM8((u8)(0x99 + x)) & 0xC0) | GAME_MEM8(0x00) | 0x30);
        r->value = result;
    }
}
