





















#include "game_memory.h"
#include "routine_context.h"

void routine_0067(RoutineContext *r);
void routine_0128(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void routine_0077(RoutineContext *r);
void routine_0127(RoutineContext *r);
void routine_0060(RoutineContext *r);
void routine_0061(RoutineContext *r);
void routine_0070(RoutineContext *r);

void routine_0137(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0x77) | (GAME_MEM8(0x78) << 8));
    u8 a;

    r->offset = 0x0C;
    GAME_MEM8(0x47) = GAME_MEM8((u16)(ptr + r->offset));
    r->offset++;
    GAME_MEM8(0x48) = GAME_MEM8((u16)(ptr + r->offset));
    r->offset++;
    a = GAME_MEM8((u16)(ptr + r->offset));
    GAME_MEM8(0x44) = a;


    if (a >= 0x08)
        a = (u8)(a - 0x08);
    else
        a = 0x00;


    if (a >= 0x31)
        a = 0x30;
    GAME_MEM8(0x7C) = a;

    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x7B) = 0x00;

    r->offset++;
    r->value = GAME_MEM8((u16)(ptr + r->offset));
    GAME_MEM8(0x45) = r->value;


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
