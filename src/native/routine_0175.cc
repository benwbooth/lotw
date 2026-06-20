#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

#define map_screen_y     GAME_MEM8(0x48)
#define player_x_fine_v  GAME_MEM8(0x43)
#define player_y_v       GAME_MEM8(0x45)
#define carried_item0    0x51
#define equipped_item    GAME_MEM8(0x55)
#define stat_jump        0x5C
#define inventory_counts 0x60
#define scroll_x_fine    GAME_MEM8(0x7B)
#define mmc3_r2_shadow   GAME_MEM8(0x2C)
#define mmc3_r3_shadow   GAME_MEM8(0x2D)
#define mmc3_r4_shadow   GAME_MEM8(0x2E)
#define mmc3_r5_shadow   GAME_MEM8(0x2F)

extern "C" void routine_0192(RoutineContext* r);
extern "C" void routine_0193(RoutineContext* r);
extern "C" void routine_0195(RoutineContext* r);
extern "C" void routine_0199(RoutineContext* r);
extern "C" void routine_0070(RoutineContext* r);
extern "C" void routine_0189(RoutineContext* r);
extern "C" void routine_0096(RoutineContext* r);
extern "C" void routine_0068(RoutineContext* r);
extern "C" void routine_0133(RoutineContext* r);
extern "C" void routine_0134(RoutineContext* r);
extern "C" void routine_0196(RoutineContext* r);
extern "C" void routine_0197(RoutineContext* r);
extern "C" void routine_0117(RoutineContext* r);
extern "C" void routine_0119(RoutineContext* r);
extern "C" void routine_0060(RoutineContext* r);
extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0188(RoutineContext* r);
extern "C" void routine_0200(RoutineContext* r);
extern "C" void routine_0129(RoutineContext* r);
extern "C" void routine_0093(RoutineContext* r);
extern "C" void routine_0094(RoutineContext* r);
extern "C" void routine_0062(RoutineContext* r);
extern "C" void routine_0201(RoutineContext* r);
extern "C" void routine_0128(RoutineContext* r);
extern "C" void routine_0191(RoutineContext* r);
extern "C" void routine_0176(RoutineContext* r);
extern "C" void routine_0177(RoutineContext* r);
extern "C" void routine_0074(RoutineContext* r);
extern "C" void routine_0127(RoutineContext* r);
extern "C" void routine_0067(RoutineContext* r);
extern "C" void song_init(RoutineContext* r);

