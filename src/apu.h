








#ifndef LOTW_APU_H
#define LOTW_APU_H
#include "platform.h"

#ifdef __cplusplus
extern "C" {
#endif

#define APU_SR 44100

void apu_reset(void);
void apu_write(u16 addr, u8 val);
void apu_frame(void);
void apu_gen(short *out, int n);

int  wav_write(const char *path, const short *samples, int n, int rate);

#ifdef __cplusplus
}
#endif

#endif
