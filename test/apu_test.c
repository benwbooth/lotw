/* Audio: run the ported sound engine against the software 2A03 APU and
 * render a song to a WAV. song_init loads a song's channel state; sound_tick
 * (once per frame) advances it and writes the $4000-$4017 registers, which the
 * shim's apu_write captures and synthesizes.
 *
 *   build: gcc -O2 -DLOTW_HOST -DLOTW_SHIM -Isrc src/ppu.c src/apu.c \
 *              src/ported/*.c test/apu_test.c -lm -o build/apu_test
 *   run:   ./build/apu_test rom/lotw.nes [song]   -> build/song.wav
 */
#include "ppu.h"
#include "apu.h"
#include "regs.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

u8 NES_MEM[0x10000];
extern void (*apu_write_hook)(u16, u8);

static long g_apu_writes = 0;
static void apu_write_logged(u16 a, u8 v)
{
    if (g_apu_writes < 24) fprintf(stderr, "  APU $%04X=%02X\n", a, v);
    g_apu_writes++;
    apu_write(a, v);
}

void ram_state_init(Regs *r);
void farcall_bank_0C0D_seed(Regs *r);
void song_init(Regs *r);
void sound_tick(Regs *r);

#define FPS 60
#define SPF (APU_SR / FPS)        /* samples per frame */

int main(int argc, char **argv)
{
    const char *path = argc > 1 ? argv[1] : "rom/lotw.nes";
    int song = argc > 2 ? atoi(argv[2]) : 0;
    int secs = argc > 3 ? atoi(argv[3]) : 6;
    FILE *f = fopen(path, "rb"); if (!f) { perror("rom"); return 1; }
    static u8 rom[1 << 20];
    size_t n = fread(rom, 1, sizeof rom, f); fclose(f);
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    ppu_load_prg(rom + 16, prg);
    ppu_load_chr(rom + 16 + prg, chr);
    ppu_reset(); apu_reset();
    apu_write_hook = apu_write;
    memcpy(&NES_MEM[0xC000], rom + 16 + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12); ppu_map_prg(0xA000, 13);

    Regs r; memset(&r, 0, sizeof r);
    ram_state_init(&r);
    farcall_bank_0C0D_seed(&r);

    RAM8(0x8E) = (u8)song;        /* song number */
    RAM8(0x8D) = 0x00;            /* not paused */
    fprintf(stderr, "song_init(%d)...\n", song);
    song_init(&r);
    fprintf(stderr,"music bank @ $8000:"); for(int i=0;i<16;i++) fprintf(stderr," %02X",NES_MEM[0x8000+i]); fprintf(stderr,"  ptr $0E=%02X $0F=%02X\n",NES_MEM[0x0E],NES_MEM[0x0F]);
    fprintf(stderr,"chan state $93:"); for(int i=0;i<16;i++) fprintf(stderr," %02X",NES_MEM[0x93+i]); fprintf(stderr,"\n$A3 dur=%02X $A4=%02X $A5=%02X $A6=%02X  snd_bank0=%02X\n",NES_MEM[0xA3],NES_MEM[0xA4],NES_MEM[0xA5],NES_MEM[0xA6],NES_MEM[0x34]);

    int frames = secs * FPS, total = frames * SPF;
    short *buf = malloc(sizeof(short) * total);
    for (int fr = 0; fr < frames; fr++) {
        sound_tick(&r);
        apu_frame();
        apu_gen(buf + fr * SPF, SPF);
    }
    wav_write("build/song.wav", buf, total, APU_SR);

    /* report: non-silent sample fraction + peak (sanity) */
    long nz = 0; int peak = 0;
    for (int i = 0; i < total; i++) { if (buf[i]) nz++; int a = buf[i] < 0 ? -buf[i] : buf[i]; if (a > peak) peak = a; }
    fprintf(stderr, "wrote build/song.wav: %d samples, %ld non-zero, peak=%d\n", total, nz, peak);
    free(buf);
    return 0;
}
