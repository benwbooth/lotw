














#include "game_memory.h"
#include "routine_context.h"

void routine_0267(RoutineContext *r);
void routine_0268(RoutineContext *r);

void routine_0266(RoutineContext *r)
{
    GAME_MEM8(0xE3) = 0x0B;
    GAME_MEM8(0xE5) = 0xB0;
    GAME_MEM8(0xE6) = 0x04;

    do {
        u16 ptr = (u16)(GAME_MEM8(0xE5) | (GAME_MEM8(0xE6) << 8));
        u8 v = GAME_MEM8((u16)(ptr + 1));

        if (v != 0) {
            r->value = v; r->offset = 0x01;
            routine_0268(r);
        } else {

            if (GAME_MEM8(0x20) & 0x40) {

                if (!(GAME_MEM8(0xFD) & 0x40)) {
                    r->value = 0x00; r->offset = 0x01;
                    routine_0267(r);
                }
            }
        }

        GAME_MEM8(0xE3)++;
        {
            u16 t = (u16)(0x10 + GAME_MEM8(0xE5));
            GAME_MEM8(0xE5) = (u8)t;
            GAME_MEM8(0xE6) = (u8)(GAME_MEM8(0xE6) + (t >> 8));
        }
    } while ((u8)(GAME_MEM8(0xE3) - 0x0B) < GAME_MEM8(0x5E));
}
