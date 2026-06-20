#ifndef LOTW_NATIVE_FRAME_RUNNER_C_H
#define LOTW_NATIVE_FRAME_RUNNER_C_H

#include "regs.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct LotwFrameRunner LotwFrameRunner;

LotwFrameRunner *lotw_frame_runner_create(PortFn entry);
void lotw_frame_runner_destroy(LotwFrameRunner *runner);

/* Start the game thread and run until it reaches a frame wait.
 * Return non-zero while the game is parked at a frame boundary; return zero if
 * the entry routine returned. */
int lotw_frame_runner_start(LotwFrameRunner *runner);

/* Resume after the host has committed/rendered one frame; run until the next
 * frame wait. Return non-zero while the game is still alive. */
int lotw_frame_runner_resume_until_wait(LotwFrameRunner *runner);

int lotw_frame_runner_done(const LotwFrameRunner *runner);
Regs *lotw_frame_runner_regs(LotwFrameRunner *runner);

#ifdef __cplusplus
}
#endif

#endif /* LOTW_NATIVE_FRAME_RUNNER_C_H */
