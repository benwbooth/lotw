




















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

static void silence_F95E(RoutineContext *r)
{
    REG_W(0x4000, (GAME_MEM8(0x99) & 0xC0) | 0x30);
    GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) & 0xFE);
}

void routine_0273(RoutineContext *r)
{
    if ((GAME_MEM8(0x94) & 0x80) == 0) {
        silence_F95E(r);
        return;
    }


    if ((u8)(--GAME_MEM8(0x93)) != 0)
        goto flow_0685;


    for (;;) {
        u16 ptr = (u16)(GAME_MEM8(0x95) | (GAME_MEM8(0x96) << 8));
        u8 note = GAME_MEM8(ptr);
        if (note == 0) {
            routine_0287(r);
            silence_F95E(r);
            return;
        }
        if (note == 0xFF) {
            routine_0277(r);
            continue;
        }

        inc16_95(r);
        GAME_MEM8(0x93) = (u8)(note & 0x7F);
        if (note & 0x80) {
            routine_0286(r);
        } else {
            routine_0283(r);
            GAME_MEM8(0x27) = (u8)(GAME_MEM8(0x27) | 0x01);
            REG_W(0x4001, GAME_MEM8(0x9A));
            REG_W(0x4002, GAME_MEM8(0x04));
            REG_W(0x4003, (GAME_MEM8(0x05) & 0x07) | 0x18);
            routine_0285(r);
        }
        break;
    }

flow_0685:
    if ((GAME_MEM8(0x27) & 0x01) == 0)
        return;

    if ((u8)(--GAME_MEM8(0x9D)) == 0) {
        routine_0288(r);
        REG_W(0x4000, r->value);
    }

    routine_0289(r);
    if (r->carry)
        silence_F95E(r);
}
