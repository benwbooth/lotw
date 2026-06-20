















#include "game_memory.h"
#include "routine_context.h"

void routine_0277(RoutineContext *r);
void routine_0285(RoutineContext *r);
void routine_0286(RoutineContext *r);
void routine_0287(RoutineContext *r);
void routine_0288(RoutineContext *r);
void routine_0289(RoutineContext *r);
void inc16_95(RoutineContext *r);

static void silence_FB82(RoutineContext *r)
{
    REG_W(0x400C, 0x30);
    GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) & 0xF7);
}

void routine_0276(RoutineContext *r)
{
    if ((GAME_MEM8(0xC4) & 0x80) == 0) {
        silence_FB82(r);
        return;
    }


    if ((u8)(--GAME_MEM8(0xC3)) != 0)
        goto flow_0725;


    for (;;) {
        u16 ptr = (u16)(GAME_MEM8(0xC5) | (GAME_MEM8(0xC6) << 8));
        u8 note = GAME_MEM8(ptr);
        if (note == 0) {
            routine_0287(r);
            silence_FB82(r);
            return;
        }
        if (note == 0xFF) {
            routine_0277(r);
            continue;
        }

        inc16_95(r);
        GAME_MEM8(0xC3) = (u8)(note & 0x7F);
        if (note & 0x80) {
            routine_0286(r);
        } else {
            GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) | 0x08);
            REG_W(0x400E, GAME_MEM8(0xCA));
            REG_W(0x400F, 0x80);
            routine_0285(r);
        }
        break;
    }

flow_0725:
    if ((GAME_MEM8(0x27) & 0x08) == 0)
        return;

    if ((u8)(--GAME_MEM8(0xCD)) == 0) {
        routine_0288(r);
        REG_W(0x400C, r->value);
    }

    routine_0289(r);
    if (r->carry)
        silence_FB82(r);
}
