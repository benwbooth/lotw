







#include "game_memory.h"
#include "routine_context.h"

void routine_0178(RoutineContext *r); void routine_0186(RoutineContext *r); void routine_0184(RoutineContext *r);

void routine_0179(RoutineContext *r)
{
    u8 f5 = GAME_MEM8(0xF5);
    u8 a = (u8)((u8)(f5 << 2) + f5);
    a = (u8)(a + GAME_MEM8(0xF7));

    if (a == 0x20) goto flow_0427;
    if (a == 0x21) goto flow_0428;
    if (a == 0x22) {
        routine_0178(r);
        return;
    }


    r->value = a;
    routine_0186(r);

    GAME_MEM8((u16)(0x0322 + r->index)) = a;
    if (r->index == 0x1F) {
        routine_0178(r);
        return;
    }


flow_0427:
    GAME_MEM8(0xF9)++;
    routine_0184(r);
    return;
flow_0428:
    GAME_MEM8(0xF9)--;
    routine_0184(r);
}
