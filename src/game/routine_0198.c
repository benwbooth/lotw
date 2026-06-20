



#include "game_memory.h"
#include "routine_context.h"

void routine_0198(RoutineContext *r)
{
    u8 x, a;


    a = 0xEF;
    x = GAME_MEM8(0x80);
    if (x & 0x80) goto G1;
    if (GAME_MEM8((u16)(0x0060 + x)) >= 0x0B) {
        GAME_MEM8(0x80) = 0xEF;
        a = 0xEF;
        goto G1;
    }
    a = (u8)(x << 2);
    a = (u8)(a + 0xA1);   GAME_MEM8(0x0241) = a;
    a = (u8)(a + 0x02);   GAME_MEM8(0x0245) = a;
    GAME_MEM8(0x0243) = 0x40;
    GAME_MEM8(0x0247) = 0x48;
    a = 0xA4;
G1:
    GAME_MEM8(0x0240) = a;
    GAME_MEM8(0x0244) = a;
    GAME_MEM8(0x0242) = 0x01;
    GAME_MEM8(0x0246) = 0x01;


    a = 0xEF;
    x = GAME_MEM8(0x82);
    if (x & 0x80) goto G2;
    if (GAME_MEM8((u16)(0x0060 + x)) >= 0x0B) {
        GAME_MEM8(0x82) = 0xEF;
        a = 0xEF;
        goto G2;
    }
    a = (u8)(x << 2);
    a = (u8)(a + 0xA1);   GAME_MEM8(0x0249) = a;
    a = (u8)(a + 0x02);   GAME_MEM8(0x024D) = a;
    GAME_MEM8(0x024B) = 0xB0;
    GAME_MEM8(0x024F) = 0xB8;
    a = 0xA0;
G2:
    GAME_MEM8(0x0248) = a;
    GAME_MEM8(0x024C) = a;
    GAME_MEM8(0x024A) = 0x01;
    GAME_MEM8(0x024E) = 0x01;
}
