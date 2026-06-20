#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C569(Regs* r);
extern "C" void sub_C9FB(Regs* r);

namespace {

lotw::native::FrameTask sub_C540_script(Regs* r)
{
    lotw::native::GameState game;
    u8 x = r->x;

    do {
        for (int i = 0x1F; i >= 0; --i)
            RAM8((u16)(0x0180 + i)) = 0x30;
        sub_C569(r);
        game.set_frame_counter(0x01);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        sub_C9FB(r);
        sub_C569(r);
        game.set_frame_counter(0x02);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
    } while (--x != 0);
    r->x = x;
}

} // namespace

extern "C" void sub_C540(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_C540_script(r),
        [] { return std::uint8_t{0}; });
}
