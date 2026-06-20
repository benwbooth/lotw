

















#include "game_memory.h"
#include "routine_context.h"

void routine_0102(RoutineContext *r);

void routine_0099(RoutineContext *r)
{
    u8 value, slot, count;

    value = GAME_MEM8(0x0405);
    if (value >= 0x6D) value = 0x6D;
    GAME_MEM8(0x08) = value;
    GAME_MEM8(0x09) = 0x00;

    slot = 0x65;
    count = 0x6B;


    value = slot;
    slot = GAME_MEM8(0x09);
    GAME_MEM8((u16)(0x0259 + slot)) = value;
    GAME_MEM8((u16)(0x025D + slot)) = value;
    GAME_MEM8((u16)(0x0261 + slot)) = value;
    GAME_MEM8((u16)(0x0265 + slot)) = value;
    GAME_MEM8((u16)(0x0269 + slot)) = value;
    value = count;
    GAME_MEM8((u16)(0x026D + slot)) = value;
    GAME_MEM8((u16)(0x0271 + slot)) = value;
    GAME_MEM8((u16)(0x0275 + slot)) = value;
    GAME_MEM8((u16)(0x0279 + slot)) = value;
    GAME_MEM8((u16)(0x027D + slot)) = value;

    routine_0102(r);
    count = r->offset;


    slot = (u8)(GAME_MEM8(0x09) + 0x18);
    for (;;) {
        count = (u8)(count - 1);
        if (count == 0) break;
        GAME_MEM8((u16)(0x0241 + slot))--;
        GAME_MEM8((u16)(0x0241 + slot))--;
        count = (u8)(count - 1);
        if (count == 0) break;
        GAME_MEM8((u16)(0x0241 + slot))--;
        GAME_MEM8((u16)(0x0241 + slot))--;
        slot = (u8)(slot + 4);
    }


    slot = (u8)(GAME_MEM8(0x09) + 0x2C);
    count = GAME_MEM8(0x08);
    for (;;) {
        count = (u8)(count - 1);
        if (count == 0) break;
        GAME_MEM8((u16)(0x0241 + slot))--;
        GAME_MEM8((u16)(0x0241 + slot))--;
        count = (u8)(count - 1);
        if (count == 0) break;
        GAME_MEM8((u16)(0x0241 + slot))--;
        GAME_MEM8((u16)(0x0241 + slot))--;
        slot = (u8)(slot + 4);
    }

    r->value = value;
    r->index = slot;
    r->offset = count;
}
