











#include "game_memory.h"
#include "routine_context.h"

#define equipped_item GAME_MEM8(0x0055)
#define carried_item0 0x0051
#define map_screen_x  GAME_MEM8(0x0047)
#define map_screen_y  GAME_MEM8(0x0048)
#define scroll_x_tile GAME_MEM8(0x007C)
#define scroll_x_fine GAME_MEM8(0x007B)
#define player_x_tile GAME_MEM8(0x0044)
#define player_x_fine GAME_MEM8(0x0043)
#define player_y      GAME_MEM8(0x0045)

void routine_0204(RoutineContext *r);
void routine_0062(RoutineContext *r);
void routine_0134(RoutineContext *r);
void routine_0067(RoutineContext *r);
void routine_0128(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void routine_0077(RoutineContext *r);
void routine_0127(RoutineContext *r);
void routine_0060(RoutineContext *r);
void routine_0061(RoutineContext *r);
void routine_0070(RoutineContext *r);

void routine_0136(RoutineContext *r)
{
    u8 y = equipped_item;
    u8 x = GAME_MEM8((u16)(carried_item0 + y));

    if (x >= 0x02) {
        if (x == 0x0B) {
            if (magic != 0)
                return;

            x = equipped_item;
            GAME_MEM8((u16)(carried_item0 + x)) = 0xFF;
            routine_0062(r);
            routine_0134(r);
            return;
        }

        if (x != 0x0D)
            return;

        if (map_screen_y >= 0x11) {
            equipped_item = 0x03;
            return;
        }

        x = equipped_item;
        GAME_MEM8((u16)(carried_item0 + x)) = 0xFF;
        routine_0062(r);
        GAME_MEM8(0x8F) = 0x12;

        map_screen_y = 0x10;
        map_screen_x = 0x03;
        scroll_x_tile = 0x12;
        player_y = 0xB0;
        player_x_tile = 0x1A;
        player_x_fine = 0x00;
        scroll_x_fine = 0x00;

        routine_0067(r);
        routine_0128(r);
        scene_assemble(r);
        routine_0077(r);
        routine_0127(r);
        routine_0060(r);
        routine_0061(r);
        routine_0070(r);
        r->carry = 1;
        return;
    }


    if (GAME_MEM8((u16)(0x86 + x)) != 0)
        return;

    r->index = x;
    routine_0204(r);
    if (r->carry == 0) {
        GAME_MEM8((u16)(0x86 + x)) = 0x02;
        return;
    }

    {
        u8 t = GAME_MEM8(0x37);
        if (t == 0 || (t & 0x80))
            return;
        GAME_MEM8(0x37) = 0xFD;
        GAME_MEM8(0x8F) = 0x1A;
    }
}
