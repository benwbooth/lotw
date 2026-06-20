











#include "game_memory.h"
#include "routine_context.h"

#define M(a) GAME_MEM8(a)

void routine_0079(RoutineContext *r)
{
    u8 ctrl_save = M(0x23);
    u8 v29_save  = M(0x29);
    u8 v24_save  = M(0x24);
    u8 c0c_save  = M(0x0C);
    u8 c0d_save  = M(0x0D);
    u16 p0C, p79;
    int outer;

    REG_W(0x2000, (ctrl_save & 0x7F) | 0x04);
    M(0x29) = 0x00;
    REG_W(0x2001, v24_save & 0xE7);

    p79 = (u16)(M(0x79) | (M(0x7A) << 8));


    {
        u8 sx = M(0x7C);
        u8 lo = (u8)((sx << 1) & 0x1C);
        u8 hi = (u8)((sx & 0x10) >> 2);
        u16 t = (u16)(0x00 + lo);
        M(0x16) = (u8)t;
        M(0x17) = (u8)(0x20 + hi + (t >> 8));
    }


    M(0x0A) = 0x12;
    p0C = (u16)(c0c_save | (c0d_save << 8));
    for (outer = 0; outer < 0x12; outer++) {
        u8 inner;

        M(0x0B) = 0x0C;
        REG_W(0x2006, M(0x17));
        REG_W(0x2006, M(0x16));
        M(0x08) = 0x00;
        do {
            u8 idx = M((u16)(p0C + M(0x08)));
            u8 y = (u8)(idx << 2);
            REG_W(0x2007, M((u16)(p79 + y)));
            REG_W(0x2007, M((u16)(p79 + (u8)(y + 1))));
            M(0x08)++;
            M(0x0B)--;
        } while (M(0x0B) != 0);


        M(0x0B) = 0x0C;
        REG_W(0x2006, M(0x17));
        inner = (u8)(M(0x16) + 1);
        REG_W(0x2006, inner);
        M(0x08) = 0x00;
        do {
            u8 idx = M((u16)(p0C + M(0x08)));
            u8 y = (u8)((idx << 2) + 2);
            REG_W(0x2007, M((u16)(p79 + y)));
            REG_W(0x2007, M((u16)(p79 + (u8)(y + 1))));
            M(0x08)++;
            M(0x0B)--;
        } while (M(0x0B) != 0);


        M(0x16) += 2;
        if (M(0x16) & 0x20) {
            M(0x16) = 0x00;
            M(0x17) ^= 0x04;
        }


        {
            u16 t = (u16)(0x0C + M(0x0C));
            M(0x0C) = (u8)t;
            M(0x0D) = (u8)(M(0x0D) + (t >> 8));
            p0C = (u16)(M(0x0C) | (M(0x0D) << 8));
        }
        M(0x0A)--;
    }


    M(0x0D) = c0d_save;
    M(0x0C) = c0c_save;
    p0C = (u16)(c0c_save | (c0d_save << 8));


    {
        u8 sx = M(0x7C);
        u8 lo = (u8)((sx >> 1) & 0x07);
        u8 hi = (u8)((sx & 0x10) >> 2);
        u16 t = (u16)(0xC0 + lo);
        M(0x16) = (u8)t;
        M(0x17) = (u8)(0x23 + hi + (t >> 8));
    }
    M(0x0A) = 0x09;

    for (;;) {
        int x;
        for (x = 6; x > 0; x--) {
            u8 a;


            a = M((u16)(p0C + 0x0D));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }

            a = M((u16)(p0C + 0x01));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }

            a = M((u16)(p0C + 0x0C));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }

            a = M((u16)(p0C + 0x00));
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }
            { u8 c1 = (a >> 7) & 1; a = (u8)(a << 1); M(0x08) = (u8)((M(0x08) << 1) | c1); }

            REG_W(0x2006, M(0x17));
            REG_W(0x2006, M(0x16));
            REG_W(0x2007, M(0x08));


            { u16 t = (u16)(0x02 + M(0x0C)); M(0x0C) = (u8)t; M(0x0D) = (u8)(M(0x0D) + (t >> 8)); }

            { u16 t = (u16)(0x08 + M(0x16)); M(0x16) = (u8)t; M(0x17) = (u8)(M(0x17) + (t >> 8)); }
            p0C = (u16)(M(0x0C) | (M(0x0D) << 8));
        }

        { u16 t = (u16)(0x0C + M(0x0C)); M(0x0C) = (u8)t; M(0x0D) = (u8)(M(0x0D) + (t >> 8)); }

        { u16 t = (u16)(0xD1 + M(0x16)); M(0x16) = (u8)t; M(0x17) = (u8)(M(0x17) + 0xFF + (t >> 8)); }
        p0C = (u16)(M(0x0C) | (M(0x0D) << 8));

        if (M(0x16) & 0x08) {
            M(0x16) = 0xC0;
            M(0x17) ^= 0x04;
        }
        M(0x0A)--;
        if (M(0x0A) == 0) break;
    }


    M(0x24) = v24_save;
    M(0x29) = v29_save;
    M(0x23) = ctrl_save;
    REG_W(0x2000, ctrl_save);

    r->value = ctrl_save;
    r->index = 0;
}
