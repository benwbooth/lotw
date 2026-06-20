



#include "game_memory.h"
#include "routine_context.h"
#include <stdio.h>
#include <string.h>

u8 LOTW_MEMORY[0x10000];

void routine_0146(RoutineContext *r);

static int expect_u8(const char *name, u16 addr, u8 want)
{
    u8 got = GAME_MEM8(addr);
    if (got == want)
        return 0;
    fprintf(stderr, "%s $%04X: got %02X, expected %02X\n", name, addr, got, want);
    return 1;
}

int main(void)
{
    RoutineContext r;
    memset(&r, 0, sizeof r);
    memset(LOTW_MEMORY, 0, sizeof LOTW_MEMORY);

    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x44) = 0x10;
    GAME_MEM8(0x45) = 0x50;
    GAME_MEM8(0x49) = 0x00;
    GAME_MEM8(0x4B) = 0x00;
    GAME_MEM8(0xE3) = 0xFF;



    GAME_MEM8(0x0400 + 0x80) = 0x02;
    GAME_MEM8(0x0401 + 0x80) = 0x0A;
    GAME_MEM8(0x0402 + 0x80) = 0x00;
    GAME_MEM8(0x040C + 0x80) = 0x00;
    GAME_MEM8(0x040D + 0x80) = GAME_MEM8(0x44);
    GAME_MEM8(0x040E + 0x80) = GAME_MEM8(0x45);

    r.value = 0x00;
    routine_0146(&r);

    int errors = 0;
    errors |= expect_u8("inventory count", 0x0060, 0x01);
    errors |= expect_u8("object active", 0x0401 + 0x80, 0x00);
    errors |= expect_u8("object y clear", 0x0406 + 0x80, 0xF0);
    errors |= expect_u8("oam clear 0", 0x0200 + ((8 << 3) | 0x80), 0xEF);
    errors |= expect_u8("oam clear 1", 0x0204 + ((8 << 3) | 0x80), 0xEF);
    return errors ? 1 : 0;
}
