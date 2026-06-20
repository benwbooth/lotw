









#include "game_memory.h"
#include "routine_context.h"

void routine_0088(RoutineContext *r);
void routine_0123(RoutineContext *r);

void text_attr_build(RoutineContext *r)
{
    u16 p = (u16)(GAME_MEM8(0x77) | (GAME_MEM8(0x78) << 8));
    u8 carry_in = r->carry;
    u8 b;


    b = GAME_MEM8(p);
    GAME_MEM8(0x7A) = (u8)(b + 0xA0 + carry_in);
    GAME_MEM8(0x79) = 0;

    GAME_MEM8(0x2D) = GAME_MEM8((u16)(p + 1));

    GAME_MEM8(0x70) = GAME_MEM8((u16)(p + 2));

    GAME_MEM8(0x71) = GAME_MEM8((u16)(p + 3));

    GAME_MEM8(0x74) = GAME_MEM8((u16)(p + 4));

    GAME_MEM8(0x2A) = GAME_MEM8((u16)(p + 5));

    GAME_MEM8(0x2B) = GAME_MEM8((u16)(p + 6));


    {
        u8 ms_y = GAME_MEM8(0x48);
        u8 ms_x = GAME_MEM8(0x47);
        u8 idx = (u8)(((ms_y << 2) & 0x04) | ms_x);
        u8 a = GAME_MEM8((u16)(0x0300 + idx));
        u8 cnt = (u8)((ms_y >> 1) + 1);

        u8 c = 0;
        do {
            c = (u8)((a >> 7) & 1);
            a = (u8)(a << 1);
        } while (--cnt != 0);
        r->value = a;
        r->carry = c;
    }

    {
        u8 y = 0x07;
        u8 a;

        if (r->carry)
            a = GAME_MEM8((u16)(p + y));
        else
            a = 0;
        GAME_MEM8(0x04A1) = a;
        if (a != 0) {
            GAME_MEM8(0x04A2) = 0x01;
            y++;
            GAME_MEM8(0x04AD) = GAME_MEM8((u16)(p + y));
            GAME_MEM8(0x04AC) = 0x00;
            y++;
            GAME_MEM8(0x04AE) = GAME_MEM8((u16)(p + y));
            y++;
            b = GAME_MEM8((u16)(p + y));
            if (b == 0x17) {
                GAME_MEM8(0x04A1) = 0x19;
                GAME_MEM8(0x04A0) = 0xDD;
            } else {
                GAME_MEM8(0x04A0) = 0xE9;
            }
        }
    }


    {
        u8 x = GAME_MEM8(0x8E);
        u8 do_d02e = 1;
        if (x < 0x05) {

            u8 a = 0x00;
            u8 c = 1;
            int i = (int)x;
            do {
                u8 nc = (u8)((a >> 7) & 1);
                a = (u8)((a << 1) | c);
                c = nc;
                --i;
            } while (i >= 0);
            a = (u8)(a & GAME_MEM8((u16)(p + 0x15)));
            if (a != 0)
                do_d02e = 0;
        }
        if (do_d02e) {

            r->value = GAME_MEM8((u16)(p + 0x0B));
            routine_0123(r);
        }
    }


    GAME_MEM8(0x80) = GAME_MEM8((u16)(p + 0x10));
    GAME_MEM8(0x81) = GAME_MEM8((u16)(p + 0x11));
    GAME_MEM8(0x82) = GAME_MEM8((u16)(p + 0x12));
    GAME_MEM8(0x83) = GAME_MEM8((u16)(p + 0x13));
    GAME_MEM8(0x41) = GAME_MEM8((u16)(p + 0x14));
}
