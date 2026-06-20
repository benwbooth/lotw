







#include "game_memory.h"
#include "routine_context.h"

void routine_0278(RoutineContext *r)
{
    u8 a = r->value;
    u8 x = r->index;
    u8 hi = (u8)((u8)(a & 0xF0) << 2);
    GAME_MEM8(0x00) = hi;
    GAME_MEM8((0x99 + x) & 0xFF) = (u8)((GAME_MEM8((0x99 + x) & 0xFF) & 0x3F) | hi);
    a = (u8)(a << 4);
    GAME_MEM8((0xA2 + x) & 0xFF) = a;
    GAME_MEM8((0x9A + x) & 0xFF) = GAME_MEM8((u16)(0xFDD2 + a));
    r->value = GAME_MEM8((u16)(0xFDD2 + a));
    r->offset = a;
    r->index = x;
}
