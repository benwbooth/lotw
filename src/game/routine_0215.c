
















#include "game_memory.h"
#include "routine_context.h"

void rng_update(RoutineContext *r);
void routine_0111(RoutineContext *r);
void routine_0253(RoutineContext *r);

void routine_0215(RoutineContext *r)
{
    u8 x, t;
    u16 e7;

    GAME_MEM8(0xF3) = (u8)(GAME_MEM8(0xF3) - 1);
    x = GAME_MEM8(0xF3);
    if (x >= 0x3C) { return; }

    e7 = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));


    if ((GAME_MEM8((u16)(e7 + 2)) | GAME_MEM8((u16)(e7 + 3))) == 0) {
        r->value = 0x0C;
        rng_update(r);
        GAME_MEM8(0x0A) = (u8)(r->value << 4);
        r->value = 0x40;
        rng_update(r);
        GAME_MEM8(0x0F) = r->value;
    } else {

        GAME_MEM8(0x0A) = GAME_MEM8((u16)(e7 + 3));
        GAME_MEM8(0x0F) = GAME_MEM8((u16)(e7 + 2));
    }


    GAME_MEM8(0x0E) = 0x00;
    GAME_MEM8(0x0B) = 0x00;

    routine_0111(r);
    if (r->carry) { return; }

    routine_0253(r);
    if (r->carry) { return; }


    GAME_MEM8(0xF9) = GAME_MEM8(0x0E);
    GAME_MEM8(0xFA) = GAME_MEM8(0x0F);
    GAME_MEM8(0xFB) = GAME_MEM8(0x0A);
    GAME_MEM8(0xF1) = 0x00;
    GAME_MEM8(0xF0) = 0x00;
    GAME_MEM8(0xF4) = 0x00;
    GAME_MEM8(0xFC) = 0x00;
    GAME_MEM8(0xF2) = GAME_MEM8((u16)(e7 + 4));
    GAME_MEM8(0xF8) = GAME_MEM8((u16)(e7 + 5));


    {
        u8 a = 0x00;
        u8 c = 1;
        u8 xi = GAME_MEM8(0x40);
        do {
            u8 nc = (u8)((a >> 7) & 1);
            a = (u8)((a << 1) | c);
            c = nc;
            xi = (u8)(xi - 1);
        } while ((xi & 0x80) == 0);
        a = (u8)(a & GAME_MEM8(0x41));
        if (a == 0) {

            u8 f8 = GAME_MEM8(0xF8);
            u8 carry = (u8)((f8 >> 7) & 1);
            GAME_MEM8(0xF8) = (u8)(f8 << 1);
            if (carry)
                GAME_MEM8(0xF8) = 0xFF;
        }
    }


    GAME_MEM8(0xEE) = 0x7F;
    GAME_MEM8(0xED) = 0xF9;
    GAME_MEM8(0xEF) = 0x01;


    t = GAME_MEM8(0xF3);
    if (t == 0) {
        GAME_MEM8(0xEE) = 0x01;
        GAME_MEM8(0xED) = GAME_MEM8((u16)(e7 + 0));
        GAME_MEM8(0xEF) = GAME_MEM8((u16)(e7 + 1));
    } else {

        if ((GAME_MEM8(0xF3) & 0x03) == 0) {
            GAME_MEM8(0xEF) = (u8)(GAME_MEM8(0xEF) ^ 0x40);
        }
    }
}
