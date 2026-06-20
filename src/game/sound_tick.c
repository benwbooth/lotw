
















#include "game_memory.h"
#include "routine_context.h"

void sound_set_default_banks(RoutineContext *r);
void sound_set_song_banks(RoutineContext *r);
void sound_restore_game_banks(RoutineContext *r);
void sfx_overlay_voice(RoutineContext *r);
void routine_0273(RoutineContext *r);
void routine_0274(RoutineContext *r);
void routine_0275(RoutineContext *r);
void routine_0276(RoutineContext *r);

#define SQ1_VOL    0x4000
#define SQ2_VOL    0x4004
#define TRI_LINEAR 0x4008
#define NOISE_VOL  0x400C

void sound_tick(RoutineContext *r)
{
    sound_set_default_banks(r);

    GAME_MEM8(0x02) = 0x40;
    r->value = 0x40;
    sfx_overlay_voice(r);

    if (GAME_MEM8(0x8D) != 0) {


        if (!(GAME_MEM8(0xD4) & 0x80)) {
            REG_W(SQ2_VOL, (GAME_MEM8(0xA9) & 0xC0) | 0x30);
        }

        REG_W(SQ1_VOL, (GAME_MEM8(0x99) & 0xC0) | 0x30);
        REG_W(TRI_LINEAR, 0x00);
        REG_W(NOISE_VOL, 0x30);
        r->value = 0x30;
    } else {

        sound_set_song_banks(r);
        GAME_MEM8(0x02) = 0x00; r->value = 0x00; routine_0273(r);
        GAME_MEM8(0x02) = 0x10; r->value = 0x10; routine_0274(r);
        GAME_MEM8(0x02) = 0x20; r->value = 0x20; routine_0275(r);
        GAME_MEM8(0x02) = 0x30; r->value = 0x30; routine_0276(r);
    }


    sound_restore_game_banks(r);
}
