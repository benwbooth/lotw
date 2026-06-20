
























#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0108(RoutineContext *r);
void routine_0231(RoutineContext *r);
void routine_0236(RoutineContext *r);
void routine_0237(RoutineContext *r);
void routine_0238(RoutineContext *r);
void routine_0239(RoutineContext *r);
void routine_0242(RoutineContext *r);
void routine_0247(RoutineContext *r);
void routine_0250(RoutineContext *r);

void routine_0223(RoutineContext *r)
{
    GAME_MEM8(0xF4) = GAME_MEM8(0xF4) & 0x0F;

    if ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) != 0) {

        if (GAME_MEM8(0xF3) < 0x10)
            goto flow_0514;
        goto flow_0513;
    }

    if (GAME_MEM8(0xF9) == 0) {
        u16 ptr;
        GAME_MEM8(0x0C) = GAME_MEM8(0xFA);
        GAME_MEM8(0x0D) = GAME_MEM8(0xFB);
        routine_0090(r);
        ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
        if ((GAME_MEM8(ptr) & 0x3F) == 0)
            goto flow_0513;
        if ((GAME_MEM8((u16)(ptr + 1)) & 0x3F) == 0)
            goto flow_0513;
    }


    if ((GAME_MEM8(0xF4) & 0x03) == 0)
        GAME_MEM8(0xF4) = 0x01;


    {
        u8 x = (u8)(GAME_MEM8(0xF3) - 1);
        GAME_MEM8(0xF3) = 0x00;
        if (x == 0) {
            if ((GAME_MEM8(0xF4) & 0x03) == 0)
                goto flow_0513;
            GAME_MEM8(0xF4) = (u8)(GAME_MEM8(0xF4) ^ 0x03);
            goto flow_0514;
        }
    }


    routine_0231(r);
    GAME_MEM8(0xF4) = (u8)(0x80 | GAME_MEM8(0xF4));
    goto flow_0514;

flow_0513:
    GAME_MEM8(0xF3) = 0x00;
    routine_0231(r);

flow_0514:
    {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        r->offset = GAME_MEM8((u16)(ptr + 0x09));
    }
    r->value = GAME_MEM8(0xF4);
    routine_0108(r);

    if (GAME_MEM8(0xF0) != 0) {

        routine_0236(r);
        if (r->carry) goto flow_0519;
        goto flow_0518;
    }

    if (GAME_MEM8(0xF1) != 0)
        goto flow_0515;
    if (!(GAME_MEM8(0xF4) & 0x80))
        goto flow_0516;

flow_0515:
    routine_0237(r);
    if (!r->carry) goto flow_0518;

flow_0516:
    GAME_MEM8(0xF1) = 0x00;
    routine_0247(r);
    if (!r->carry) goto flow_0518;
    routine_0239(r);
    goto flow_0519;

flow_0518:
    routine_0238(r);

flow_0519:
    routine_0250(r);
    routine_0242(r);
}
