





#include "game_memory.h"
#include "routine_context.h"

void routine_0213(RoutineContext *r); void routine_0214(RoutineContext *r); void routine_0215(RoutineContext *r);
void routine_0216(RoutineContext *r); void routine_0219(RoutineContext *r); void routine_0218(RoutineContext *r);
void routine_0240(RoutineContext *r); void routine_0259(RoutineContext *r); void routine_0263(RoutineContext *r);
void routine_0264(RoutineContext *r); void routine_0258(RoutineContext *r); void routine_0257(RoutineContext *r);
void routine_0265(RoutineContext *r); void routine_0217(RoutineContext *r);

void routine_0212(RoutineContext *r)
{
    if (GAME_MEM8(0x48) == 0x10)
        return;
    if (GAME_MEM8(0x2D) >= 0x30)
        goto flow_0469;


    {
        u8 e9 = GAME_MEM8(0xE9);
        u8 v = (u8)((e9 << 1) + e9);
        GAME_MEM8(0xE3) = v;
        GAME_MEM8(0xE4) = (u8)(v + 3);
        u8 e5 = (u8)(GAME_MEM8(0xE3) << 4);
        GAME_MEM8(0xE5) = e5;
        GAME_MEM8(0xE7) = (u8)(e5 + 0x20);
        GAME_MEM8(0xE6) = 0x04;
        GAME_MEM8(0xE8) = GAME_MEM8(0x78);
    }
    do {
        u8 ee;
        routine_0213(r);
        ee = GAME_MEM8(0xEE);
        if (ee == 0)         routine_0215(r);
        else if (ee & 0x80)  routine_0240(r);
        else if (ee == 0x01) routine_0218(r);
        else if (ee >= 0x18) routine_0216(r);
        else                 routine_0219(r);
        routine_0214(r);
        GAME_MEM8(0xE3)++;
        GAME_MEM8(0xE5) = (u8)(GAME_MEM8(0xE5) + 0x10);
        GAME_MEM8(0xE7) = (u8)(GAME_MEM8(0xE7) + 0x10);
    } while (GAME_MEM8(0xE3) < GAME_MEM8(0xE4));
    {
        u8 e9 = (u8)(GAME_MEM8(0xE9) + 1);
        GAME_MEM8(0xE9) = (e9 >= 0x03) ? 0x00 : e9;
    }
    return;

flow_0469:
    if (GAME_MEM8(0xE9) & 0x01)
        goto flow_0472;

    GAME_MEM8(0xE5) = 0x00; GAME_MEM8(0xE6) = 0x04; GAME_MEM8(0xE3) = 0x00;
    GAME_MEM8(0xE7) = 0x20; GAME_MEM8(0xE8) = GAME_MEM8(0x78);
    routine_0213(r);
    {
        u8 ee = GAME_MEM8(0xEE);
        if (ee == 0)             routine_0257(r);
        else if (ee & 0x80)    { routine_0259(r); routine_0263(r); routine_0264(r); }
        else                     routine_0258(r);
    }
    routine_0214(r);
    routine_0265(r);
    goto flow_0475;

flow_0472:
    GAME_MEM8(0xE3) = 0x04; GAME_MEM8(0xE5) = 0x40; GAME_MEM8(0xE6) = 0x04;
    GAME_MEM8(0xE7) = 0x60; GAME_MEM8(0xE8) = GAME_MEM8(0x78);
    do {
        u8 ee;
        routine_0213(r);
        ee = GAME_MEM8(0xEE);
        if (ee == 0 || (ee & 0x80)) {
            GAME_MEM8(0xEE) = 0x00;
            routine_0217(r);
        } else {
            routine_0218(r);
        }
        routine_0214(r);
        GAME_MEM8(0xE3)++;
        GAME_MEM8(0xE5) = (u8)(GAME_MEM8(0xE5) + 0x10);
        GAME_MEM8(0xE7) = (u8)(GAME_MEM8(0xE7) + 0x10);
    } while (GAME_MEM8(0xE3) < 0x09);

flow_0475:
    GAME_MEM8(0xE9) ^= 0x01;
}
