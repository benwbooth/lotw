#include "native/game_state.hpp"

#include "game_memory.h"

namespace lotw::native {

std::uint8_t GameState::buttons() const
{
    return GAME_MEM8(0x20);
}

void GameState::set_buttons(std::uint8_t buttons)
{
    GAME_MEM8(0x20) = buttons;
}

std::uint8_t GameState::button_chord() const
{
    return GAME_MEM8(0x21);
}

std::uint8_t GameState::frame_counter() const
{
    return GAME_MEM8(0x36);
}

void GameState::set_frame_counter(std::uint8_t frames)
{
    GAME_MEM8(0x36) = frames;
}

bool GameState::frame_counter_active() const
{
    return frame_counter() != 0;
}

bool GameState::ppu_job_pending() const
{
    return GAME_MEM8(0x28) != 0;
}

void GameState::request_ppu_job(std::uint8_t job)
{
    GAME_MEM8(0x28) = job;
}

void GameState::set_prompt_state(std::uint8_t state)
{
    GAME_MEM8(0x8F) = state;
}

void GameState::set_prompt_argument(std::uint8_t argument)
{
    GAME_MEM8(0x90) = argument;
}

void GameState::set_countdown_timer(std::uint8_t frames)
{
    GAME_MEM8(0x8C) = frames;
}

bool GameState::countdown_timer_active() const
{
    return GAME_MEM8(0x8C) != 0;
}

bool GameState::frame_status_bit6_set() const
{
    return (GAME_MEM8(0x26) & 0x40) != 0;
}

void GameState::push_dialog_depth()
{
    GAME_MEM8(0x8D)++;
}

void GameState::pop_dialog_depth()
{
    GAME_MEM8(0x8D)--;
}

std::uint8_t GameState::sprite_blink_timer() const
{
    return GAME_MEM8(0x85);
}

void GameState::set_sprite_blink_timer(std::uint8_t frames)
{
    GAME_MEM8(0x85) = frames;
}

std::uint8_t GameState::player_health() const
{
    return GAME_MEM8(0x58);
}

void GameState::set_player_health(std::uint8_t value)
{
    GAME_MEM8(0x58) = value;
}

std::uint8_t GameState::player_magic() const
{
    return GAME_MEM8(0x59);
}

void GameState::set_player_magic(std::uint8_t value)
{
    GAME_MEM8(0x59) = value;
}

}
