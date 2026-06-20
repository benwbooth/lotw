




#include "game_memory.h"
#include "routine_context.h"

void routine_0024(RoutineContext *r)
{
    r->index = 0x09;

    if ((u8)(GAME_MEM8(0x20) & 0xBF) == 0x80) {
        GAME_MEM8(0x56) = r->index;
        return;
    }

    if (GAME_MEM8(0x4B) == 0) goto flow_0108;
    if (GAME_MEM8(0x4B) & 0x80) {

        if (GAME_MEM8(0x4F) == 0) {
            GAME_MEM8(0x56) = r->index;
            return;
        }
        goto flow_0112;
    }
    if (GAME_MEM8(0x4E) != 0) goto flow_0112;
    if ((GAME_MEM8(0x20) & 0x04) == 0) goto flow_0108;
    r->index = 0x0D;
    GAME_MEM8(0x56) = r->index;
    return;

flow_0108:
    r->index = 0x01;
    r->offset = 0x00;
    if (GAME_MEM8(0x49) & 0x80) goto flow_0109;
    if (GAME_MEM8(0x49) == 0) return;
    r->offset = 0x40;
flow_0109:
    GAME_MEM8(0x08) = r->index;
    GAME_MEM8(0x56) = (u8)((GAME_MEM8(0x56) & 0x07) | GAME_MEM8(0x08));
    GAME_MEM8(0x57) = r->offset;
    return;

flow_0112:
    r->index = 0x39;
    r->offset = 0x00;
    if (GAME_MEM8(0x49) & 0x80) goto flow_0113;
    if (GAME_MEM8(0x49) == 0) return;
    r->offset = 0x40;
flow_0113:
    GAME_MEM8(0x08) = r->index;
    GAME_MEM8(0x56) = (u8)((GAME_MEM8(0x56) & 0x03) | GAME_MEM8(0x08));
    GAME_MEM8(0x57) = r->offset;
}
