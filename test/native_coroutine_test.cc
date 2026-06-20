#include "native/lotw_scripts.hpp"

#include <cstdio>
#include <cstring>

#include "ram.h"

extern "C" {
u8 NES_MEM[0x10000];
}

static int expect_u8(const char* name, u16 addr, u8 want)
{
    u8 got = RAM8(addr);
    if (got == want)
        return 0;
    std::fprintf(stderr, "%s $%04X: got %02X, expected %02X\n", name, addr, got, want);
    return 1;
}

int main()
{
    std::memset(NES_MEM, 0, sizeof NES_MEM);

    lotw::native::GameState game;
    auto gate = lotw::native::ae11_press_start_gate(game);

    int errors = 0;
    if (gate.step({lotw::native::kButtonStart}))
        errors |= 1;
    errors |= expect_u8("prompt state while held", 0x008F, 0x03);
    errors |= expect_u8("dialog depth while held", 0x008D, 0x01);

    if (gate.step({0x00}))
        errors |= 1;
    errors |= expect_u8("prompt state after release", 0x008F, 0x03);

    if (gate.step({lotw::native::kButtonStart}))
        errors |= 1;
    if (gate.step({lotw::native::kButtonStart}))
        errors |= 1;

    if (!gate.step({0x00}))
        errors |= 1;
    errors |= expect_u8("prompt state done", 0x008F, 0x04);
    errors |= expect_u8("dialog depth done", 0x008D, 0x00);

    auto release = lotw::native::wait_buttons_released();
    if (release.step({0x01}))
        errors |= 1;
    if (!release.step({0x00}))
        errors |= 1;

    auto press = lotw::native::wait_any_button_pressed();
    if (press.step({0x00}))
        errors |= 1;
    if (!press.step({0x40}))
        errors |= 1;

    auto release_then_press = lotw::native::wait_release_then_any_press();
    if (release_then_press.step({0x04}))
        errors |= 1;
    if (release_then_press.step({0x00}))
        errors |= 1;
    if (!release_then_press.step({0x20}))
        errors |= 1;

    auto transition_press = lotw::native::wait_release_then_button_then_release(0x10);
    if (transition_press.step({0x10}))
        errors |= 1;
    if (transition_press.step({0x00}))
        errors |= 1;
    if (transition_press.step({0x08}))
        errors |= 1;
    if (transition_press.step({0x10}))
        errors |= 1;
    if (!transition_press.step({0x00}))
        errors |= 1;

    auto two_frames = lotw::native::wait_frames(2);
    if (two_frames.step({0x00, false}))
        errors |= 1;
    if (two_frames.step({0x00, false}))
        errors |= 1;
    if (two_frames.step({0x00, true}))
        errors |= 1;
    if (!two_frames.step({0x00, true}))
        errors |= 1;

    return errors ? 1 : 0;
}
