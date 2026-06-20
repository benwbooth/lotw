












#include "game_memory.h"
#include "routine_context.h"

void farcall_bank_0C0D_seed(RoutineContext *r)
{
    GAME_MEM8(0x32) = GAME_MEM8(0x30);
    GAME_MEM8(0x33) = GAME_MEM8(0x31);

    GAME_MEM8(0x25) = 0x06;
    REG_W(0x8000, 0x06);
    GAME_MEM8(0x30) = 0x0C;
    REG_W(0x8001, 0x0C);

    GAME_MEM8(0x25) = 0x07;
    REG_W(0x8000, 0x07);
    GAME_MEM8(0x31) = 0x0D;
    REG_W(0x8001, 0x0D);

    r->value = 0x0D;
    r->offset = 0x07;
}
