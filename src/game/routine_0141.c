












#include "game_memory.h"
#include "routine_context.h"

void routine_0065(RoutineContext *r);
void routine_0061(RoutineContext *r);
void routine_0062(RoutineContext *r);
void routine_0080(RoutineContext *r);
void routine_0074(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0141(RoutineContext *r)
{
    u8 outer;

    routine_0065(r);
    GAME_MEM8(0x85) = 0x00;
    routine_0061(r);
    routine_0062(r);

    if (GAME_MEM8(0x7C) >= 0x21)
        GAME_MEM8(0x7C) = 0x20;
    routine_0080(r);

    GAME_MEM8(0x7C) = (u8)(GAME_MEM8(0x7C) + 0x10);
    routine_0080(r);

    GAME_MEM8(0x08) = 0x01;
    do {
        u8 x = 0x0C;
        do {
            u16 sum = (u16)(GAME_MEM8(0x1C) + GAME_MEM8(0x08));
            GAME_MEM8(0x1C) = (u8)sum;
            if (sum & 0x100)
                GAME_MEM8(0x1D) = (u8)(GAME_MEM8(0x1D) ^ 0x01);
            r->value = 0xFF;
            queue_ppu_job_and_wait(r);
        } while (--x != 0);
        GAME_MEM8(0x08) = (u8)(GAME_MEM8(0x08) + 1);
        outer = GAME_MEM8(0x08);
    } while (outer < 0x20);

    GAME_MEM8(0x8F) = 0x18;
    GAME_MEM8(0x90) = 0xFF;
    r->index = 0x08;
    routine_0074(r);
}
