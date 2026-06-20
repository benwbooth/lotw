














#include "game_memory.h"
#include "routine_context.h"


#define player_x_fine  0x43
#define player_x_tile  0x44
#define player_y       0x45
#define scroll_x_fine  0x7B
#define scroll_x_tile  0x7C
#define map_screen_x   0x47
#define map_screen_y   0x48



#define ROOM_CKPT_BYTES  7
#define ROOM_CKPT_DEPTH  4
u8  room_ckpt_stack[ROOM_CKPT_DEPTH][ROOM_CKPT_BYTES];
int room_ckpt_sp = 0;

void routine_0193(RoutineContext *r)
{
    (void)r;
    GAME_MEM8(0xFE) = GAME_MEM8(0x8E);

    if (room_ckpt_sp < ROOM_CKPT_DEPTH) {
        u8 *c = room_ckpt_stack[room_ckpt_sp++];
        c[0] = GAME_MEM8(player_x_fine);
        c[1] = GAME_MEM8(player_x_tile);
        c[2] = GAME_MEM8(player_y);
        c[3] = GAME_MEM8(scroll_x_fine);
        c[4] = GAME_MEM8(scroll_x_tile);
        c[5] = GAME_MEM8(map_screen_x);
        c[6] = GAME_MEM8(map_screen_y);
    }
}
