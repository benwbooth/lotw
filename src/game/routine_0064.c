



#include "game_memory.h"
#include "routine_context.h"

#define sprite_tables 0x0400
#define scroll_x_fine GAME_MEM8(0x007B)
#define scroll_x_tile GAME_MEM8(0x007C)

void routine_0064(RoutineContext *r)
{
    u16 x = r->index;
    u16 y = r->offset;
    u8 a, t;
    u8 carry;

    if (GAME_MEM8(sprite_tables + 1 + y) == 0) goto blank;
    if (GAME_MEM8(sprite_tables + 0x0E + y) >= 0xBF) goto blank;

    a = GAME_MEM8(sprite_tables + 2 + y);
    GAME_MEM8(0x0202 + x) = a;
    GAME_MEM8(0x0206 + x) = a;
    if (a & 0x40) {
        a = GAME_MEM8(sprite_tables + y);
        GAME_MEM8(0x0205 + x) = a;
        a = (u8)(a + 0x02);
        GAME_MEM8(0x0201 + x) = a;
    } else {
        a = GAME_MEM8(sprite_tables + y);
        GAME_MEM8(0x0201 + x) = a;
        a = (u8)(a + 0x02);
        GAME_MEM8(0x0205 + x) = a;
    }


    {
        u16 d = (u16)GAME_MEM8(sprite_tables + 0x0C + y) + 0x100 - scroll_x_fine;
        a = (u8)d & 0x0F;
        GAME_MEM8(0x08) = a;
        carry = (u8)(d >> 8);

        d = (u16)GAME_MEM8(sprite_tables + 0x0D + y) + carry
            - scroll_x_tile - 1;
        a = (u8)d;
        if (a >= 0x10) goto blank;
        a = (u8)((a << 4) | GAME_MEM8(0x08));
        GAME_MEM8(0x08) = a;
    }

    if (GAME_MEM8(sprite_tables + 1 + y) == 0x01) {
        if (GAME_MEM8(sprite_tables + 0x0F + y) != 0) {
            GAME_MEM8(0x08) = (u8)(GAME_MEM8(0x08) + GAME_MEM8(sprite_tables + 0x0F + y));
            GAME_MEM8(sprite_tables + 0x0F + y) = 0x00;
        }
    }


    a = GAME_MEM8(0x08);
    if (a >= 0xEF) {
        GAME_MEM8(0x0203 + x) = a;
        t = (u8)(GAME_MEM8(sprite_tables + 0x0E + y) + 0x2B);
        GAME_MEM8(0x0200 + x) = t;
        GAME_MEM8(0x0204 + x) = 0xEF;
        return;
    }
    GAME_MEM8(0x0203 + x) = a;
    a = (u8)(a + 0x08);
    GAME_MEM8(0x0207 + x) = a;
    t = (u8)(GAME_MEM8(sprite_tables + 0x0E + y) + 0x2B);
    GAME_MEM8(0x0200 + x) = t;
    GAME_MEM8(0x0204 + x) = t;
    return;

blank:
    GAME_MEM8(0x0200 + x) = 0xEF;
    GAME_MEM8(0x0204 + x) = 0xEF;
}
