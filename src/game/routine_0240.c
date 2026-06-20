




















#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);
void routine_0115(RoutineContext *r);
void routine_0241(RoutineContext *r);
void routine_0250(RoutineContext *r);

static const u8 drop_item_table[9] =
    { 0x03,0x03,0x03,0x03,0x04,0x04,0x05,0x06,0x07 };

static void item_spawn_setup(RoutineContext *r, u8 x)
{
    GAME_MEM8(0xEE) = (u8)(x + 2);
    GAME_MEM8(0xED) = (u8)((x << 2) | 0x81);
    GAME_MEM8(0xEF) = 0x01;
    GAME_MEM8(0xFB) = GAME_MEM8(0xFC);
    GAME_MEM8(0xF3) = 0xF0;
    GAME_MEM8(0xF0) = 0x00;
    GAME_MEM8(0xF1) = 0x00;
    routine_0250(r);
}

void routine_0240(RoutineContext *r)
{
    if ((GAME_MEM8(0xEE) & 0x7F) == 0) {
        GAME_MEM8(0xEE) = (u8)(GAME_MEM8(0xEE) + 1);
        GAME_MEM8(0x8F) = 0x0E;
        GAME_MEM8(0xF1) = 0x08;
        GAME_MEM8(0xF5) = 0x00;
        GAME_MEM8(0xF6) = 0x00;
        GAME_MEM8(0xF0) = 0x00;
        GAME_MEM8(0xFC) = GAME_MEM8(0xFB);
        {
            u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
            GAME_MEM8(0xED) = GAME_MEM8((u16)(ptr + 6));
        }
        GAME_MEM8(0xEF) = (u8)(GAME_MEM8(0xEF) & 0x03);
    }

    if (GAME_MEM8(0xF0) == 0) {
        GAME_MEM8(0xF1) = (u8)(GAME_MEM8(0xF1) - 1);
        if (GAME_MEM8(0xF1) == 0)
            goto flow_0559;
        GAME_MEM8(0xF7) = (u8)(0 - GAME_MEM8(0xF1));
        routine_0241(r);
        routine_0115(r);
        if (r->carry) goto flow_0559;
        GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
        return;
    flow_0559:
        GAME_MEM8(0xEF) = (u8)(GAME_MEM8(0xEF) | 0x80);
        GAME_MEM8(0xF0) = 0x01;
        return;
    }


    GAME_MEM8(0xF0) = (u8)(GAME_MEM8(0xF0) + 1);
    GAME_MEM8(0xF7) = (u8)((GAME_MEM8(0xF0) >> 1) + 2);
    routine_0241(r);
    routine_0115(r);
    if (!r->carry) {
        GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
        return;
    }


    {
        u8 x = 0x00;
        if (health < 0x14) { item_spawn_setup(r, x); return; }
        x = 0x01;
        if (magic < 0x1E) { item_spawn_setup(r, x); return; }
        x = 0x04;
        if (keys < 0x02) { item_spawn_setup(r, x); return; }

        r->value = 0x14;
        rng_update(r);
        if (r->value >= 0x09) {

            x = 0x00;
            if (health < magic) {
                if (health < gold) { item_spawn_setup(r, x); return; }
                x = 0x02;
                item_spawn_setup(r, x);
                return;
            }
            x = 0x01;
            if (magic < gold) { item_spawn_setup(r, x); return; }
            x = 0x02;
            item_spawn_setup(r, x);
            return;
        }
        x = drop_item_table[r->value];
        item_spawn_setup(r, x);
        return;
    }
}
