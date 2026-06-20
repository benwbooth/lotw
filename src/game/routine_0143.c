















#include "game_memory.h"
#include "routine_context.h"

void routine_0143(RoutineContext *r)
{
    u8 dx, sum, carry;

    GAME_MEM8(0x0E) = GAME_MEM8(0x43);
    GAME_MEM8(0x0F) = GAME_MEM8(0x44);
    GAME_MEM8(0x0A) = GAME_MEM8(0x45);

    if (GAME_MEM8(0x4B) != 0)
        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x4B) + GAME_MEM8(0x0A));

    dx = GAME_MEM8(0x49);
    if (dx != 0) {
        sum = (u8)(dx + GAME_MEM8(0x0E));
        GAME_MEM8(0x0E) = (u8)(sum & 0x0F);
        carry = (u8)((sum >> 4) & 1);
        GAME_MEM8(0x0F) = (u8)(GAME_MEM8(0x0F) + GAME_MEM8(0x4A) + carry);
    }
}
