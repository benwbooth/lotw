#include "native/frame_wait_helpers.hpp"
#include "native/lotw_scripts.hpp"

#include "ram.h"
#include "regs.h"

extern "C" void sub_E5FD(Regs* r);
extern "C" void sub_E620(Regs* r);
extern "C" void sub_E660(Regs* r);
extern "C" void sub_E6FF(Regs* r);
extern "C" void sub_CFBC(Regs* r);
extern "C" void sub_E778(Regs* r);
extern "C" void sub_C492(Regs* r);
extern "C" void sub_E514(Regs* r);
extern "C" void sub_E842(Regs* r);

static void wait_release(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        lotw::native::wait_buttons_released(),
        [r] { return lotw::native::read_buttons(r); });
}

extern "C" void sub_E424(Regs* r)
{
    lotw::native::GameState game;

    sub_E620(r);

    {
        const u8 s80 = RAM8(0x80);
        const u8 s81 = RAM8(0x81);
        const u8 s82 = RAM8(0x82);
        const u8 s83 = RAM8(0x83);
        r->a = RAM8(0x47);
        sub_E660(r);
        RAM8(0x83) = s83;
        RAM8(0x82) = s82;
        RAM8(0x81) = s81;
        RAM8(0x80) = s80;
    }

    sub_E6FF(r);
    sub_CFBC(r);
    sub_E778(r);
    sub_C492(r);

    for (;;) {
        sub_E514(r);
        if (r->c) {
            sub_E5FD(r);
            return;
        }

        const u8 nib = RAM8(0x44) & 0x0F;
        u8 x;
        if (nib < 0x03) {
            continue;
        }
        if (nib < 0x05) {
            x = 0x00;
        } else {
            x = 0x02;
            if (nib < 0x0A || nib >= 0x0C)
                continue;
        }

        const u8 item = RAM8((u16)(0x80 + x));
        if (item & 0x80) {
            game.set_prompt_state(0x06);
        } else {
            const u8 price = RAM8((u16)(0x81 + x));
            r->a = price;
            sub_E842(r);
            if (r->c) {
                RAM8((u16)(0x80 + x)) = 0xFF;
                sub_E6FF(r);
                RAM8((u16)(0x60 + item))++;
                game.set_prompt_state(0x10);
            } else {
                if (item == 0x0D && RAM8(0x37) != 0)
                    RAM8(0x61) = 0x01;
                game.set_prompt_state(0x06);
            }
        }

        wait_release(r);
    }
}
