





#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r);
void routine_0015(RoutineContext *r);
void routine_0008(RoutineContext *r);
void routine_0010(RoutineContext *r);
void routine_0009(RoutineContext *r);
void routine_0214(RoutineContext *r);

void routine_0006(RoutineContext *r)
{
    routine_0213(r);

    GAME_MEM8(0xFD) = (u8)((GAME_MEM8(0x20) & 0x40) | GAME_MEM8(0xFD));

    r->value = GAME_MEM8(0xFD);
    r->offset = 0x02;
    routine_0015(r);
    routine_0008(r);
    routine_0010(r);
    if (!r->carry) {
        GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
        GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
        GAME_MEM8(0xEE) = 0x18;
        GAME_MEM8(0xEF) = 0x00;
        GAME_MEM8(0xED) = 0x21;
        GAME_MEM8(0x8F) = 0x19;

    }


    if (GAME_MEM8(0xEE) != 0)
        routine_0009(r);
    routine_0214(r);
}
