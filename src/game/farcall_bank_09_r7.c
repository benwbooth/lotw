














#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void metasprite_build(RoutineContext *r);

void farcall_bank_09_r7(RoutineContext *r)
{
    u8 saved_r7 = GAME_MEM8(0x31);

    GAME_MEM8(0x25) = 0x07;
    REG_W(0x8000, 0x07);
    GAME_MEM8(0x31) = 0x09;
    REG_W(0x8001, 0x09);

    GAME_MEM8(0x0D) = 0x00;

    r->value = 0x00;
    routine_0090(r);
    metasprite_build(r);

    GAME_MEM8(0x25) = 0x07;
    REG_W(0x8000, 0x07);
    GAME_MEM8(0x31) = saved_r7;
    REG_W(0x8001, saved_r7);

    r->value = saved_r7;
}
