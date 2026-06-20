#include "native/frame_wait_helpers.hpp"

#include "ram.h"
#include "regs.h"

#define map_screen_y     RAM8(0x48)
#define player_x_fine_v  RAM8(0x43)
#define player_y_v       RAM8(0x45)
#define carried_item0    0x51
#define equipped_item    RAM8(0x55)
#define stat_jump        0x5C
#define inventory_counts 0x60
#define scroll_x_fine    RAM8(0x7B)
#define mmc3_r2_shadow   RAM8(0x2C)
#define mmc3_r3_shadow   RAM8(0x2D)
#define mmc3_r4_shadow   RAM8(0x2E)
#define mmc3_r5_shadow   RAM8(0x2F)

extern "C" void sub_E5FD(Regs* r);
extern "C" void sub_E620(Regs* r);
extern "C" void sub_E660(Regs* r);
extern "C" void sub_E778(Regs* r);
extern "C" void sub_C492(Regs* r);
extern "C" void sub_E514(Regs* r);
extern "C" void sub_CAF8(Regs* r);
extern "C" void sub_C430(Regs* r);
extern "C" void sub_D16A(Regs* r);
extern "C" void sub_D199(Regs* r);
extern "C" void sub_E667(Regs* r);
extern "C" void sub_E6B7(Regs* r);
extern "C" void sub_CF30(Regs* r);
extern "C" void sub_CF82(Regs* r);
extern "C" void sub_C1C7(Regs* r);
extern "C" void sub_C1D8(Regs* r);
extern "C" void sub_E4AA(Regs* r);
extern "C" void sub_E79D(Regs* r);
extern "C" void sub_D0A5(Regs* r);
extern "C" void sub_CAB6(Regs* r);
extern "C" void sub_CACC(Regs* r);
extern "C" void sub_C234(Regs* r);
extern "C" void sub_E7B2(Regs* r);
extern "C" void sub_D08A(Regs* r);
extern "C" void sub_E5B4(Regs* r);
extern "C" void sub_E27D(Regs* r);
extern "C" void sub_E2AA(Regs* r);
extern "C" void sub_C540(Regs* r);
extern "C" void sub_D07C(Regs* r);
extern "C" void sub_C3E5(Regs* r);
extern "C" void song_init(Regs* r);

