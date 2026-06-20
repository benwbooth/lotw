#include "native/frame_wait_helpers.hpp"

#include "game_memory.h"
#include "routine_context.h"

extern "C" void routine_0127(RoutineContext* r);
extern "C" void routine_0050(RoutineContext* r);
extern "C" void routine_0123(RoutineContext* r);
extern "C" void routine_0061(RoutineContext* r);
extern "C" void routine_0062(RoutineContext* r);
extern "C" void routine_0133(RoutineContext* r);
extern "C" void routine_0103(RoutineContext* r);
extern "C" void routine_0069(RoutineContext* r);
extern "C" void routine_0066(RoutineContext* r);
extern "C" void routine_0128(RoutineContext* r);
extern "C" void routine_0063(RoutineContext* r);
extern "C" void routine_0065(RoutineContext* r);
extern "C" void routine_0072(RoutineContext* r);
extern "C" void routine_0071(RoutineContext* r);
extern "C" void routine_0130(RoutineContext* r);
extern "C" void routine_0076(RoutineContext* r);
extern "C" void routine_0093(RoutineContext* r);
extern "C" void routine_0094(RoutineContext* r);
extern "C" void routine_0095(RoutineContext* r);
extern "C" void routine_0096(RoutineContext* r);
extern "C" void scene_assemble(RoutineContext* r);
extern "C" void queue_ppu_job_and_wait(RoutineContext* r);

namespace {

using RoutineFn = void (*)(RoutineContext*);

void enter_return_home(u8 lo, u8 hi)
{
    GAME_MEM8(0x0E) = lo;
    GAME_MEM8(0x0F) = hi;
    GAME_MEM8(0x30) = GAME_MEM8(0x32);
    GAME_MEM8(0x31) = GAME_MEM8(0x33);
    GAME_MEM8(0x25) = 0x06;
    LOTW_BANK_SYNC();
}

void leave_return_home()
{
    GAME_MEM8(0x30) = 0x0C;
    GAME_MEM8(0x31) = 0x0D;
    GAME_MEM8(0x25) = 0x07;
    LOTW_BANK_SYNC();
}

void farcall_cce4(RoutineContext* r, u8 lo, u8 hi, RoutineFn target)
{
    enter_return_home(lo, hi);
    target(r);
    leave_return_home();
}

void vram_blit(RoutineContext* r, u8 dlo, u8 dhi, u8 slo, u8 shi, u8 len)
{
    GAME_MEM8(0x16) = dlo;
    GAME_MEM8(0x17) = dhi;
    GAME_MEM8(0x18) = slo;
    GAME_MEM8(0x19) = shi;
    GAME_MEM8(0x1A) = len;
    r->value = 0x05;
    queue_ppu_job_and_wait(r);
}

lotw::native::FrameTask frame_task_0008_script(RoutineContext* r)
{
    lotw::native::GameState game;
    const u8 saved_song = GAME_MEM8(0x8E);

    game.push_dialog_depth();
    routine_0127(r);
    r->index = 0x35;
    r->offset = 0x00;
    routine_0050(r);

    game.set_frame_counter(0x3C);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    r->value = 0x08;
    routine_0123(r);
    game.pop_dialog_depth();

    GAME_MEM8(0x0A) = 0x05;
    do {
        r->index = 0x0D; r->offset = 0x00; routine_0050(r);
        r->index = 0x01; r->offset = 0x00; routine_0050(r);
        r->index = 0x09; r->offset = 0x00; routine_0050(r);
        r->index = 0x01; r->offset = 0x40; routine_0050(r);
        GAME_MEM8(0x0A) = (u8)(GAME_MEM8(0x0A) - 1);
    } while (GAME_MEM8(0x0A) != 0);

    game.set_frame_counter(0x01);
    GAME_MEM8(0x56) = 0x31;
    routine_0061(r);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    bool use_game_over_screen = GAME_MEM8(0xEC) != 0;
    if (!use_game_over_screen) {
        if (GAME_MEM8(0x37) & 0x80) {
            const u8 x = GAME_MEM8(0x55);
            if (GAME_MEM8((u16)(0x51 + x)) == 0x0C) {
                GAME_MEM8((u16)(0x51 + x)) = 0xFF;
                routine_0062(r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            GAME_MEM8(0x37) = (u8)(GAME_MEM8(0x37) + 1);
        }

        if (!use_game_over_screen) {
            routine_0133(r);
            GAME_MEM8(0x56) = 0x19;
            routine_0103(r);
            r->value = saved_song;
            routine_0123(r);
            r->index = 0x00;
            co_return;
        }
    }

    routine_0069(r);
    GAME_MEM8(0xEC) = 0x00;
    GAME_MEM8(0x3E) = 0x00;
    GAME_MEM8(0x3F) = 0x80;
    routine_0066(r);
    routine_0128(r);
    routine_0063(r);
    GAME_MEM8(0x2B) = 0x16;
    GAME_MEM8(0x2C) = 0x36;
    GAME_MEM8(0x1C) = 0x00;
    GAME_MEM8(0x1D) = 0x00;
    GAME_MEM8(0x1E) = 0x00;
    GAME_MEM8(0x7B) = 0x00;
    GAME_MEM8(0x7C) = 0x00;

    vram_blit(r, 0x6B, 0x21, 0xAF, 0xB4, 0x09);
    vram_blit(r, 0x4C, 0x22, 0xB8, 0xB4, 0x05);
    vram_blit(r, 0x8C, 0x22, 0xBD, 0xB4, 0x08);

    GAME_MEM8(0x44) = 0x05;
    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x45) = 0x70;
    GAME_MEM8(0x56) = 0x39;
    routine_0065(r);
    routine_0061(r);
    farcall_cce4(r, 0xE0, 0xC4, routine_0072);

    for (;;) {
        routine_0103(r);
        if (r->value & 0x10)
            break;
        GAME_MEM8(0x45) = (u8)(GAME_MEM8(0x45) ^ 0x10);
        game.set_prompt_state(0x0C);
    }

    game.set_prompt_state(0x18);
    if (GAME_MEM8(0x45) != 0x70) {
        routine_0069(r);
        game.set_frame_counter(0x78);
        enter_return_home(0x35, 0xC1);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        leave_return_home();
        r->index = 0x02;
        co_return;
    }

    routine_0130(r);
    GAME_MEM8(0x51) = 0xFF;
    GAME_MEM8(0x52) = 0xFF;
    GAME_MEM8(0x53) = 0xFF;
    GAME_MEM8(0x55) = 0x03;
    GAME_MEM8(0x40) = 0x06;
    GAME_MEM8(0x47) = 0x03;
    GAME_MEM8(0x48) = 0x10;
    routine_0069(r);
    GAME_MEM8(0x8E) = 0x02;
    routine_0066(r);
    routine_0076(r);
    routine_0093(r);
    routine_0094(r);
    routine_0095(r);
    routine_0096(r);
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);

    r->value = 0x0F;
    for (int x = 0x1F; x >= 0; x--)
        GAME_MEM8((u16)(0x0180 + x)) = 0x0F;
    GAME_MEM8(0x0210) = 0xEF;
    GAME_MEM8(0x0214) = 0xEF;
    farcall_cce4(r, 0xB4, 0xC4, routine_0071);
    r->index = 0x01;
}

}

extern "C" void routine_0049(RoutineContext* r)
{
    lotw::native::run_frame_task(
        r,
        frame_task_0008_script(r),
        [] { return std::uint8_t{0}; });
}
