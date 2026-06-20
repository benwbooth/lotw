



#include "game_memory.h"
#include "routine_context.h"

void routine_0067(RoutineContext *r);
void routine_0127(RoutineContext *r);
void routine_0084(RoutineContext *r);
void routine_0078(RoutineContext *r);
void routine_0144(RoutineContext *r);
void routine_0061(RoutineContext *r);
void routine_0060(RoutineContext *r);

void routine_0195(RoutineContext *r)
{
    u8 a = r->value;

    routine_0067(r);


    GAME_MEM8(0x08) = a;
    GAME_MEM8(0x47) = (u8)((a & 0x0C) >> 2);
    GAME_MEM8(0x7C) = (u8)((a & 0x03) << 4);
    GAME_MEM8(0x44) = (u8)(GAME_MEM8(0x7C) + 0x07);
    GAME_MEM8(0x48) = 0x10;
    GAME_MEM8(0x43) = 0x08;
    GAME_MEM8(0x45) = 0xA0;
    GAME_MEM8(0x4F) = 0x00;
    GAME_MEM8(0x4E) = 0x00;
    GAME_MEM8(0x7B) = 0x00;

    routine_0127(r);
    routine_0084(r);


    if (a == 0x04)
        GAME_MEM8(0x7A) = (u8)(0x1F + 0xA0);

    routine_0078(r);
    routine_0144(r);
    routine_0061(r);
    routine_0060(r);
}
