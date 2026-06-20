















#include "game_memory.h"
#include "routine_context.h"

#define TRI_LINEAR 0x4008
#define TRI_LO     0x400A
#define TRI_HI     0x400B

void routine_0277(RoutineContext *);
void routine_0287(RoutineContext *);
void inc16_95(RoutineContext *);
void routine_0283(RoutineContext *);

static void fa54(RoutineContext *r)
{
    r->value = 0x00;
    REG_W(TRI_LINEAR, 0x00);
    GAME_MEM8(0x27) = GAME_MEM8(0x27) & 0xFB;
    r->value = GAME_MEM8(0x27);
}

void routine_0275(RoutineContext *r)
{
    if ((GAME_MEM8(0xB4) & 0x80) == 0) {
        fa54(r);
        return;
    }


    if ((u8)(GAME_MEM8(0xB3) - 1) != 0) {
        GAME_MEM8(0xB3) = (u8)(GAME_MEM8(0xB3) - 1);
        return;
    }
    GAME_MEM8(0xB3) = (u8)(GAME_MEM8(0xB3) - 1);


    for (;;) {
        u16 ptr = (u16)(GAME_MEM8(0xB5) | (GAME_MEM8(0xB6) << 8));
        u8 cmd = GAME_MEM8(ptr);
        if (cmd == 0) {
            routine_0287(r);
            fa54(r);
            return;
        }
        if (cmd != 0xFF) {

            u8 saved_n = (u8)(cmd & 0x80);
            r->value = cmd;
            inc16_95(r);
            r->value = (u8)(cmd & 0x7F);
            GAME_MEM8(0xB3) = r->value;
            if (saved_n) {
                fa54(r);
                return;
            }
            routine_0283(r);
            GAME_MEM8(0x27) = GAME_MEM8(0x27) | 0x04;
            REG_W(TRI_LINEAR, GAME_MEM8(0xBA));
            REG_W(TRI_LO, GAME_MEM8(0x04));
            r->value = (u8)((GAME_MEM8(0x05) & 0x07) | 0xF8);
            REG_W(TRI_HI, r->value);
            return;
        }

        routine_0277(r);
    }
}
