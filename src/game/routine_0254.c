













#include "game_memory.h"
#include "routine_context.h"

void routine_0090(RoutineContext *r);
void routine_0255(RoutineContext *r);

static int probe(RoutineContext *r, u8 y)
{
    r->offset = y;
    routine_0255(r);
    return r->carry;
}

void routine_0254(RoutineContext *r)
{
    GAME_MEM8(0x0C) = GAME_MEM8(0x0F);
    GAME_MEM8(0x0D) = GAME_MEM8(0x0A);
    routine_0090(r);

    if (probe(r, 0x00)) return;
    if (probe(r, 0x01)) return;
    if (probe(r, 0x0C)) return;
    if (probe(r, 0x0D)) return;

    if (GAME_MEM8(0x0E) != 0) {
        if (probe(r, 0x18)) return;
        if (probe(r, 0x19)) return;
    }


    if (GAME_MEM8(0x0A) >= 0xB0) { r->carry = 0; return; }
    if ((GAME_MEM8(0x0A) & 0x0F) == 0) { r->carry = 0; return; }

    if (probe(r, 0x02)) return;
    if (probe(r, 0x0E)) return;

    if (GAME_MEM8(0x0E) == 0) { r->carry = 0; return; }

    if (probe(r, 0x1A)) return;

    r->carry = 0;
}
