




#include "game_memory.h"
#include "routine_context.h"

void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0086(RoutineContext *r)
{
    u8 bank = (u8)(GAME_MEM8(0x48) >> 1);
    u8 t, lo;

    if (bank != GAME_MEM8(0x30)) {
        GAME_MEM8(0x30) = bank;
        r->value = 0xFF;
        queue_ppu_job_and_wait(r);
    }

    t = (u8)(((GAME_MEM8(0x48) & 0x01) << 2));
    t = (u8)((t | GAME_MEM8(0x47)) << 2);
    lo = (u8)(t + 0x80);
    GAME_MEM8(0x76) = lo;
    GAME_MEM8(0x78) = (u8)(lo + 0x03);
    GAME_MEM8(0x77) = 0x00;
    GAME_MEM8(0x75) = 0x00;




    r->carry = ((lo + 0x03) > 0xFF) ? 1 : 0;
}
