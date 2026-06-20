




















#include "game_memory.h"
#include "routine_context.h"

void routine_0233(RoutineContext *r);
void routine_0236(RoutineContext *r);
void routine_0108(RoutineContext *r);
void routine_0247(RoutineContext *r);
void routine_0252(RoutineContext *r);
void routine_0238(RoutineContext *r);
void routine_0239(RoutineContext *r);
void routine_0250(RoutineContext *r);
void routine_0242(RoutineContext *r);

void routine_0222(RoutineContext *r)
{
    int reached_EBC6 = 0;
    int reached_EBCC = 0;
    int done = 0;

    if ((GAME_MEM8(0xF5) | GAME_MEM8(0xF7)) == 0)
        routine_0233(r);


    if (GAME_MEM8(0xF0) != 0) {
        routine_0236(r);
        if (r->carry == 0)
            reached_EBC6 = 1;
        else
            done = 1;
    } else {

        u16 ptr = (u16)(GAME_MEM8(0xE7) | (GAME_MEM8(0xE8) << 8));
        r->offset = GAME_MEM8((u16)(ptr + 9));
        r->value = GAME_MEM8(0xF4);
        routine_0108(r);

        routine_0247(r);
        if (r->carry) {
            reached_EBCC = 1;
        } else {
            r->offset = 0x01;
            routine_0252(r);
            if (r->carry == 0) {
                reached_EBCC = 1;
            } else if (GAME_MEM8(0x0E) == 0) {
                reached_EBC6 = 1;
            } else {
                r->offset = 0x0D;
                routine_0252(r);
                if (r->carry == 0)
                    reached_EBCC = 1;
                else
                    reached_EBC6 = 1;
            }
        }
    }

    if (!done) {
        if (reached_EBCC)
            routine_0239(r);
        else if (reached_EBC6)
            routine_0238(r);
    }


    routine_0250(r);
    routine_0242(r);

}
