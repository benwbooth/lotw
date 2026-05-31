/* Headless software 2A03 APU for the Legacy of the Wizard PC port.
 *
 * The decompiled sound engine (sound_tick + the voice updaters) writes the
 * $4000-$4017 audio registers every frame, exactly as on hardware. This module
 * is the chip on the other end: it captures those writes and synthesizes audio
 * samples (pulse 1/2, triangle, noise), so we can render a song to a WAV
 * headlessly. Register-driven phase-accumulator synthesis — the engine already
 * computes per-frame periods/volumes, so this stays simple.
 */
#ifndef LOTW_APU_H
#define LOTW_APU_H
#include "nes.h"

#define APU_SR 44100               /* output sample rate */

void apu_reset(void);
void apu_write(u16 addr, u8 val);  /* $4000-$4017 register write */
void apu_frame(void);              /* call once per game frame (~60 Hz): clocks length */
void apu_gen(short *out, int n);   /* synthesize n mono 16-bit samples */

int  wav_write(const char *path, const short *samples, int n, int rate);

#endif /* LOTW_APU_H */
