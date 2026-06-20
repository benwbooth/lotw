






















#include "game_memory.h"
#include "routine_context.h"

void routine_0277(RoutineContext *r);
void routine_0283(RoutineContext *r);
void routine_0285(RoutineContext *r);
void routine_0286(RoutineContext *r);
void routine_0287(RoutineContext *r);
void routine_0288(RoutineContext *r);
void routine_0289(RoutineContext *r);
void inc16_95(RoutineContext *r);

static void silence_F9F9(RoutineContext *r)
{
    REG_W(0x4004, (GAME_MEM8(0xA9) & 0xC0) | 0x30);
    GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) & 0xFD);
}

void routine_0274(RoutineContext *r)
{
    u8 a4 = GAME_MEM8(0xA4);
    if ((a4 & 0x80) == 0) {
        if (a4 & 0x40)
            return;
        silence_F9F9(r);
        return;
    }


    if ((u8)(--GAME_MEM8(0xA3)) != 0)
        goto flow_0698;


    for (;;) {
        u16 ptr = (u16)(GAME_MEM8(0xA5) | (GAME_MEM8(0xA6) << 8));
        u8 note = GAME_MEM8(ptr);
        if (note == 0) {
            routine_0287(r);
            silence_F9F9(r);
            return;
        }
        if (note == 0xFF) {
            routine_0277(r);
            continue;
        }

        inc16_95(r);
        GAME_MEM8(0xA3) = (u8)(note & 0x7F);
        if (note & 0x80) {

            if (GAME_MEM8(0xA4) & 0x40)
                return;
            routine_0286(r);
            goto flow_0698;
        }

        if (GAME_MEM8(0xA4) & 0x40) {
            inc16_95(r);
            return;
        }

        routine_0283(r);
        GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) | 0x02);
        REG_W(0x4004, GAME_MEM8(0xA9));
        REG_W(0x4005, GAME_MEM8(0xAA));
        REG_W(0x4006, GAME_MEM8(0x04));
        REG_W(0x4007, (GAME_MEM8(0x05) & 0x07) | 0x18);
        routine_0285(r);
        break;
    }

flow_0698:
    if (GAME_MEM8(0xA4) & 0x40)
        return;
    if ((GAME_MEM8(0x27) & 0x02) == 0)
        return;

    if ((u8)(--GAME_MEM8(0xAD)) == 0) {
        routine_0288(r);
        REG_W(0x4004, r->value);
    }

    routine_0289(r);
    if (r->carry)
        silence_F9F9(r);
}
