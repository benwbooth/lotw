















#include "game_memory.h"
#include "routine_context.h"

void sound_set_song_banks(RoutineContext *r);
void ppu_commit_banks(RoutineContext *r);

void song_init(RoutineContext *r)
{
    u8 song = GAME_MEM8(0x8E);
    u8 idx, x;
    int blk;

    x = (song < 0x0A) ? 0x0A : 0x0C;
    GAME_MEM8(0x34) = x;
    GAME_MEM8(0x35) = (u8)(x + 1);

    sound_set_song_banks(r);

    GAME_MEM8(0x92) = 0x00;
    GAME_MEM8(0x8F) = 0x00;

    idx = (song < 0x0A) ? song : (u8)(song - 0x0A);
    idx = (u8)(idx << 1);





    {
#ifdef LOTW_SHIM


        GAME_MEM8(0x0E) = GAME_MEM8((u16)(0x8000 + idx));
        GAME_MEM8(0x0F) = GAME_MEM8((u16)(0x8001 + idx));
#else
        u8 lo = (idx == 0) ? 0x07 : (idx == 1 ? GAME_MEM8(0x35) : 0x00);
        u8 hi = (idx + 1 == 0) ? 0x07 : ((idx + 1 == 1) ? GAME_MEM8(0x35) : 0x00);
        GAME_MEM8(0x0E) = lo;
        GAME_MEM8(0x0F) = hi;
#endif
    }
    GAME_MEM8(0x0C) = 0x93;
    GAME_MEM8(0x0D) = 0x00;

    for (blk = 0; blk < 4; blk++) {
        int y;
        u16 s = (u16)(GAME_MEM8(0x0E) | (GAME_MEM8(0x0F) << 8));
        u16 d = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
        for (y = 7; y >= 0; y--)
            GAME_MEM8((u16)(d + y)) = GAME_MEM8((u16)(s + y));

        d = (u16)(GAME_MEM8(0x0C) + 8);
        GAME_MEM8(0x0C) = (u8)d;
        GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0D) + (d >> 8));

        d = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
        for (y = 7; y >= 0; y--)
            GAME_MEM8((u16)(d + y)) = 0x00;

        d = (u16)(GAME_MEM8(0x0C) + 8);
        GAME_MEM8(0x0C) = (u8)d;
        GAME_MEM8(0x0D) = (u8)(GAME_MEM8(0x0D) + (d >> 8));

        s = (u16)(GAME_MEM8(0x0E) + 8);
        GAME_MEM8(0x0E) = (u8)s;
        GAME_MEM8(0x0F) = (u8)(GAME_MEM8(0x0F) + (s >> 8));
    }

    ppu_commit_banks(r);
}
