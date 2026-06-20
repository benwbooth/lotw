









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
#define vram_dst_lo   GAME_MEM8(0x0016)
#define vram_dst_hi   GAME_MEM8(0x0017)

void routine_0163(RoutineContext *r);
void routine_0164(RoutineContext *r);
void routine_0061(RoutineContext *r);
void routine_0128(RoutineContext *r);
void routine_0127(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void routine_0077(RoutineContext *r);
void routine_0075(RoutineContext *r);
void routine_0080(RoutineContext *r);
void queue_ppu_job_and_wait(RoutineContext *r);
void farcall_bank_09_r7(RoutineContext *r);
void routine_0067(RoutineContext *r);
void routine_0060(RoutineContext *r);
void routine_0070(RoutineContext *r);















static void scene_rebuild_full(RoutineContext *r)
{
    routine_0067(r);
    routine_0128(r);
    scene_assemble(r);
    routine_0077(r);
    routine_0127(r);
    routine_0060(r);
    routine_0061(r);
    routine_0070(r);
    GAME_MEM8(0x36) = 0;
    r->carry = 1;
}


static void scene_rebuild_vert(RoutineContext *r)
{
    routine_0128(r);
    routine_0127(r);
    scene_assemble(r);
    routine_0077(r);
    routine_0075(r);
    GAME_MEM8(0x36) = 0;
    r->carry = 1;
}

void routine_0142(RoutineContext *r)
{
    u8 a = player_y;

    if (a < 0x10) {

        routine_0164(r);
        if (r->carry == 0) { r->carry = 0; return; }
        if (map_screen_y == 0x00) {

            map_screen_y = 0x10; map_screen_x = 0x03; scroll_x_tile = 0x12;
            player_y = 0xB0; player_x_tile = 0x1A; player_x_fine = 0x00;
            scroll_x_fine = 0x00;
            scene_rebuild_full(r);
            return;
        }
        if (map_screen_y == 0x10) { r->carry = 0; return; }
        map_screen_y = (u8)(map_screen_y - 1);
        player_y = 0xB0;
        scene_rebuild_vert(r);
        return;
    }

    if (a >= 0xA1) {

        if (map_screen_y == 0x10) {

            map_screen_y = 0x00; map_screen_x = 0x00; scroll_x_tile = 0x00;
            player_y = 0x00; player_x_fine = 0x00; scroll_x_fine = 0x00;
            player_x_tile = 0x01;
            scene_rebuild_full(r);
            return;
        }
        if ((u8)(map_screen_y + 1) >= 0x10) { r->carry = 0; return; }
        map_screen_y = (u8)(map_screen_y + 1);
        player_y = 0x00;
        scene_rebuild_vert(r);
        return;
    }


    if (map_screen_y == 0x10) { r->carry = 0; return; }
    routine_0163(r);
    GAME_MEM8(0x85) = 0x00;
    GAME_MEM8(0x56) = (u8)(GAME_MEM8(0x56) & 0x07);

    if (player_x_tile == 0x00) {

        if ((u8)((map_screen_x - 1)) & 0x80) { r->carry = 0; return; }
        map_screen_x = (u8)(map_screen_x - 1);
        GAME_MEM8(0x57) = 0x00;
        routine_0061(r);
        scroll_x_tile = 0x30;
        player_x_tile = 0x3F;
        player_x_fine = 0x00;

    } else {
        if (player_x_tile < 0x3E) { r->carry = 0; return; }

        if ((u8)(map_screen_x + 1) >= 0x04) { r->carry = 0; return; }
        map_screen_x = (u8)(map_screen_x + 1);
        GAME_MEM8(0x57) = 0x40;
        routine_0061(r);
        scroll_x_tile = 0x00;
        player_x_fine = 0x00;
        player_x_tile = 0x00;

    }


    routine_0128(r);
    routine_0127(r);
    scroll_x_fine = 0x00;
    scene_assemble(r);
    routine_0080(r);
    routine_0075(r);

    if (player_x_tile != 0x00) {

        GAME_MEM8(0x1D) = 0x01;
        GAME_MEM8(0x1C) = 0x00;
        GAME_MEM8(0x0213) = 0x00;
        GAME_MEM8(0x0217) = 0x08;
        GAME_MEM8(0x0A) = 0x0F;
        do {
            GAME_MEM8(0x0B) = 0x03;
            do {
                if (GAME_MEM8(0x0B) == 0) {
                    GAME_MEM8(0x0213) = (u8)(GAME_MEM8(0x0213) - 1);
                    GAME_MEM8(0x0217) = (u8)(GAME_MEM8(0x0217) - 1);
                    if ((GAME_MEM8(0x4E) | GAME_MEM8(0x4F)) == 0) {
                        GAME_MEM8(0x0211) ^= 0x04;
                        GAME_MEM8(0x0215) ^= 0x04;
                    }
                }

                GAME_MEM8(0x0213) = (u8)(GAME_MEM8(0x0213) + 0x04);
                GAME_MEM8(0x0217) = (u8)(GAME_MEM8(0x0213) + 0x08);
                GAME_MEM8(0x1C) = (u8)(GAME_MEM8(0x1C) - 0x04);
                r->value = 0xFF;
                queue_ppu_job_and_wait(r);
                GAME_MEM8(0x0B) = (u8)(GAME_MEM8(0x0B) - 1);
            } while ((GAME_MEM8(0x0B) & 0x80) == 0);
            GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x0A) - 1);
        } while ((GAME_MEM8(0x0A) & 0x80) == 0);
        vram_dst_lo = 0x1E;
        vram_dst_hi = 0x20;
        GAME_MEM8(0x0C) = 0x2F;
        farcall_bank_09_r7(r);
        GAME_MEM8(0x36) = 0;
        r->carry = 1;
        return;
    }


    GAME_MEM8(0x1C) = 0xFC;
    GAME_MEM8(0x1D) = 0x01;
    GAME_MEM8(0x0213) = 0xF0;
    GAME_MEM8(0x0217) = 0xF8;
    GAME_MEM8(0x0A) = 0x0F;
    do {
        GAME_MEM8(0x0B) = 0x03;
        do {
            if (GAME_MEM8(0x0B) == 0) {
                GAME_MEM8(0x0213) = (u8)(GAME_MEM8(0x0213) + 1);
                GAME_MEM8(0x0217) = (u8)(GAME_MEM8(0x0217) + 1);
                if ((GAME_MEM8(0x4E) | GAME_MEM8(0x4F)) == 0) {
                    GAME_MEM8(0x0211) ^= 0x04;
                    GAME_MEM8(0x0215) ^= 0x04;
                }
            }

            GAME_MEM8(0x0213) = (u8)(GAME_MEM8(0x0213) - 0x04);
            GAME_MEM8(0x0217) = (u8)(GAME_MEM8(0x0213) + 0x08);
            GAME_MEM8(0x1C) = (u8)(GAME_MEM8(0x1C) + 0x04);
            r->value = 0xFF;
            queue_ppu_job_and_wait(r);
            GAME_MEM8(0x0B) = (u8)(GAME_MEM8(0x0B) - 1);
        } while ((GAME_MEM8(0x0B) & 0x80) == 0);
        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x0A) - 1);
    } while ((GAME_MEM8(0x0A) & 0x80) == 0);
    vram_dst_lo = 0x00;
    vram_dst_hi = 0x24;
    GAME_MEM8(0x0C) = 0x10;
    farcall_bank_09_r7(r);
    GAME_MEM8(0x36) = 0;
    r->carry = 1;
}
