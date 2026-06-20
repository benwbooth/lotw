







#include "game_memory.h"
#include "routine_context.h"

void routine_0133(RoutineContext *r); void routine_0134(RoutineContext *r); void routine_0154(RoutineContext *r);
void routine_0155(RoutineContext *r); void routine_0157(RoutineContext *r); void routine_0159(RoutineContext *r);
void routine_0162(RoutineContext *r); void routine_0161(RoutineContext *r);
void routine_0089(RoutineContext *r); void routine_0138(RoutineContext *r);

void routine_0149(RoutineContext *r)
{
    u8 n = (u8)(r->value - 0x02);
    GAME_MEM8(0x04A1) = 0x00;

    if (n >= 0x18) {
        GAME_MEM8(0x8F) = 0x06;
        return;
    }
    if (n < 0x08) {


        static const u16 tbl[8] = { 0xD16A, 0xD199, 0xDB47, 0xDB52,
                                    0xDB66, 0xDB7B, 0xDBB7, 0xDB9B };
        GAME_MEM8(0x0C) = (u8)(tbl[n] & 0xFF);
        GAME_MEM8(0x0D) = (u8)(tbl[n] >> 8);
        r->value = (u8)(n << 1);
        r->index = r->value;
        switch (n) {
            case 0: routine_0133(r); break;
            case 1: routine_0134(r); break;
            case 2: routine_0154(r); break;
            case 3: routine_0155(r); break;
            case 4: routine_0157(r); break;
            case 5: routine_0159(r); break;
            case 6: routine_0162(r); break;
            case 7: routine_0161(r); break;
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
