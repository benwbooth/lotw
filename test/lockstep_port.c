/* Headless lockstep tracer for the C port — co-simulation against FCEUX.
 *
 * Runs the real boot path (reset -> main_init -> main_loop) on the same ucontext
 * coroutine the SDL front-end uses, but with NO window/audio. Each frame it fires
 * the NMI and writes the 2 KiB of CPU RAM ($0000-$07FF — which includes the MMC3
 * bank shadows $2A-$31, the RNG state, OAM page $02, and all game state) to a raw
 * binary trace. tools/re/lockstep.py diffs this against FCEUX's per-frame RAM dump
 * of the real ROM under the same input; the first divergent (frame,address) is the
 * bug — auto-localizing what the RAM-only/isolation diff-test is blind to (PPU
 * writes aside; that's a second layer).
 *
 *   build: cmake --build build/cmake --target lockstep_port   (no SDL dependency)
 *   run:   ./lockstep_port rom/lotw.nes <frames> [out.bin] [input.bin]
 *     input.bin: optional 1 byte/frame of NES buttons (bit7 A..bit0 Right, the
 *     ppu_set_buttons order); absent => zero input (attract demo).
 */
#include "ppu.h"
#include "apu.h"
#include "regs.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ucontext.h>

u8 NES_MEM[0x10000];
extern void (*apu_write_hook)(u16, u8);
void reset(Regs*);
void nmi_handler(Regs*);

static Regs        g_regs;
static ucontext_t  g_main_ctx, g_game_ctx;
static int         g_done;
static char        g_stack[16 * 1024 * 1024];

/* Per-READ input (content-aligned): served to read_controllers via nes_next_input
 * so input advances by controller-read count, immune to frame-timing slips. */
static u8         *g_input;
static long        g_ninput, g_inpos;
static u8 next_input(void) { return (g_input && g_inpos < g_ninput) ? g_input[g_inpos++] : 0x00; }

static void frame_yield(Regs *r) { Regs s = *r; swapcontext(&g_game_ctx, &g_main_ctx); *r = s; }
static void game_entry(void) { reset(&g_regs); g_done = 1; swapcontext(&g_game_ctx, &g_main_ctx); }

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb"); if (!f) { perror(path); exit(1); }
    static u8 rom[1 << 20]; size_t n = fread(rom, 1, sizeof rom, f); fclose(f); (void)n;
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    for (unsigned a = 0; a < 0x0800; a++)     /* match FCEUX power-on RAM pattern */
        NES_MEM[a] = (a & 4) ? 0xFF : 0x00;   /* 4 bytes $00, 4 bytes $FF, repeating */
    ppu_load_prg(rom + 16, prg);
    ppu_load_chr(rom + 16 + prg, chr);
    ppu_reset(); apu_reset(); apu_write_hook = apu_write;
    memcpy(&NES_MEM[0xC000], rom + 16 + (prg - 0x4000), 0x4000);
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
    nes_next_input = next_input;   /* content-aligned per-read input */
    getcontext(&g_game_ctx);
    g_game_ctx.uc_stack.ss_sp = g_stack;
    g_game_ctx.uc_stack.ss_size = sizeof g_stack;
    g_game_ctx.uc_link = &g_main_ctx;
    makecontext(&g_game_ctx, game_entry, 0);
    nes_vblank_wait = frame_yield;

    FILE *out = fopen(outp, "wb"); if (!out) { perror(outp); return 1; }

    /* Prime: run boot (reset -> main_init -> ...) up to the first vblank-wait so the
     * game is parked waiting for its first NMI. */
    swapcontext(&g_main_ctx, &g_game_ctx);

    /* Per frame, mirror FCEUX's emu.registerafter sampling point (END of frame =
     * after the NMI AND after the main code reacts to it): fire the NMI, hand the
     * frame's input to the main code, let it run to the next vblank-wait, THEN dump.
     * (Dumping right after the NMI — before main reacts — samples a sub-frame too
     * early and drifts one frame per wait loop vs. the real ROM.) */
    FILE *rc = fopen("/tmp/port_readcount.bin", "wb");   /* per-frame cumulative read count */
    for (int i = 0; i < frames; i++) {
        nmi_handler(&g_regs);
        swapcontext(&g_main_ctx, &g_game_ctx);   /* input pulled per-read via nes_next_input */
        if (g_done) { fprintf(stderr, "lockstep_port: game loop returned at frame %d\n", i); break; }
        fwrite(&NES_MEM[0x0000], 1, 0x800, out);   /* 2 KiB CPU RAM signature (end of frame) */
        { unsigned c = (unsigned)g_inpos; fwrite(&c, 4, 1, rc); }
    }
    if (rc) fclose(rc);
    fprintf(stderr, "lockstep_port: consumed %ld/%ld per-read inputs\n", g_inpos, g_ninput);
    fclose(out);
    fprintf(stderr, "lockstep_port: wrote %d frames x 0x800 to %s\n", frames, outp);
    return 0;
}
