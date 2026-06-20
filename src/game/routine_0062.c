



#include "game_memory.h"
#include "routine_context.h"

#define equipped_item GAME_MEM8(0x0055)
#define carried_item0 0x0051

void routine_0062(RoutineContext *r)
{
    u8 value, slot, offset;

    value = equipped_item;
    slot = 0x13;
    if (value >= 0x03) {
        slot = 0xEF;
        GAME_MEM8(0x0238) = slot;
        GAME_MEM8(0x023C) = slot;

    } else {
        GAME_MEM8(0x0238) = slot;
        GAME_MEM8(0x023C) = slot;
        value = (u8)(value << 4);
        value = (u8)(value + 0xC8);  GAME_MEM8(0x023B) = value;
        value = (u8)(value + 0x08);  GAME_MEM8(0x023F) = value;
        GAME_MEM8(0x0239) = 0xFF;
        GAME_MEM8(0x023D) = 0xFF;
        GAME_MEM8(0x023A) = 0x01;
        GAME_MEM8(0x023E) = 0x41;
    }


    slot = 0x02;
    offset = 0x10;
    for (;;) {
        value = GAME_MEM8((u16)(carried_item0 + slot));
        if (value & 0x80) {
            value = 0xEF;
        } else {
            value = (u8)(value << 2);
            value = (u8)(value + 0xA1);  GAME_MEM8((u16)(0x0221 + offset)) = value;
            value = (u8)(value + 0x02);  GAME_MEM8((u16)(0x0225 + offset)) = value;
            value = (u8)(offset << 1);
            value = (u8)(value + 0xC8);  GAME_MEM8((u16)(0x0223 + offset)) = value;
            value = (u8)(value + 0x08);  GAME_MEM8((u16)(0x0227 + offset)) = value;
            GAME_MEM8((u16)(0x0222 + offset)) = 0x01;
            GAME_MEM8((u16)(0x0226 + offset)) = 0x01;
            value = 0x13;
        }

        GAME_MEM8((u16)(0x0220 + offset)) = value;
        GAME_MEM8((u16)(0x0224 + offset)) = value;
        offset = (u8)(offset - 0x08);
        if (slot-- == 0) break;
    }
}
