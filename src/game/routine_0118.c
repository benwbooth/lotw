


















#include "game_memory.h"
#include "routine_context.h"

void routine_0121(RoutineContext *r);
void routine_0122(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0118(RoutineContext *r)
{
    u8 x = r->index;
    u8 lo, hi;
    u16 s;

    lo = (u8)((x & 0x07) << 2);
    lo = (u8)(((x & 0x08) << 4) | lo);
    hi = 0x00;

    s = (u16)(0xC2 + lo);
    GAME_MEM8(0x16) = (u8)s;
    GAME_MEM8(0x17) = (u8)(0x20 + hi + (s >> 8));


    r->value = r->offset;
    routine_0121(r);




    {
        u8 in = x;
        u8 dx = (u8)(GAME_MEM8(0x40) << 1);
        u8 yy, carry, v;
        if (in >= 0x08) dx++;
        yy = (u8)((in & 0x07) + 1);
        v = GAME_MEM8((u16)(0xFFBB + dx));
        carry = 0;
        do {
            carry = (u8)(v >> 7);
            v = (u8)(v << 1);
        } while (--yy != 0);
        r->carry = carry;
    }
    r->value = x;
    routine_0122(r);

    if (!r->carry) {
        GAME_MEM8(0x18) = (u8)(GAME_MEM8(0x18) - 0x40);
        GAME_MEM8(0x19) = (u8)(GAME_MEM8(0x19) - 0x40);
    }

    r->value = 0x06;
    queue_ppu_job_and_wait(r);
}
