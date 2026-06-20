#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0059(RoutineContext* r);
extern "C" void routine_0063(RoutineContext* r);
extern "C" void routine_0266(RoutineContext* r);
extern "C" void routine_0212(RoutineContext* r);
extern "C" void routine_0271(RoutineContext* r);
extern "C" void routine_0049(RoutineContext* r);
extern "C" void routine_0001(RoutineContext* r);
extern "C" void routine_0021(RoutineContext* r);
extern "C" void routine_0005(RoutineContext* r);
extern "C" void routine_0014(RoutineContext* r);
extern "C" void routine_0002(RoutineContext* r);
extern "C" void game_update(RoutineContext* r);
extern "C" void main_init(RoutineContext* r);

namespace {

using RoutineFn = void (*)(RoutineContext*);

void farcall_0C0D(RoutineContext* r, u8 lo, u8 hi, RoutineFn target)
{
    const u8 old6 = GAME_MEM8(0x30);
    const u8 old7 = GAME_MEM8(0x31);
    GAME_MEM8(0x32) = old6;
    GAME_MEM8(0x33) = old7;
    GAME_MEM8(0x0E) = lo;
    GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = 0x0C;
    GAME_MEM8(0x31) = 0x0D;
    GAME_MEM8(0x25) = 0x07;
    LOTW_BANK_SYNC();
    target(r);
    GAME_MEM8(0x31) = old7;
    GAME_MEM8(0x30) = old6;
    GAME_MEM8(0x25) = 0x06;
    LOTW_BANK_SYNC();
}

lotw::native::FrameTask main_loop_dispatch_script(RoutineContext* r)
{
    lotw::native::GameState game;

    for (;;) {
        if (game.player_health() == 0) {
            game.set_sprite_blink_timer(0x00);
            routine_0061(r);
            farcall_0C0D(r, 0x07, 0xB3, routine_0049);
            if (r->index == 0)
                continue;
            r->index = (u8)(r->index - 1);
            main_init(r);
            co_return;
        }

        game.set_frame_counter(0x01);
        GAME_MEM8(0x7E) = GAME_MEM8(0x7C);
        lotw::native::read_buttons(r);
        game_update(r);

        if (GAME_MEM8(0xEC) != 0) {
            farcall_0C0D(r, 0xEB, 0xA2, routine_0001);
            do {
                lotw::native::read_buttons(r);
                farcall_0C0D(r, 0xBC, 0xAB, routine_0021);
                farcall_0C0D(r, 0xE6, 0xA5, routine_0005);
                farcall_0C0D(r, 0x5D, 0xA7, routine_0014);
                farcall_0C0D(r, 0xE3, 0xA3, routine_0002);
            } while (game.player_health() == 0);

            GAME_MEM8(0x44) = (u8)(GAME_MEM8(0x43) >> 4);
            GAME_MEM8(0x43) = (u8)(GAME_MEM8(0x43) & 0x0F);
            GAME_MEM8(0x0200) = 0xEF;
            game.set_sprite_blink_timer(0x00);
            routine_0061(r);
            farcall_0C0D(r, 0x07, 0xB3, routine_0049);
            r->index = (u8)(r->index - 1);
            main_init(r);
            co_return;
        }

        routine_0266(r);
        routine_0212(r);
        routine_0271(r);
        routine_0059(r);
        {
            const u8 saved_c = r->carry;
            routine_0061(r);
            routine_0063(r);
            r->carry = saved_c;
        }
        if (!r->carry && GAME_MEM8(0x7E) != GAME_MEM8(0x7C))
            GAME_MEM8(0x3D)++;

        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    }
}

}

extern "C" void main_loop_dispatch(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        main_loop_dispatch_script(r),
        [] { return std::uint8_t{0}; });
}
