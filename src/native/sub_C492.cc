#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_C9FB(Regs* r);
extern "C" void sub_C520(Regs* r);
extern "C" void sub_C569(Regs* r);

namespace {

lotw::native::FrameTask sub_C492_script(Regs* r)
{
    lotw::native::GameState game;

    u8 v = 0x40;
    RAM8(0x09) = v;
    do {
        game.set_frame_counter(0x05);
        sub_C9FB(r);
        r->x = 0x04;
        r->y = 0x1C;
        sub_C520(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();
        v = (u8)(RAM8(0x09) - 0x10);
        RAM8(0x09) = v;
    } while ((v & 0x80) == 0);
    sub_C569(r);
}

} // namespace

extern "C" void sub_C492(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_C492_script(r),
        [] { return std::uint8_t{0}; });
}
