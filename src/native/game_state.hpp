#ifndef LOTW_NATIVE_GAME_STATE_HPP
#define LOTW_NATIVE_GAME_STATE_HPP

#include <cstdint>

namespace lotw::native {

class GameState {
public:
    std::uint8_t buttons() const;
    void set_buttons(std::uint8_t buttons);
    std::uint8_t button_chord() const;

    std::uint8_t frame_counter() const;
    void set_frame_counter(std::uint8_t frames);
    bool frame_counter_active() const;

    bool ppu_job_pending() const;
    void request_ppu_job(std::uint8_t job);

    void set_prompt_state(std::uint8_t state);
    void set_prompt_argument(std::uint8_t argument);

    void set_countdown_timer(std::uint8_t frames);
    bool countdown_timer_active() const;

    bool frame_status_bit6_set() const;

    void push_dialog_depth();
    void pop_dialog_depth();

    std::uint8_t sprite_blink_timer() const;
    void set_sprite_blink_timer(std::uint8_t frames);

    std::uint8_t player_health() const;
    void set_player_health(std::uint8_t value);

    std::uint8_t player_magic() const;
    void set_player_magic(std::uint8_t value);
};

}

#endif
