







#include "game_memory.h"
#include "routine_context.h"

void inc16_95(RoutineContext *r);

void routine_0283(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x02);
    u16 ptr = (u16)(GAME_MEM8((u8)(0x95 + x)) | (GAME_MEM8((u8)(0x96 + x)) << 8));
    u8 note = GAME_MEM8(ptr);

    inc16_95(r);

    {
        u8 y = note;
        u8 idx = (u8)((note & 0x0F) << 1);
        u8 lo = GAME_MEM8((u16)(0xFDB1 + idx));
        u8 hi = GAME_MEM8((u16)(0xFDB2 + idx));

        x = GAME_MEM8(0x02);
        {
            u16 sub = (u16)((u16)lo - GAME_MEM8((u8)(0xA1 + x)));
            lo = (u8)sub;
            if (sub & 0x100)
                hi = (u8)(hi - 1);
        }

        {
            u8 cnt = (u8)(y >> 4);
            while (cnt != 0) {
                u8 newcarry = (u8)(hi & 1);
                hi = (u8)(hi >> 1);
                lo = (u8)((lo >> 1) | (newcarry << 7));
                --cnt;
            }
        }

        GAME_MEM8(0x04) = lo;
        GAME_MEM8(0x05) = hi;
    }
}
