#ifndef LOTW_NATIVE_LOTW_SCRIPTS_HPP
#define LOTW_NATIVE_LOTW_SCRIPTS_HPP

#include "native/frame_task.hpp"
#include "native/game_state.hpp"

namespace lotw::native {

constexpr std::uint8_t kButtonStart = 0x10;

FrameTask ae11_press_start_gate(GameState& game);
FrameTask wait_buttons_released();
FrameTask wait_any_button_pressed();
FrameTask wait_release_then_any_press();
FrameTask wait_release_then_button_then_release(std::uint8_t mask);
FrameTask wait_frames(unsigned count);

} // namespace lotw::native

#endif /* LOTW_NATIVE_LOTW_SCRIPTS_HPP */
