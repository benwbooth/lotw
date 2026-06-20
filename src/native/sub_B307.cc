#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_D07C(Regs* r);
extern "C" void sub_B4C5(Regs* r);
extern "C" void sub_D02E(Regs* r);
extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_C234(Regs* r);
extern "C" void sub_D16A(Regs* r);
extern "C" void sub_CC09(Regs* r);
extern "C" void sub_C461(Regs* r);
extern "C" void sub_C38B(Regs* r);
extern "C" void sub_D08A(Regs* r);
extern "C" void sub_C2B1(Regs* r);
extern "C" void sub_C375(Regs* r);
extern "C" void sub_C4E0(Regs* r);
extern "C" void sub_C4B4(Regs* r);
extern "C" void sub_D0C5(Regs* r);
extern "C" void sub_C57A(Regs* r);
extern "C" void sub_CAB6(Regs* r);
extern "C" void sub_CACC(Regs* r);
extern "C" void sub_CAE2(Regs* r);
extern "C" void sub_CAF8(Regs* r);
extern "C" void scene_assemble(Regs* r);
extern "C" void queue_ppu_job_and_wait(Regs* r);

namespace {

using PortFn = void (*)(Regs*);

void enter_return_home(u8 lo, u8 hi)
{
    RAM8(0x0E) = lo;
    RAM8(0x0F) = hi;
    RAM8(0x30) = RAM8(0x32);
    RAM8(0x31) = RAM8(0x33);
    RAM8(0x25) = 0x06;
    NES_PRG_SYNC();
}

void leave_return_home()
{
    RAM8(0x30) = 0x0C;
    RAM8(0x31) = 0x0D;
    RAM8(0x25) = 0x07;
    NES_PRG_SYNC();
}

void farcall_cce4(Regs* r, u8 lo, u8 hi, PortFn target)
{
    enter_return_home(lo, hi);
    target(r);
    leave_return_home();
}

void vram_blit(Regs* r, u8 dlo, u8 dhi, u8 slo, u8 shi, u8 len)
{
    RAM8(0x16) = dlo;
    RAM8(0x17) = dhi;
    RAM8(0x18) = slo;
    RAM8(0x19) = shi;
    RAM8(0x1A) = len;
    r->a = 0x05;
    queue_ppu_job_and_wait(r);
}

lotw::native::FrameTask sub_B307_script(Regs* r)
{
    lotw::native::GameState game;
    const u8 saved_song = RAM8(0x8E);

    game.push_dialog_depth();
    sub_D07C(r);
    r->x = 0x35;
    r->y = 0x00;
    sub_B4C5(r);

    game.set_frame_counter(0x3C);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    r->a = 0x08;
    sub_D02E(r);
    game.pop_dialog_depth();

    RAM8(0x0A) = 0x05;
    do {
        r->x = 0x0D; r->y = 0x00; sub_B4C5(r);
        r->x = 0x01; r->y = 0x00; sub_B4C5(r);
        r->x = 0x09; r->y = 0x00; sub_B4C5(r);
        r->x = 0x01; r->y = 0x40; sub_B4C5(r);
        RAM8(0x0A) = (u8)(RAM8(0x0A) - 1);
    } while (RAM8(0x0A) != 0);

    game.set_frame_counter(0x01);
    RAM8(0x56) = 0x31;
    sub_C1D8(r);
    lotw::native::commit_frame_work(r);
    while (game.frame_counter_active())
        co_yield lotw::native::Wait::next_frame();

    bool use_game_over_screen = RAM8(0xEC) != 0;
    if (!use_game_over_screen) {
        if (RAM8(0x37) & 0x80) {
            const u8 x = RAM8(0x55);
            if (RAM8((u16)(0x51 + x)) == 0x0C) {
                RAM8((u16)(0x51 + x)) = 0xFF;
                sub_C234(r);
            } else {
                use_game_over_screen = true;
            }
        } else {
            RAM8(0x37) = (u8)(RAM8(0x37) + 1);
        }

        if (!use_game_over_screen) {
            sub_D16A(r);
            RAM8(0x56) = 0x19;
            sub_CC09(r);
            r->a = saved_song;
            sub_D02E(r);
            r->x = 0x00;
            co_return;
        }
    }

    sub_C461(r);
    RAM8(0xEC) = 0x00;
    RAM8(0x3E) = 0x00;
    RAM8(0x3F) = 0x80;
    sub_C38B(r);
    sub_D08A(r);
    sub_C2B1(r);
    RAM8(0x2B) = 0x16;
    RAM8(0x2C) = 0x36;
    RAM8(0x1C) = 0x00;
    RAM8(0x1D) = 0x00;
    RAM8(0x1E) = 0x00;
    RAM8(0x7B) = 0x00;
    RAM8(0x7C) = 0x00;

    vram_blit(r, 0x6B, 0x21, 0xAF, 0xB4, 0x09);
    vram_blit(r, 0x4C, 0x22, 0xB8, 0xB4, 0x05);
    vram_blit(r, 0x8C, 0x22, 0xBD, 0xB4, 0x08);

    RAM8(0x44) = 0x05;
    RAM8(0x43) = 0x00;
    RAM8(0x45) = 0x70;
    RAM8(0x56) = 0x39;
    sub_C375(r);
    sub_C1D8(r);
    farcall_cce4(r, 0xE0, 0xC4, sub_C4E0);

    for (;;) {
        sub_CC09(r);
        if (r->a & 0x10)
            break;
        RAM8(0x45) = (u8)(RAM8(0x45) ^ 0x10);
        game.set_prompt_state(0x0C);
    }

    game.set_prompt_state(0x18);
    if (RAM8(0x45) != 0x70) {
        sub_C461(r);
        game.set_frame_counter(0x78);
        enter_return_home(0x35, 0xC1);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        leave_return_home();
        r->x = 0x02;
        co_return;
    }

    sub_D0C5(r);
    RAM8(0x51) = 0xFF;
    RAM8(0x52) = 0xFF;
    RAM8(0x53) = 0xFF;
    RAM8(0x55) = 0x03;
    RAM8(0x40) = 0x06;
    RAM8(0x47) = 0x03;
    RAM8(0x48) = 0x10;
    sub_C461(r);
    RAM8(0x8E) = 0x02;
    sub_C38B(r);
    sub_C57A(r);
    sub_CAB6(r);
    sub_CACC(r);
    sub_CAE2(r);
    sub_CAF8(r);
    farcall_cce4(r, 0xF2, 0xC8, scene_assemble);

    r->a = 0x0F;
    for (int x = 0x1F; x >= 0; x--)
        RAM8((u16)(0x0180 + x)) = 0x0F;
    RAM8(0x0210) = 0xEF;
    RAM8(0x0214) = 0xEF;
    farcall_cce4(r, 0xB4, 0xC4, sub_C4B4);
    r->x = 0x01;
}

} // namespace

extern "C" void sub_B307(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_B307_script(r),
        [] { return std::uint8_t{0}; });
}
