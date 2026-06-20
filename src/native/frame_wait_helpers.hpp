#ifndef LOTW_NATIVE_FRAME_WAIT_HELPERS_HPP
#define LOTW_NATIVE_FRAME_WAIT_HELPERS_HPP

#include <cstdint>

#include "native/frame_task.hpp"
#include "native/game_state.hpp"
#include "native/lotw_scripts.hpp"
#include "ppu.h"
#include "regs.h"

extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_C2B1(Regs* r);
extern "C" void sub_C234(Regs* r);
extern "C" void read_controllers(Regs* r);

namespace lotw::native {

void commit_frame_work(Regs* r);
bool frame_runner_stop_requested() noexcept;
[[noreturn]] void exit_frame_runner_thread();

inline std::uint8_t read_buttons(Regs* r)
{
    GameState game;
    read_controllers(r);
    r->a = game.buttons();
    return r->a;
}

template <typename Sampler>
inline std::uint8_t run_frame_task(Regs* r, FrameTask task, Sampler sample)
{
    bool frame_elapsed = false;
    for (;;) {
        const std::uint8_t buttons = sample();
        if (task.step({buttons, frame_elapsed}))
            return buttons;
        nes_frame_wait(r);
        if (frame_runner_stop_requested()) {
            task = FrameTask{};
            exit_frame_runner_thread();
        }
        frame_elapsed = true;
    }
}

inline void wait_frame(Regs* r)
{
    run_frame_task(r, wait_frames(1), [] { return std::uint8_t{0}; });
}

template <typename Predicate>
inline void wait_frames_while(Regs* r, Predicate predicate)
{
    while (predicate())
        wait_frame(r);
}

inline void wait_for_frame_counter(Regs* r)
{
    GameState game;
    wait_frames_while(r, [&game] { return game.frame_counter_active(); });
}

inline std::uint8_t redraw_scene_and_read_buttons(Regs* r)
{
    GameState game;
    game.set_frame_counter(0x01);
    sub_C1D8(r);
    sub_C2B1(r);
    sub_C234(r);
    commit_frame_work(r);
    wait_for_frame_counter(r);
    return read_buttons(r);
}

inline void set_frame_counter_and_wait(Regs* r, std::uint8_t frames)
{
    GameState game;
    game.set_frame_counter(frames);
    wait_for_frame_counter(r);
}

inline void wait_for_ppu_job_idle(Regs* r)
{
    GameState game;
    wait_frames_while(r, [&game] { return game.ppu_job_pending(); });
}

inline void wait_for_countdown_timer(Regs* r)
{
    GameState game;
    wait_frames_while(r, [&game] { return game.countdown_timer_active(); });
}

} // namespace lotw::native

#endif /* LOTW_NATIVE_FRAME_WAIT_HELPERS_HPP */
