















#include "ppu.h"
#include "apu.h"
#include "routine_context.h"
#include "native/frame_runner_c.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>

u8 LOTW_MEMORY[0x10000];
extern void (*apu_write_hook)(u16, u8);
void reset(RoutineContext*);
void vblank_commit(RoutineContext*);



static u8         *g_input;
static long        g_ninput, g_inpos;
static u8 next_input(void) { return (g_input && g_inpos < g_ninput) ? g_input[g_inpos++] : 0x00; }

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb"); if (!f) { perror(path); exit(1); }
    static u8 rom[1 << 20]; size_t n = fread(rom, 1, sizeof rom, f); fclose(f); (void)n;
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    for (unsigned a = 0; a < 0x0800; a++)
        LOTW_MEMORY[a] = (a & 4) ? 0xFF : 0x00;
    ppu_load_prg(rom + 16, prg);
    ppu_load_chr(rom + 16 + prg, chr);
    ppu_reset(); apu_reset(); apu_write_hook = apu_write;
    memcpy(&LOTW_MEMORY[0xC000], rom + 16 + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12);
    ppu_map_prg(0xA000, 13);
    ppu_set_vblank(1);
}

int main(int argc, char **argv)
{
    const char *path   = argc > 1 ? argv[1] : "rom/lotw.nes";
    int         frames = argc > 2 ? atoi(argv[2]) : 2000;
    const char *outp   = argc > 3 ? argv[3] : "/tmp/port_trace.bin";
    const char *inp    = argc > 4 ? argv[4] : NULL;

    if (inp) { FILE *fi = fopen(inp, "rb"); if (fi) {
        fseek(fi, 0, SEEK_END); g_ninput = ftell(fi); fseek(fi, 0, SEEK_SET);
        g_input = malloc(g_ninput); if (fread(g_input, 1, g_ninput, fi) != (size_t)g_ninput) g_ninput = 0; fclose(fi); } }

    load_rom(path);
    lotw_next_input = next_input;

    LotwFrameRunner *runner = lotw_frame_runner_create(reset);
    if (!runner) {
        fprintf(stderr, "lockstep_port: failed to create frame runner\n");
        return 1;
    }

    FILE *out = fopen(outp, "wb");
    if (!out) {
        perror(outp);
        lotw_frame_runner_destroy(runner);
        return 1;
    }



    if (!lotw_frame_runner_start(runner)) {
        fprintf(stderr, "lockstep_port: game loop returned during boot\n");
        lotw_frame_runner_destroy(runner);
        fclose(out);
        return 1;
    }
    RoutineContext *regs = lotw_frame_runner_context(runner);






    FILE *rc = fopen("/tmp/port_readcount.bin", "wb");
    for (int i = 0; i < frames; i++) {
        vblank_commit(regs);
        if (!lotw_frame_runner_resume_until_wait(runner)) {
            fprintf(stderr, "lockstep_port: game loop returned at frame %d\n", i);
            break;
        }
        fwrite(&LOTW_MEMORY[0x0000], 1, 0x800, out);
        { unsigned c = (unsigned)g_inpos; fwrite(&c, 4, 1, rc); }
    }
    if (rc) fclose(rc);
    fprintf(stderr, "lockstep_port: consumed %ld/%ld per-read inputs\n", g_inpos, g_ninput);
    fclose(out);
    lotw_frame_runner_destroy(runner);
    fprintf(stderr, "lockstep_port: wrote %d frames x 0x800 to %s\n", frames, outp);
    return 0;
}
