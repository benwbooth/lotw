









#include "game_memory.h"
#include "routine_context.h"

void routine_0286(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x02);
    u8 y = (u8)(GAME_MEM8((0xA2 + x) & 0xFF) + 0x0C);
    GAME_MEM8((0x9B + x) & 0xFF) = y;
    GAME_MEM8((0x9C + x) & 0xFF) = GAME_MEM8((u16)(0xFDCB + y));
    GAME_MEM8((0x9D + x) & 0xFF) = GAME_MEM8((u16)(0xFDCC + y));
    GAME_MEM8((0x9E + x) & 0xFF) = GAME_MEM8((u16)(0xFDCD + y));
    r->index = x;
    r->offset = y;
    r->value = GAME_MEM8((u16)(0xFDCD + y));
}
