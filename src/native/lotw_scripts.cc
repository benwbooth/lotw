#include "native/lotw_scripts.hpp"

namespace lotw::native {

FrameTask ae11_press_start_gate(GameState& game)
{
    game.set_prompt_state(0x03);
    game.push_dialog_depth();

    co_yield Wait::buttons_released();
    co_yield Wait::button_pressed(kButtonStart);
    co_yield Wait::buttons_released();

    game.set_prompt_state(0x04);
    game.pop_dialog_depth();
}

FrameTask wait_buttons_released()
{
    co_yield Wait::buttons_released();
}

FrameTask wait_any_button_pressed()
{
    co_yield Wait::button_pressed(0xFF);
}

FrameTask wait_release_then_any_press()
{
    co_yield Wait::buttons_released();
    co_yield Wait::button_pressed(0xFF);
}

FrameTask wait_release_then_button_then_release(std::uint8_t mask)
{
    co_yield Wait::buttons_released();
    co_yield Wait::button_pressed(mask);
    co_yield Wait::buttons_released();
}

FrameTask wait_frames(unsigned count)
{
    while (count-- != 0)
        co_yield Wait::next_frame();
}

}