namespace {

lotw::native::FrameTask sub_E077_script(Regs* r)
{
    lotw::native::GameState game;

    if (map_screen_y != 0x10)
        goto L_E080;
    goto L_E0F4;

L_E080:
    sub_E620(r);
    r->a = 0x04;
    sub_E660(r);
    sub_E778(r);
    sub_C492(r);

L_E08E:
    sub_E514(r);
    if (r->c)
        goto L_E5FD;
    if (gold < 0x0A) {
        game.set_prompt_state(0x06);
        goto L_E08E;
    }

    {
        u8 x = 0x0A;
        do {
            gold = (u8)(gold - 1);
            sub_CAF8(r);
            game.set_prompt_state(0x0C);
            game.set_frame_counter(0x0A);
            lotw::native::commit_frame_work(r);
            while (game.frame_counter_active())
                co_yield lotw::native::Wait::next_frame();
            x = (u8)(x - 1);
        } while (x != 0);
    }
    sub_C430(r);
    sub_D16A(r);
    sub_D199(r);
    r->a = 0x08; sub_E667(r);
    sub_E6B7(r);
    sub_CF30(r);
    sub_CF82(r);
    scroll_x_fine = 0x08;
    sub_C1C7(r);
    sub_C1D8(r);
    sub_C492(r);
    sub_E4AA(r);
    r->a = 0x04; sub_E667(r);
    sub_E79D(r);
    sub_E778(r);
    sub_C492(r);
    goto L_E08E;

L_E0F4:
    game.set_player_health(0x00);
    game.set_player_magic(0x00);
    if (cur_character < 0x06) {
        for (int y = 2; y >= 0; y--) {
            const u8 x = RAM8((u16)(carried_item0 + y));
            if ((x & 0x80) == 0)
                RAM8((u16)(inventory_counts + x))++;
            RAM8((u16)(carried_item0 + y)) = 0xFF;
        }
        sub_D0A5(r);
    }

    sub_E620(r);
    cur_character = 0x06;
    r->a = 0x06;
    sub_E660(r);
    sub_CAB6(r);
    sub_CACC(r);
    equipped_item = 0x03;
    sub_C234(r);
    RAM8(0x56) = 0xF1;
    RAM8(0x57) = 0x00;
    sub_C1D8(r);
    sub_E7B2(r);
    sub_D08A(r);
    sub_C492(r);

L_E13F:
    sub_E5B4(r);
    {
        u8 hi = (u8)(RAM8(0x0A) & 0xF0);
        if (hi != 0x50)
            goto L_E186;

        if ((RAM8(0x0F) & 0x0F) != 0x05)
            goto L_E13F;
        if (RAM8(0x37) == 0)
            goto L_E13F;
        {
            u8 x = (u8)(RAM8(0x8E) + 1);
            if (x >= 0x10)
                x = 0x00;
            RAM8(0x8E) = x;
        }
        song_init(r);
        if ((RAM8(0x37) & 0x80) == 0)
            goto L_E13F;
        if (game.buttons() != 0xC3)
            goto L_E13F;
        for (int x = 0x0D; x >= 0; x--)
            RAM8((u16)(inventory_counts + x)) = 0x10;
        RAM8(0x37) = 0x80;
        gold = 0x80;
        keys = 0x80;
        game.set_prompt_state(0x1A);
        goto L_E13F;
    }

L_E186:
    {
        u8 x = 0x00;
        u8 hi = (u8)(RAM8(0x0A) & 0xF0);
        if (hi == 0x70)
            goto L_E1A8;
        x = 0x02;
        if (hi == 0x80)
            goto L_E1B8;
        if (hi != 0x90)
            goto L_E13F;
        x = 0x03;
        {
            u8 lo = (u8)(RAM8(0x0F) & 0x0F);
            if (lo == 0x06)
                goto L_E1DC;
            x = (u8)(x + 1);
            if (lo == 0x0A)
                goto L_E1DC;
            goto L_E13F;
        }

    L_E1A8:
        {
            u8 lo = (u8)(RAM8(0x0F) & 0x0F);
            if (lo == 0x06)
                goto L_E1DC;
            x = (u8)(x + 1);
            if (lo == 0x08)
                goto L_E1DC;
            goto L_E13F;
        }

    L_E1B8:
        {
            u8 lo = (u8)(RAM8(0x0F) & 0x0F);
            if (lo == 0x04)
                goto L_E1DC;
            if (lo != 0x0A) {
                if (lo == 0x0C) {
                    game.set_prompt_state(0x03);
                    sub_E2AA(r);
                }
                goto L_E13F;
            }
            game.set_prompt_state(0x03);
            sub_E27D(r);
            goto L_E13F;
        }

    L_E1DC:
        cur_character = x;
        {
            u8 a = x;
            a = (u8)(a << 1);
            a = (u8)(a << 1);
            a = (u8)(a + 0x03);
            r->y = a;
        }
        for (int xi = 3; xi >= 0; xi--) {
            RAM8((u16)(stat_jump + xi)) = RAM8((u16)(0xFFA7 + r->y));
            r->y = (u8)(r->y - 1);
        }
        game.set_prompt_state(0x18);
        game.set_prompt_argument(0xFF);
        game.set_frame_counter(0x04);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->x = 0x05;
        sub_C540(r);
        mmc3_r2_shadow = (u8)(cur_character + 0x38);
        mmc3_r3_shadow = 0x3D;
        mmc3_r4_shadow = 0x3E;
        mmc3_r5_shadow = 0x3F;
        RAM8(0x56) = 0x0D;
        RAM8(0x57) = 0x00;
        player_y_v = (u8)(player_y_v & 0xF0);
        player_x_fine_v = 0x04;
        sub_D07C(r);
        sub_C1D8(r);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        r->x = 0x05;
        sub_C540(r);
        game.set_frame_counter(0x78);
        lotw::native::commit_frame_work(r);
        while (game.frame_counter_active())
            co_yield lotw::native::Wait::next_frame();

        sub_C3E5(r);
        RAM8(0x56) = 0x08;
        RAM8(0x57) = 0x00;
        game.set_player_health(0x63);
        game.set_player_magic(0x63);
        sub_CAB6(r);
        sub_CACC(r);
        equipped_item = 0x02;
        sub_C234(r);
        r->a = 0x08;
        sub_E660(r);
        sub_E6B7(r);
        sub_CF30(r);
        sub_CF82(r);
        scroll_x_fine = 0x08;
        sub_C1C7(r);
        sub_C1D8(r);
        sub_C492(r);
        sub_E4AA(r);
        goto L_E5FD;
    }

L_E5FD:
    sub_E5FD(r);
}

} // namespace

extern "C" void sub_E077(Regs* r)
{
    lotw::native::run_frame_task(
        r,
        sub_E077_script(r),
        [] { return std::uint8_t{0}; });
}

#undef map_screen_y
#undef player_x_fine_v
#undef player_y_v
#undef carried_item0
#undef equipped_item
#undef stat_jump
#undef inventory_counts
#undef scroll_x_fine
#undef mmc3_r2_shadow
#undef mmc3_r3_shadow
#undef mmc3_r4_shadow
#undef mmc3_r5_shadow
