





#include "game_memory.h"
#include "routine_context.h"

void read_controllers(RoutineContext *r)
{
    u8 x, a, c;

#ifdef LOTW_SHIM


    {
        extern u8 (*lotw_next_input)(void);
        extern void ppu_set_buttons(u8 b);
        if (lotw_next_input) ppu_set_buttons(lotw_next_input());
    }


    REG_W(0x4016, 0x01);
    REG_W(0x4016, 0x00);
    for (x = 8; x != 0; x--) {
        a = (u8)(REG_R(0x4016) | REG_R(0x4017));
        c = a & 1; a >>= 1;
        GAME_MEM8(0x20) = (u8)((GAME_MEM8(0x20) << 1) | c);
        c = a & 1;
        GAME_MEM8(0x21) = (u8)((GAME_MEM8(0x21) << 1) | c);
    }
    GAME_MEM8(0x20) = GAME_MEM8(0x20) | GAME_MEM8(0x21);
    (void)r; return;
#endif
    GAME_MEM8(0x4016) = 0x01;
    GAME_MEM8(0x4016) = 0x00;
    for (x = 8; x != 0; x--) {
        a = GAME_MEM8(0x4016) | GAME_MEM8(0x4017);
        c = a & 1; a >>= 1;
        GAME_MEM8(0x20) = (u8)((GAME_MEM8(0x20) << 1) | c);
        c = a & 1;
        GAME_MEM8(0x21) = (u8)((GAME_MEM8(0x21) << 1) | c);
    }
    GAME_MEM8(0x20) = GAME_MEM8(0x20) | GAME_MEM8(0x21);
}
