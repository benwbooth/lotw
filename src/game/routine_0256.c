
























#include "game_memory.h"
#include "routine_context.h"

void routine_0247(RoutineContext *r);

void routine_0256(RoutineContext *r)
{
    u8 v;
    GAME_MEM8(0xF6) = 0x00;

    if (GAME_MEM8(0xF5) != 0) {
        GAME_MEM8(0xF5) = 0x00;
        v = GAME_MEM8(0xFB) & 0x0F;
        if (v == 0) goto sec_ret;
        if (v < 0x06) {
            if (GAME_MEM8(0xF4) & 0x04) goto sec_ret;
            GAME_MEM8(0xF7) = 0xFF;
            goto call_f0e1;
        }
        if (v >= 0x0B) {
            if (GAME_MEM8(0xF4) & 0x08) goto sec_ret;
            GAME_MEM8(0xF7) = 0x01;
            goto call_f0e1;
        }
        goto sec_ret;
    }


    if (GAME_MEM8(0xF7) == 0) goto sec_ret;
    GAME_MEM8(0xF7) = 0x00;
    v = GAME_MEM8(0xF9);
    if (v == 0) goto sec_ret;
    if (v < 0x06) {
        if (GAME_MEM8(0xF4) & 0x01) goto sec_ret;
        GAME_MEM8(0xF5) = 0x0F;
        GAME_MEM8(0xF6) = 0xFF;
        goto call_f0e1;
    }
    if (v >= 0x0B) {
        if (GAME_MEM8(0xF4) & 0x02) goto sec_ret;
        GAME_MEM8(0xF5) = 0x01;
        GAME_MEM8(0xF6) = 0x00;
        goto call_f0e1;
    }
    goto sec_ret;

call_f0e1:
    routine_0247(r);
    return;

sec_ret:
    r->carry = 1;
}
