



























#include "game_memory.h"
#include "routine_context.h"

void routine_0277(RoutineContext *r);
void routine_0283(RoutineContext *r);
void routine_0285(RoutineContext *r);
void routine_0286(RoutineContext *r);
void routine_0288(RoutineContext *r);
void routine_0289(RoutineContext *r);
void inc16_95(RoutineContext *r);

static void silence_FB0F(RoutineContext *r)
{
    REG_W(0x4004, (GAME_MEM8(0xD9) & 0xC0) | 0x30);
    GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) & 0xFD);
}

void sfx_overlay_voice(RoutineContext *r)
{
    int start = 0;

    if (GAME_MEM8(0x8F) != 0) {
        if ((GAME_MEM8(0xD4) & 0x80) == 0) {
            start = 1;
        } else if (GAME_MEM8(0x90) >= GAME_MEM8(0x91)) {
            start = 1;
        } else {
            GAME_MEM8(0x90) = 0x00;
            GAME_MEM8(0x8F) = 0x00;
        }
    }

    if (!start) {

        if ((GAME_MEM8(0xD4) & 0x80) == 0)
            return;

        if ((u8)(--GAME_MEM8(0xD3)) != 0)
            goto flow_0716;
    } else {

        u8 x;
        GAME_MEM8(0x91) = GAME_MEM8(0x90);
        x = (u8)(GAME_MEM8(0x8F) << 1);
        GAME_MEM8(0xD5) = GAME_MEM8((u16)(0x8014 + x));
        GAME_MEM8(0xD6) = GAME_MEM8((u16)(0x8015 + x));
        GAME_MEM8(0xD4) = 0x80;
        GAME_MEM8(0xA4) = (u8)(GAME_MEM8(0xA4) | 0x40);
        GAME_MEM8(0x8F) = 0x00;
        GAME_MEM8(0x90) = 0x00;

    }


    for (;;) {
        u16 ptr = (u16)(GAME_MEM8(0xD5) | (GAME_MEM8(0xD6) << 8));
        u8 note = GAME_MEM8(ptr);
        if (note == 0) {
            GAME_MEM8(0xD4) = 0x00;
            GAME_MEM8(0x91) = 0x00;
            GAME_MEM8(0xA4) = (u8)(GAME_MEM8(0xA4) & 0xBF);
            silence_FB0F(r);
            return;
        }
        if (note == 0xFF) {
            routine_0277(r);
            continue;
        }

        inc16_95(r);
        GAME_MEM8(0xD3) = (u8)(note & 0x7F);
        if (note & 0x80) {
            routine_0286(r);
        } else {
            routine_0283(r);
            GAME_MEM8(0x27) = (u8)(0x02 | GAME_MEM8(0x27));
            REG_W(0x4005, GAME_MEM8(0xDA));
            REG_W(0x4006, GAME_MEM8(0x04));
            REG_W(0x4007, (GAME_MEM8(0x05) & 0x07) | 0xC0);
            routine_0285(r);
        }
        break;
    }

flow_0716:
    if ((GAME_MEM8(0x27) & 0x02) == 0)
        return;

    if ((u8)(--GAME_MEM8(0xDD)) == 0) {
        routine_0288(r);
        REG_W(0x4004, r->value);
    }

    routine_0289(r);
    if (r->carry)
        silence_FB0F(r);
}
