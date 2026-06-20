

















#include "game_memory.h"
#include "routine_context.h"

void routine_0279(RoutineContext *r);
void routine_0278(RoutineContext *r);
void routine_0280(RoutineContext *r);
void routine_0281(RoutineContext *r);
void routine_0282(RoutineContext *r);
void inc16_95(RoutineContext *r);

static u8 deref_stream(RoutineContext *r)
{

    u8 x = r->index;
    u8 lo = GAME_MEM8((0x95 + x) & 0xFF);
    u8 hi = GAME_MEM8((0x96 + x) & 0xFF);
    return GAME_MEM8((u16)(lo | (hi << 8)));
}

void routine_0277(RoutineContext *r)
{
    r->index = GAME_MEM8(0x02);

    inc16_95(r);
    GAME_MEM8(0x04) = deref_stream(r);

    inc16_95(r);
    GAME_MEM8(0x05) = deref_stream(r);

    inc16_95(r);

    u8 idx = GAME_MEM8(0x04);
    if (idx >= 0x05)
        return;


    static const u16 tbl[5] = { 0xFBC5, 0xFBE2, 0xFBFF, 0xFC02, 0xFC05 };
    u16 p = tbl[idx];
    GAME_MEM8(0x06) = (u8)(p & 0xFF);
    GAME_MEM8(0x07) = (u8)(p >> 8);

    r->value = GAME_MEM8(0x05);
    r->index = GAME_MEM8(0x02);

    switch (idx) {
    case 0: routine_0278(r); break;
    case 1: routine_0279(r); break;
    case 2: routine_0280(r); break;
    case 3: routine_0281(r); break;
    case 4: routine_0282(r); break;
    }
}
