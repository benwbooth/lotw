

























#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0109(RoutineContext *r);
void routine_0166(RoutineContext *r);
void routine_0165(RoutineContext *r);
void routine_0202(RoutineContext *r);

#define player_x_fine GAME_MEM8(0x43)
#define player_x_tile GAME_MEM8(0x44)
#define player_y      GAME_MEM8(0x45)
#define equipped_item GAME_MEM8(0x55)
#define stat_jump     GAME_MEM8(0x5C)

void routine_0163(RoutineContext *r)
{
    u8 x;


    if (GAME_MEM8(0x86) == 0 && GAME_MEM8(0x4F) == 0) {

        GAME_MEM8(0x0C) = player_x_tile;
        GAME_MEM8(0x0F) = player_x_tile;
        GAME_MEM8(0x0E) = player_x_fine;
        GAME_MEM8(0x0D) = player_y;
        GAME_MEM8(0x0A) = (u8)(player_y + 1);
        routine_0090(r);

        if (player_x_fine == 0) {
            GAME_MEM8(0x50) = 0x01;
            r->offset = 0x00;
            {
                u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
                if ((GAME_MEM8((u16)(ptr + r->offset)) & 0x3F) == 0)
                    goto dc4d;
            }

        }


        GAME_MEM8(0x50) = 0x00;
        if (player_y >= 0xB0)
            goto dc4a;

        routine_0109(r);
        if (r->carry) {
            if (GAME_MEM8(0x2D) >= 0x30) goto dc4d;
            {
                u8 y = equipped_item;
                x = GAME_MEM8((u16)(0x0051 + y));
            }
            if (x != 0x05) goto dc4d;
            if (GAME_MEM8(0x4E) == 0) goto dc4d;
            x = GAME_MEM8(0x09);
            GAME_MEM8((u16)(0x0401 + x)) = 0x80;
        }


        r->offset = 0x01;
        routine_0166(r);
        if (r->carry) goto dc4d;
        if (player_x_fine == 0) goto dc4a;
        r->offset = 0x0D;
        routine_0166(r);
        if (r->carry) goto dc4d;

    dc4a:
        GAME_MEM8(0x4E) = (u8)(GAME_MEM8(0x4E) + 1);
        return;

    dc4d:
        {
            u8 v = GAME_MEM8(0x4E);
            if (v >= stat_jump) {
                v = (u8)(v - 0x07);
                if (v >= stat_jump)
                    v = stat_jump;

                v = (u8)(v - 0x01);
                GAME_MEM8(0x4F) = v;
                GAME_MEM8(0x46) = (u8)(v + 0x0A);
                GAME_MEM8(0x8F) = 0x0A;
                routine_0202(r);
            }
        }

        if (GAME_MEM8(0x4E) != 0) goto dc82;
        r->offset = 0x01;
        routine_0165(r);
        if (r->carry) goto dc82;
        if (player_x_fine == 0) goto dc82;
        r->offset = 0x0D;
        routine_0165(r);

    } else {

        GAME_MEM8(0x50) = 0x00;

    }

dc82:
    GAME_MEM8(0x4E) = 0x00;
}
