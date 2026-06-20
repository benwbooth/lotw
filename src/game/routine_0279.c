







#include "game_memory.h"
#include "routine_context.h"

void routine_0279(RoutineContext *r)
{
    u8 x = r->index;
    u8 a;
    u8 take_fbec;

    a = GAME_MEM8(0x02);
    if (a == 0x40) {
        take_fbec = 1;
    } else {
        a = GAME_MEM8(0x92);
        if (a != 0) {
            r->value = a;
            r->index = x;
            return;
        }
        take_fbec = 1;
    }
    (void)take_fbec;


    {
        u16 sum = (u16)(0x0F + GAME_MEM8(0x05));
        int carry_in = 1;
        u16 diff = (u16)((sum & 0xFF) - 0x08 + (carry_in - 1));
        u8 bcs = ((sum & 0xFF) >= 0x08);
        a = (u8)diff;
        if (!bcs)
            a = 0x00;
        a = (u8)(a << 1);
        a = (u8)(a + 1);
        GAME_MEM8((0xA0 + x) & 0xFF) = a;
    }
    r->value = a;
    r->index = x;
}
