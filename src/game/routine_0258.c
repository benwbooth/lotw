
































#include "game_memory.h"
#include "routine_context.h"

void routine_0231(RoutineContext *r);
void routine_0108(RoutineContext *r);
void routine_0261(RoutineContext *r);
void routine_0262(RoutineContext *r);
void routine_0239(RoutineContext *r);
void routine_0260(RoutineContext *r);
void routine_0238(RoutineContext *r);
void routine_0251(RoutineContext *r);
void routine_0263(RoutineContext *r);
void routine_0264(RoutineContext *r);

void routine_0258(RoutineContext *r)
{
    u8 a;

    GAME_MEM8(0xF4) = GAME_MEM8(0xF4) & 0x0F;

    if ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) == 0) {
        if ((GAME_MEM8(0xF4) & 0x03) == 0)
            GAME_MEM8(0xF4) = 0x01;


        {
            u8 x = GAME_MEM8(0xF3);
            GAME_MEM8(0xF3) = 0x00;
            x = (u8)(x - 1);
            if (x == 0) {
                a = GAME_MEM8(0xF4) & 0x03;
                if (a != 0) {
                    GAME_MEM8(0xF4) = (u8)(a ^ 0x03);
                    goto F3F5;
                }
                goto F3EE;
            }

            routine_0231(r);
            GAME_MEM8(0xF4) = (u8)(0x80 | GAME_MEM8(0xF4));
            goto F3F5;
        }
    } else {

        if (GAME_MEM8(0xF3) < 0x32)
            goto F3F5;

    }

F3EE:
    GAME_MEM8(0xF3) = 0x00;
    routine_0231(r);

F3F5:
    r->value = GAME_MEM8(0xF4);
    r->offset = 0x02;
    routine_0108(r);

    if (GAME_MEM8(0xF0) != 0) {
        routine_0260(r);
        if (r->carry)
            goto F424;
        goto F421;
    }


    if (GAME_MEM8(0xF1) != 0)
        goto F408;
    if (!(GAME_MEM8(0xF4) & 0x80))
        goto F40D;

F408:
    routine_0261(r);
    if (!r->carry)
        goto F421;

F40D:
    GAME_MEM8(0xF1) = 0x00;
    routine_0262(r);
    if (!r->carry)
        goto F421;
    routine_0239(r);
    goto F424;

F421:
    routine_0238(r);

F424:
    routine_0251(r);
    routine_0263(r);
    routine_0264(r);

}
