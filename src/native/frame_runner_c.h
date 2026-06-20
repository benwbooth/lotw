#ifndef LOTW_NATIVE_FRAME_RUNNER_C_H
#define LOTW_NATIVE_FRAME_RUNNER_C_H

#include "routine_context.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct LotwFrameRunner LotwFrameRunner;

LotwFrameRunner *lotw_frame_runner_create(RoutineFn entry);
void lotw_frame_runner_destroy(LotwFrameRunner *runner);




int lotw_frame_runner_start(LotwFrameRunner *runner);



int lotw_frame_runner_resume_until_wait(LotwFrameRunner *runner);

int lotw_frame_runner_done(const LotwFrameRunner *runner);
RoutineContext *lotw_frame_runner_context(LotwFrameRunner *runner);

#ifdef __cplusplus
}
#endif

#endif
