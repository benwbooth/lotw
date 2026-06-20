



#ifndef LOTW_ROUTINE_CONTEXT_H
#define LOTW_ROUTINE_CONTEXT_H
#include "platform.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct {
    u8 value;
    u8 index;
    u8 offset;
    u8 carry;
    u8 zero;
    u8 negative;
    u8 overflow;
} RoutineContext;







typedef void (*RoutineFn)(RoutineContext *r);

#ifdef __cplusplus
}
#endif

#endif
