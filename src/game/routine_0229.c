










#include "game_memory.h"
#include "routine_context.h"

void routine_0232(RoutineContext *);
void routine_0108(RoutineContext *);
void routine_0248(RoutineContext *);
void routine_0238(RoutineContext *);
void routine_0242(RoutineContext *);

void routine_0229(RoutineContext *r)
{
    u8 dec = (u8)(GAME_MEM8(0xF1) - 1);
    GAME_MEM8(0xF1) = dec;
    if (dec == 0)
        goto edeb;

    if (GAME_MEM8(0xF4) == 0) {
        routine_0232(r);

    } else {

        if (GAME_MEM8(0xF3) >= 0x08) {
            u8 diff, bit_count, changed_bits;
            GAME_MEM8(0x08) = GAME_MEM8(0xF4);
            routine_0232(r);
            diff = (u8)(GAME_MEM8(0xF4) ^ GAME_MEM8(0x08));
            changed_bits = 0x00;
            bit_count = 0x04;
            do {
                u8 bit = diff & 1;
                diff >>= 1;
                if (bit) changed_bits++;
            } while (--bit_count != 0);
            changed_bits--;
            if (changed_bits != 0)
                GAME_MEM8(0xF4) = GAME_MEM8(0x08);
        }
    }


    {
        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        r->offset = GAME_MEM8((u16)(ptr + 0x09));
        r->value = GAME_MEM8(0xF4);
        routine_0108(r);
    }
    routine_0248(r);
    if (r->carry)
        goto edeb;


    routine_0238(r);
    routine_0242(r);
    return;

edeb:
    r->value = 0x00;
    GAME_MEM8(0xEE) = 0x00;
}
