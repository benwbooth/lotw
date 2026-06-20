



#include "game_memory.h"
#include "routine_context.h"
void farcall_return_home(RoutineContext *r)
{
    (void)r;
    GAME_MEM8(0x31) = GAME_MEM8(0x33);
    GAME_MEM8(0x30) = GAME_MEM8(0x32);

}
