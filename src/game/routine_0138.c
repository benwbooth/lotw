













#include "game_memory.h"
#include "routine_context.h"

void routine_0141(RoutineContext *r);
void routine_0067(RoutineContext *r);
void routine_0128(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void routine_0077(RoutineContext *r);
void routine_0127(RoutineContext *r);
void routine_0060(RoutineContext *r);
void routine_0061(RoutineContext *r);
void routine_0070(RoutineContext *r);

void routine_0138(RoutineContext *r)
{
    routine_0141(r);

    GAME_MEM8(0x48) = 0x11;
    r->index = (u8)(GAME_MEM8(0x6E) - 1);
    GAME_MEM8(0x47) = r->index;
    GAME_MEM8(0x7C) = 0x12;
    GAME_MEM8(0x45) = 0x10;
    GAME_MEM8(0x44) = 0x1A;
    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x7B) = 0x00;
    r->value = 0x00;


    routine_0067(r);
    routine_0128(r);
    scene_assemble(r);
    routine_0077(r);
    routine_0127(r);
    routine_0060(r);
    routine_0061(r);
    routine_0070(r);
    r->carry = 1;
}
