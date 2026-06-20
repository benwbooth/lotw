







#include "game_memory.h"
#include "routine_context.h"


#define player_x_fine  0x43
#define player_x_tile  0x44
#define player_y       0x45
#define scroll_x_fine  0x7B
#define scroll_x_tile  0x7C
#define map_screen_x   0x47
#define map_screen_y   0x48


#define ROOM_CKPT_BYTES 7
extern u8  room_ckpt_stack[][ROOM_CKPT_BYTES];
extern int room_ckpt_sp;

void routine_0194(RoutineContext *r)
{
    (void)r;
    if (room_ckpt_sp > 0) {
        u8 *c = room_ckpt_stack[--room_ckpt_sp];
        GAME_MEM8(player_x_fine) = c[0];
        GAME_MEM8(player_x_tile) = c[1];
        GAME_MEM8(player_y)      = c[2];
        GAME_MEM8(scroll_x_fine) = c[3];
        GAME_MEM8(scroll_x_tile) = c[4];
        GAME_MEM8(map_screen_x)  = c[5];
        GAME_MEM8(map_screen_y)  = c[6];
    }
}
