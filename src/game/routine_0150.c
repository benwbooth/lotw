






#include "game_memory.h"
#include "routine_context.h"

void routine_0151(RoutineContext *r); void routine_0152(RoutineContext *r); void routine_0153(RoutineContext *r);
void routine_0155(RoutineContext *r); void routine_0156(RoutineContext *r); void routine_0158(RoutineContext *r);
void routine_0162(RoutineContext *r); void routine_0160(RoutineContext *r);
void routine_0089(RoutineContext *r); void routine_0138(RoutineContext *r);

void routine_0150(RoutineContext *r)
{
    u8 n = (u8)(r->value - 0x02);
    if (n >= 0x18)
        return;


    {
        u8 slot = r->index;
        GAME_MEM8((u16)(0x0401 + slot)) = 0x00;
        GAME_MEM8((u16)(0x0406 + slot)) = 0xF0;
    }
    {
        u8 oam = (u8)((GAME_MEM8(0x08) << 3) | 0x80);
        GAME_MEM8((u16)(0x0200 + oam)) = 0xEF;
        GAME_MEM8((u16)(0x0204 + oam)) = 0xEF;
        r->index = oam;
    }

    if (n < 0x08) {

        static const u16 tbl[8] = { 0xDB26, 0xDB31, 0xDB3C, 0xDB52,
                                    0xDB5D, 0xDB71, 0xDBB7, 0xDB85 };
        GAME_MEM8(0x0C) = (u8)(tbl[n] & 0xFF);
        GAME_MEM8(0x0D) = (u8)(tbl[n] >> 8);
        r->value = (u8)(n << 1);
        r->index = r->value;
        switch (n) {
            case 0: routine_0151(r); break;
            case 1: routine_0152(r); break;
            case 2: routine_0153(r); break;
            case 3: routine_0155(r); break;
            case 4: routine_0156(r); break;
            case 5: routine_0158(r); break;
            case 6: routine_0162(r); break;
            case 7: routine_0160(r); break;
        }
        return;
    }


    {
        u8 x = (u8)(n - 0x08);
        if (GAME_MEM8((u16)(0x60 + x)) >= 0x0B) {
            GAME_MEM8(0x8F) = 0x1D;
            return;
        }
        GAME_MEM8((u16)(0x60 + x))++;
        GAME_MEM8(0x8F) = 0x13;
        if (x == 0x0E) {
            routine_0089(r);
            routine_0138(r);
        }
    }
}
