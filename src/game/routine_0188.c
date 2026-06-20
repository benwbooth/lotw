












#include "game_memory.h"
#include "routine_context.h"

void routine_0190(RoutineContext *r);
void routine_0122(RoutineContext *r);
void routine_0197(RoutineContext *r);
void routine_0062(RoutineContext *r);
void routine_0117(RoutineContext *r);
void routine_0119(RoutineContext *r);

void routine_0188(RoutineContext *r)
{
    for (;;) {
        u8 x, py;

        routine_0190(r);
        if (r->carry) {
            u8 e = GAME_MEM8(0x55);
            if (GAME_MEM8((u16)(0x51 + e)) == 0x0D) {
                GAME_MEM8(0x55) = 0x03;
                routine_0062(r);
            }
            return;
        }

        x = 0xFF;
        py = GAME_MEM8(0x45);
        if (py >= 0x58)
            goto flow_0441;
        x = (py < 0x38) ? 0x00 : 0x08;


        GAME_MEM8(0x08) = x;
        x = (u8)((GAME_MEM8(0x44) >> 1) | GAME_MEM8(0x08));
        if (GAME_MEM8((u16)(0x60 + x)) != 0) {
            r->value = x;
            routine_0122(r);
            if (r->carry) {
                GAME_MEM8((u16)(0x60 + x))--;
                goto flow_0441;
            }
        }

        GAME_MEM8(0x8F) = 0x06;
        continue;

    flow_0441:
        GAME_MEM8(0x08) = x;
        {
            u8 ci0 = GAME_MEM8(0x51);
            if (!(ci0 & 0x80))
                GAME_MEM8((u16)(0x60 + ci0))++;
        }

        GAME_MEM8(0x51) = GAME_MEM8(0x52);
        GAME_MEM8(0x52) = GAME_MEM8(0x53);
        GAME_MEM8(0x53) = GAME_MEM8(0x08);
        GAME_MEM8(0x8F) = 0x12;
        routine_0197(r);
        routine_0062(r);
        routine_0117(r);
        routine_0119(r);

    }
}
