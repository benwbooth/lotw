





#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0169(RoutineContext *r);

void routine_0168(RoutineContext *r)
{
    u8 s_0E, s_0F, s_0A;

    GAME_MEM8(0xE5) = 0x90;
    GAME_MEM8(0xE6) = 0x04;

    s_0E = GAME_MEM8(0x0E);
    s_0F = GAME_MEM8(0x0F);
    s_0A = GAME_MEM8(0x0A);

    GAME_MEM8(0x0C) = GAME_MEM8(0x0F);
    GAME_MEM8(0x0D) = GAME_MEM8(0x0A);

    routine_0090(r);

    r->offset = 0x00;
    routine_0169(r);
    if (r->carry) goto restore;

    if (GAME_MEM8(0x0E) != 0) {
        r->offset = 0x0C;
        routine_0169(r);
        if (r->carry) goto restore;
    }


    {
        u8 a = GAME_MEM8(0x0A);
        if (a >= 0xB0) goto done_clc;
        if ((a & 0x0F) == 0) goto done_clc;

        r->offset = 0x01;
        routine_0169(r);
        if (r->carry) goto restore;

        if (GAME_MEM8(0x0E) == 0) goto done_clc;

        r->offset = 0x0D;
        routine_0169(r);
        if (r->carry) goto restore;
    }

done_clc:
    r->carry = 0;

restore:
    GAME_MEM8(0x0A) = s_0A;
    GAME_MEM8(0x0F) = s_0F;
    GAME_MEM8(0x0E) = s_0E;
}
