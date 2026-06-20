







#include "game_memory.h"
#include "routine_context.h"

void routine_0254(RoutineContext *r); void routine_0017(RoutineContext *r); void routine_0098(RoutineContext *r);

static void farcall_0C0D(RoutineContext *r, void (*target)(RoutineContext *))
{
    u8 old6 = GAME_MEM8(0x30), old7 = GAME_MEM8(0x31);
    GAME_MEM8(0x32) = old6; GAME_MEM8(0x33) = old7;
    GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
    target(r);
    GAME_MEM8(0x31) = old7; GAME_MEM8(0x30) = old6; GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
}

void routine_0257(RoutineContext *r)
{
    u16 e7 = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));

    GAME_MEM8(0x2E) = 0x3D;
    GAME_MEM8(0x0A) = GAME_MEM8((u16)(e7 + 3));
    GAME_MEM8(0x0F) = GAME_MEM8((u16)(e7 + 2));
    GAME_MEM8(0x0E) = 0x00;
    GAME_MEM8(0x0B) = 0x00;
    routine_0254(r);
    if (r->carry)
        return;


    GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
    GAME_MEM8(0xF1) = 0x00; GAME_MEM8(0xF0) = 0x00; GAME_MEM8(0xF4) = 0x00;
    GAME_MEM8(0xEE) = 0x01;
    GAME_MEM8(0xED) = 0x81;
    GAME_MEM8(0xEF) = 0x02;
    GAME_MEM8(0xF8) = GAME_MEM8((u16)(e7 + 5));
    {
        u8 bl = GAME_MEM8((u16)(e7 + 4));
        GAME_MEM8(0xF2) = bl;
        GAME_MEM8(0x0415) = bl; GAME_MEM8(0x0425) = bl; GAME_MEM8(0x0435) = bl;
    }

    GAME_MEM8(0x0E) = 0xE1; GAME_MEM8(0x0F) = 0xA7;
    farcall_0C0D(r, routine_0017);
    GAME_MEM8(0x0E) = 0x53; GAME_MEM8(0x0F) = 0xCB;
    farcall_0C0D(r, routine_0098);
}
