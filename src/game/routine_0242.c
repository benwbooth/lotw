





#include "game_memory.h"
#include "routine_context.h"

void routine_0243(RoutineContext *r);
void routine_0244(RoutineContext *r);
void routine_0245(RoutineContext *r);
void routine_0246(RoutineContext *r);

void routine_0242(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
    static const u16 table[4] = { 0xF03B, 0xF04B, 0xF071, 0xF0B9 };
    u8 idx = (u8)(GAME_MEM8((u16)(ptr + 7)) & 0x03);
    u16 handler = table[idx];


    GAME_MEM8(0x0E) = (u8)(handler & 0xFF);
    GAME_MEM8(0x0F) = (u8)(handler >> 8);


    r->offset = 0x07;
    r->index = (u8)(idx << 1);
    r->value = (u8)(idx << 1);

    switch (idx) {
    case 0: routine_0243(r); break;
    case 1: routine_0244(r); break;
    case 2: routine_0245(r); break;
    case 3: routine_0246(r); break;
    }
}
