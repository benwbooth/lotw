























#include "game_memory.h"
#include "routine_context.h"

void farcall_bank_0C0D_seed(RoutineContext *r);
void ram_state_init(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void game_update(RoutineContext *r);
void routine_0033(RoutineContext *r);
void main_loop_dispatch(RoutineContext *r);


static void farcall_0C0D(RoutineContext *r, u8 lo, u8 hi, void (*target)(RoutineContext *))
{
    u8 old6 = GAME_MEM8(0x30), old7 = GAME_MEM8(0x31);
    GAME_MEM8(0x32) = old6; GAME_MEM8(0x33) = old7;
    GAME_MEM8(0x0E) = lo; GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
    target(r);
    GAME_MEM8(0x31) = old7; GAME_MEM8(0x30) = old6; GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
}

void main_init(RoutineContext *r)
{

    REG_W(0x2000, 0x00);
    REG_W(0x2001, 0x00);
    REG_W(0x4010, 0x00);
    GAME_MEM8(0x0027) = 0x1F;
    REG_W(0x4015, 0x1F);
    REG_W(0x4017, 0xC0);








    REG_W(0xA000, 0x00);
    farcall_bank_0C0D_seed(r);


    ram_state_init(r);
    farcall_0C0D(r, 0x64, 0xAE, routine_0033);


    GAME_MEM8(0x46) = 0x00;
    GAME_MEM8(0x7B) = 0x00;
    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x7C) = 0x30;
    GAME_MEM8(0x44) = 0x3C;
    GAME_MEM8(0x45) = 0xA0;
    scene_assemble(r);
    GAME_MEM8(0x20) = 0x08;
    game_update(r);

    main_loop_dispatch(r);
}
