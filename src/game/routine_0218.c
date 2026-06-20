









#include "game_memory.h"
#include "routine_context.h"

void routine_0220(RoutineContext *r);
void routine_0221(RoutineContext *r);
void routine_0222(RoutineContext *r);
void routine_0223(RoutineContext *r);
void routine_0224(RoutineContext *r);
void routine_0225(RoutineContext *r);
void routine_0226(RoutineContext *r);
void routine_0228(RoutineContext *r);
void routine_0229(RoutineContext *r);


static const u16 boss_state_table[9] = {
    0xEAFD, 0xEB69, 0xEB90, 0xEBD8, 0xEC76, 0xECA8, 0xED2A, 0xED6F, 0xED9F,
};

void routine_0218(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
    u8 idx = GAME_MEM8((u16)(ptr + 8));
    if (idx >= 0x09)
        idx = 0x00;


    GAME_MEM8(0x0E) = (u8)(boss_state_table[idx] & 0xFF);
    GAME_MEM8(0x0F) = (u8)(boss_state_table[idx] >> 8);

    switch (idx) {
    case 0: routine_0220(r); break;
    case 1: routine_0221(r); break;
    case 2: routine_0222(r); break;
    case 3: routine_0223(r); break;
    case 4: routine_0224(r); break;
    case 5: routine_0225(r); break;
    case 6: routine_0226(r); break;
    case 7: routine_0228(r); break;
    case 8: routine_0229(r); break;
    }
}
