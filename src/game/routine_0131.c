





#include "game_memory.h"
#include "routine_context.h"

void queue_ppu_job_and_wait(RoutineContext *r);

void routine_0131(RoutineContext *r)
{
    int y = 0x1F;
    int x = 0x26;
    int i;

    do {
        for (i = 0; i < 4; ++i) {
            u8 out = (u8)(GAME_MEM8((u16)(0x0322 + y)) | 0x80);
            if (out >= 0xA0)
                out = 0x7F;
            GAME_MEM8((u16)(0x0362 + (x & 0xFF))) = out;
            x = (x - 1) & 0xFF;
            y = (y - 1) & 0xFF;
        }
        x = (x - 1) & 0xFF;
    } while ((x & 0x80) == 0);

    GAME_MEM8(0x1A) = 0x13;
    GAME_MEM8(0x1B) = 0x00;
    GAME_MEM8(0x16) = 0xE6;
    GAME_MEM8(0x17) = 0x24;
    GAME_MEM8(0x18) = 0x62;
    GAME_MEM8(0x19) = 0x03;
    r->value = 0x05;
    queue_ppu_job_and_wait(r);

    GAME_MEM8(0x16) = 0x06;
    GAME_MEM8(0x17) = 0x25;
    GAME_MEM8(0x18) = 0x76;
    GAME_MEM8(0x19) = 0x03;
    r->value = 0x05;
    queue_ppu_job_and_wait(r);
}
