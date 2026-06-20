






#include "game_memory.h"
#include "routine_context.h"

void routine_0086(RoutineContext *r);
void routine_0085(RoutineContext *r);
void text_attr_build(RoutineContext *r);
void routine_0087(RoutineContext *r);

void scene_assemble(RoutineContext *r)
{
    routine_0086(r);
    routine_0085(r);







    r->carry = (u8)(((GAME_MEM8(0x76) + 0x03) > 0xFF) ? 1 : 0);
    text_attr_build(r);
    routine_0087(r);
}
