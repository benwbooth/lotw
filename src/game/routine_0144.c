


#include "game_memory.h"
#include "routine_context.h"

void routine_0144(RoutineContext *r)
{
    u8 x, y, a;

    x = 0x3D;
    if (GAME_MEM8(0x46) != 0) { GAME_MEM8(0x56) = x; return; }
    x = 0x09;
    if (GAME_MEM8(0x50) != 0) { GAME_MEM8(0x56) = x; return; }
    if ((GAME_MEM8(0x20) & 0xBF) == 0x80) { GAME_MEM8(0x56) = x; return; }

    a = GAME_MEM8(0x4B);
    if (a == 0) goto D913;
    if (a & 0x80) goto D90C;


    if (GAME_MEM8(0x4E) != 0) goto D931;
    if ((GAME_MEM8(0x20) & 0x04) == 0) goto D913;
    x = 0x0D;
    GAME_MEM8(0x56) = x;
    return;

D90C:
    if (GAME_MEM8(0x4F) == 0) { GAME_MEM8(0x56) = x; return; }
    goto D931;

D913:
    x = 0x01;
    y = 0x00;
    if (GAME_MEM8(0x4A) & 0x80) goto D921;
    if (GAME_MEM8(0x49) == 0) return;
    y = 0x40;
D921:
    GAME_MEM8(0x08) = x;
    GAME_MEM8(0x56) = (GAME_MEM8(0x56) & 0x07) | GAME_MEM8(0x08);
    GAME_MEM8(0x57) = y;
    return;

D931:
    x = 0x39;
    y = 0x00;
    a = GAME_MEM8(0x4A) | GAME_MEM8(0x49);
    if (a & 0x80) goto D941;
    if (a != 0) goto D93F;
    x = 0x09;
D93F:
    y = 0x40;
D941:
    GAME_MEM8(0x08) = x;
    GAME_MEM8(0x56) = (GAME_MEM8(0x56) & 0x03) | GAME_MEM8(0x08);
    GAME_MEM8(0x57) = y;
}
