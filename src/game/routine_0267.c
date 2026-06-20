

















#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r);
void routine_0108(RoutineContext *r);
void routine_0269(RoutineContext *r);
void routine_0115(RoutineContext *r);
void routine_0204(RoutineContext *r);
void routine_0126(RoutineContext *r);
void routine_0125(RoutineContext *r);
void routine_0270(RoutineContext *r);
void routine_0214(RoutineContext *r);

void routine_0267(RoutineContext *r)
{
    routine_0213(r);

    GAME_MEM8(0xFD) = (u8)((GAME_MEM8(0x20) & 0x40) | GAME_MEM8(0xFD));

    r->offset = (GAME_MEM8(0x88) != 0) ? 0x04 : 0x02;
    r->value = GAME_MEM8(0xFD);
    routine_0108(r);

    routine_0269(r);

    routine_0115(r);
    if (r->carry) goto done;

    routine_0204(r);
    if (r->carry) goto done;

    GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);

    routine_0126(r);
    GAME_MEM8(0xEE) = r->value;
    if (r->carry == 0)
        routine_0204(r);

    routine_0125(r);
    GAME_MEM8(0xF8) = r->value;
    if (r->carry == 0)
        routine_0204(r);

    GAME_MEM8(0xEF) = 0x00;
    GAME_MEM8(0xED) = 0x21;
    GAME_MEM8(0x8F) = (u8)(0x22 + GAME_MEM8(0x40));

done:

    if (GAME_MEM8(0xEE) != 0)
        routine_0270(r);
    routine_0214(r);
}