namespace {

lotw::native::FrameTask frame_task_0021_script(RoutineContext* r)
{
    lotw::native::GameState game;

    if (map_screen_y != 0x10)
        goto flow_0416;
    goto flow_0418;

flow_0416:
    routine_0193(r);
    r->value = 0x04;
    routine_0195(r);
    routine_0199(r);
    routine_0070(r);

flow_0417:
    routine_0189(r);
    if (r->carry)
        goto flow_0446;
    if (gold < 0x0A) {
        game.set_prompt_state(0x06);
        goto flow_0417;
    }

    {
        u8 x = 0x0A;
        do {
            gold = (u8)(gold - 1);
            routine_0096(r);
            game.set_prompt_state(0x0C);
            game.set_frame_counter(0x0A);
            lotw::native::commit_frame_work(r);
            while (game.frame_counter_active())
                co_yield lotw::native::Wait::next_frame();
            x = (u8)(x - 1);
        } while (x != 0);
    }
    routine_0068(r);
    routine_0133(r);
    routine_0134(r);
    r->value = 0x08; routine_0196(r);
    routine_0197(r);
    routine_0117(r);
    routine_0119(r);
    scroll_x_fine = 0x08;
    routine_0060(r);
    routine_0061(r);
    routine_0070(r);
    routine_0188(r);
    r->value = 0x04; routine_0196(r);
    routine_0200(r);
    routine_0199(r);
    routine_0070(r);
    goto flow_0417;

flow_0418:
    game.set_player_health(0x00);
    game.set_player_magic(0x00);
    if (cur_character < 0x06) {
        for (int y = 2; y >= 0; y--) {
            const u8 x = GAME_MEM8((u16)(carried_item0 + y));
            if ((x & 0x80) == 0)
                GAME_MEM8((u16)(inventory_counts + x))++;
            GAME_MEM8((u16)(carried_item0 + y)) = 0xFF;
        }
        routine_0129(r);
    }

    routine_0193(r);
    cur_character = 0x06;
    r->value = 0x06;
    routine_0195(r);
    routine_0093(r);
    routine_0094(r);
    equipped_item = 0x03;
    routine_0062(r);
    GAME_MEM8(0x56) = 0xF1;
    GAME_MEM8(0x57) = 0x00;
    routine_0061(r);
    routine_0201(r);
    routine_0128(r);
    routine_0070(r);

flow_0419:
    routine_0191(r);
    {
        u8 hi = (u8)(GAME_MEM8(0x0A) & 0xF0);
        if (hi != 0x50)
            goto flow_0420;

        if ((GAME_MEM8(0x0F) & 0x0F) != 0x05)
            goto flow_0419;
        if (GAME_MEM8(0x37) == 0)
            goto flow_0419;
        {
            u8 x = (u8)(GAME_MEM8(0x8E) + 1);
            if (x >= 0x10)
                x = 0x00;
            GAME_MEM8(0x8E) = x;
        }
        song_init(r);
        if ((GAME_MEM8(0x37) & 0x80) == 0)
            goto flow_0419;
        if (game.buttons() != 0xC3)
            goto flow_0419;
        for (int x = 0x0D; x >= 0; x--)
            GAME_MEM8((u16)(inventory_counts + x)) = 0x10;
        GAME_MEM8(0x37) = 0x80;
        gold = 0x80;
        keys = 0x80;
        game.set_prompt_state(0x1A);
        goto flow_0419;
    }

flow_0420:
    {
        u8 x = 0x00;
        u8 hi = (u8)(GAME_MEM8(0x0A) & 0xF0);
        if (hi == 0x70)
            goto flow_0421;
        x = 0x02;
        if (hi == 0x80)
            goto flow_0422;
        if (hi != 0x90)
            goto flow_0419;
        x = 0x03;
        {
            u8 lo = (u8)(GAME_MEM8(0x0F) & 0x0F);
            if (lo == 0x06)
                goto flow_0423;
            x = (u8)(x + 1);
            if (lo == 0x0A)
                goto flow_0423;
            goto flow_0419;
        }

    flow_0421:
        {
            u8 lo = (u8)(GAME_MEM8(0x0F) & 0x0F);
            if (lo == 0x06)
                goto flow_0423;
            x = (u8)(x + 1);
            if (lo == 0x08)
                goto flow_0423;
            goto flow_0419;
        }

    flow_0422:
        {
            u8 lo = (u8)(GAME_MEM8(0x0F) & 0x0F);
            if (lo == 0x04)
                goto flow_0423;
            if (lo != 0x0A) {
                if (lo == 0x0C) {
                    game.set_prompt_state(0x03);
                    routine_0177(r);
                }
                goto flow_0419;
            }
            game.set_prompt_state(0x03);
            routine_0176(r);
            goto flow_0419;
        }

    flow_0423:
        cur_character = x;
        {
            u8 a = x;
            a = (u8)(a << 1);
            a = (u8)(a << 1);
            a = (u8)(a + 0x03);
            r->offset = a;
        }
        for (int xi = 3; xi >= 0; xi--) {
            GAME_MEM8((u16)(stat_jump + xi)) = GAME_MEM8((u16)(0xFFA7 + r->offset));
            r->offset = (u8)(r->offset - 1);
        }
        game.set_prompt_state(0x18);
        game.set_prompt_argument(0xFF);
        game.set_frame_counter(0x04);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->index = 0x05;
        routine_0074(r);
        mmc3_r2_shadow = (u8)(cur_character + 0x38);
        mmc3_r3_shadow = 0x3D;
        mmc3_r4_shadow = 0x3E;
        mmc3_r5_shadow = 0x3F;
        GAME_MEM8(0x56) = 0x0D;
        GAME_MEM8(0x57) = 0x00;
        player_y_v = (u8)(player_y_v & 0xF0);
        player_x_fine_v = 0x04;
        routine_0127(r);
        routine_0061(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->index = 0x05;
        routine_0074(r);
        game.set_frame_counter(0x78);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        routine_0067(r);
        GAME_MEM8(0x56) = 0x08;
        GAME_MEM8(0x57) = 0x00;
        game.set_player_health(0x63);
        game.set_player_magic(0x63);
        routine_0093(r);
        routine_0094(r);
        equipped_item = 0x02;
        routine_0062(r);
        r->value = 0x08;
        routine_0195(r);
        routine_0197(r);
        routine_0117(r);
        routine_0119(r);
        scroll_x_fine = 0x08;
        routine_0060(r);
        routine_0061(r);
        routine_0070(r);
        routine_0188(r);
        goto flow_0446;
    }

flow_0446:
    routine_0192(r);
}

}

extern "C" void routine_0175(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0021_script(r),
        [] { return std::uint8_t{0}; });
}

#undef map_screen_y
#undef player_x_fine_v
#undef player_y_v
#undef carried_item0
#undef equipped_item
#undef stat_jump
#undef inventory_counts
#undef scroll_x_fine
#undef mmc3_r2_shadow
#undef mmc3_r3_shadow
#undef mmc3_r4_shadow
#undef mmc3_r5_shadow
