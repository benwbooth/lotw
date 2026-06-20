




#include "game_memory.h"
#include "routine_context.h"
void routine_0140(RoutineContext *r)
{
    u8 x = GAME_MEM8(0x55);
    if (GAME_MEM8((u16)(0x51 + x)) == 0x0F &&
        GAME_MEM8(0x47) == 0x01 && GAME_MEM8(0x48) == 0x05 &&
        GAME_MEM8(0x7C) == 0x10 && GAME_MEM8(0x7B) == 0x00 &&
        GAME_MEM8(0x45) == 0xA0) {
        GAME_MEM8(0xEC) = 0x01;

    }
}
