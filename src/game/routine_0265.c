




#include "game_memory.h"
#include "routine_context.h"

void routine_0098(RoutineContext *r);

static void swap(u16 a, u16 b) { u8 t = GAME_MEM8(a); GAME_MEM8(a) = GAME_MEM8(b); GAME_MEM8(b) = t; }

void routine_0265(RoutineContext *r)
{
    GAME_MEM8(0x041F) = GAME_MEM8(0xFC); GAME_MEM8(0x042F) = GAME_MEM8(0xFC); GAME_MEM8(0x043F) = GAME_MEM8(0xFC);
    {
        u8 fb = GAME_MEM8(0xFB);
        GAME_MEM8(0x041E) = fb;
        GAME_MEM8(0x042E) = (u8)(fb + 0x10); GAME_MEM8(0x043E) = (u8)(fb + 0x10);
    }
    GAME_MEM8(0x041C) = GAME_MEM8(0xF9); GAME_MEM8(0x042C) = GAME_MEM8(0xF9); GAME_MEM8(0x043C) = GAME_MEM8(0xF9);
    {
        u8 fa = GAME_MEM8(0xFA);
        GAME_MEM8(0x042D) = fa;
        GAME_MEM8(0x041D) = (u8)(fa + 1); GAME_MEM8(0x043D) = (u8)(fa + 1);
    }
    {
        u8 xv = GAME_MEM8(0xEE);
        if (!(xv & 0x80)) {
            if ((GAME_MEM8(0x0411) | GAME_MEM8(0x0421) | GAME_MEM8(0x0431)) & 0x80)
                xv = 0x80;
        }
        GAME_MEM8(0x0401) = xv; GAME_MEM8(0x0411) = xv; GAME_MEM8(0x0421) = xv; GAME_MEM8(0x0431) = xv;
    }
    {
        u8 a = GAME_MEM8(0xF2);
        if (a >= GAME_MEM8(0x0415)) a = GAME_MEM8(0x0415);
        if (a >= GAME_MEM8(0x0425)) a = GAME_MEM8(0x0425);
        if (a >= GAME_MEM8(0x0435)) a = GAME_MEM8(0x0435);
        GAME_MEM8(0x0405) = a;
    }
    {
        u8 ed = GAME_MEM8(0xED);
        u8 a = (u8)(ed | 0x04); GAME_MEM8(0x0410) = a;
        a = (u8)(a | 0x20); GAME_MEM8(0x0430) = a;
        a = (u8)(a & 0xFB); GAME_MEM8(0x0420) = a;
    }
    {
        u8 ef = GAME_MEM8(0xEF);
        GAME_MEM8(0x0412) = ef; GAME_MEM8(0x0422) = ef; GAME_MEM8(0x0432) = ef;
        if (ef & 0x40) {
            swap(0x0400, 0x0410);
            swap(0x0420, 0x0430);
        }
        if (ef & 0x80) {
            swap(0x0400, 0x0420);
            swap(0x0410, 0x0430);
        }
    }




    {
        u8 old6 = GAME_MEM8(0x30), old7 = GAME_MEM8(0x31);
        GAME_MEM8(0x32) = old6; GAME_MEM8(0x33) = old7;
        GAME_MEM8(0x30) = 0x0C; GAME_MEM8(0x31) = 0x0D; GAME_MEM8(0x25) = 0x07; LOTW_BANK_SYNC();
        GAME_MEM8(0x0E) = 0x53; GAME_MEM8(0x0F) = 0xCB;
        routine_0098(r);
        GAME_MEM8(0x31) = old7; GAME_MEM8(0x30) = old6; GAME_MEM8(0x25) = 0x06; LOTW_BANK_SYNC();
    }
}
