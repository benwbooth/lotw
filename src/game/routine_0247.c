



















#include "game_memory.h"
#include "routine_context.h"

void routine_0241(RoutineContext *r);
void routine_0115(RoutineContext *r);
void routine_0111(RoutineContext *r);
void routine_0249(RoutineContext *r);
void routine_0253(RoutineContext *r);

void routine_0247(RoutineContext *r)
{
    u8 saved_f7 = GAME_MEM8(0xF7);
    u8 cflag;

    for (;;) {
        routine_0241(r);

        routine_0115(r);
        if (r->carry) {
            GAME_MEM8(0xEE) = 0x00;
            GAME_MEM8(0xF3) = 0xF0;
            cflag = 1;
            break;
        }

        if ((u8)(GAME_MEM8(0xEE) - 1) == 0) {
            routine_0111(r);
            if (r->carry)
                routine_0249(r);
        }


        routine_0253(r);
        if (r->carry == 0) {
            cflag = 0;
            break;
        }

        {
            u8 x = GAME_MEM8(0xF7);
            if (x == 0) {
                cflag = 1;
                break;
            }
            if (!(x & 0x80)) {
                x = (u8)(x - 2);
            }
            x = (u8)(x + 1);
            GAME_MEM8(0xF7) = x;
            if (x == 0) {
                cflag = 1;
                break;
            }

        }
    }


    GAME_MEM8(0xF7) = saved_f7;
    r->carry = cflag;
}
