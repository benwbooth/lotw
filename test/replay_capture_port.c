







#include "ppu.h"
#include "apu.h"
#include "routine_context.h"
#include "native/frame_runner_c.h"
#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>

u8 LOTW_MEMORY[0x10000];
extern void (*apu_write_hook)(u16, u8);

void reset(RoutineContext*);
void vblank_commit(RoutineContext*);

static FILE       *g_apu_trace;
static int         g_frame;

static void apu_write_traced(u16 addr, u8 val)
{
    if (g_apu_trace)
        fprintf(g_apu_trace, "%d\t%04X\t%02X\n", g_frame, addr, val);
    apu_write(addr, val);
}

static void mkdir_p(const char *path)
{
    char tmp[512];
    size_t n = strlen(path);
    if (n >= sizeof tmp) {
        fprintf(stderr, "output path too long: %s\n", path);
        exit(1);
    }
    memcpy(tmp, path, n + 1);
    for (char *p = tmp + 1; *p; p++) {
        if (*p == '/') {
            *p = 0;
            mkdir(tmp, 0777);
            *p = '/';
        }
    }
    mkdir(tmp, 0777);
}

static u8 button_bit(const char *s)
{
    if (!strcmp(s, "A")) return 0x01;
    if (!strcmp(s, "B")) return 0x02;
    if (!strcmp(s, "select")) return 0x04;
    if (!strcmp(s, "start")) return 0x08;
    if (!strcmp(s, "up")) return 0x10;
    if (!strcmp(s, "down")) return 0x20;
    if (!strcmp(s, "left")) return 0x40;
    if (!strcmp(s, "right")) return 0x80;
    fprintf(stderr, "unknown replay button: %s\n", s);
    exit(1);
}

static u8 *parse_replay(const char *path, int *out_frames)
{
    FILE *f = fopen(path, "r");
    if (!f) { perror(path); exit(1); }

    int cap = 4096, n = 0;
    u8 *frames = calloc((size_t)cap + 1, 1);
    if (!frames) { perror("calloc"); exit(1); }

    char line[512];
    while (fgets(line, sizeof line, f)) {
        char *hash = strchr(line, '#');
        if (hash) *hash = 0;

        char *tok = strtok(line, " \t\r\n");
        if (!tok) continue;
        if (strcmp(tok, "frame")) {
            fprintf(stderr, "unknown replay directive: %s\n", tok);
            exit(1);
        }

        tok = strtok(NULL, " \t\r\n");
        if (!tok) {
            fprintf(stderr, "missing frame count in %s\n", path);
            exit(1);
        }
        int count = atoi(tok);
        if (count < 1) {
            fprintf(stderr, "invalid frame count: %s\n", tok);
            exit(1);
        }

        u8 b = 0;
        while ((tok = strtok(NULL, " \t\r\n")) != NULL)
            b |= button_bit(tok);

        if (n + count + 1 >= cap) {
            while (n + count + 1 >= cap) cap *= 2;
            frames = realloc(frames, (size_t)cap + 1);
            if (!frames) { perror("realloc"); exit(1); }
        }
        for (int i = 0; i < count; i++)
            frames[++n] = b;
    }
    fclose(f);
    *out_frames = n;
    return frames;
}

static unsigned char *parse_capture_set(const char *csv, int *out_max)
{
    int cap = 4096, max = 0;
    unsigned char *set = calloc((size_t)cap + 1, 1);
    if (!set) { perror("calloc"); exit(1); }

    const char *p = csv;
    while (*p) {
        while (*p == ',' || isspace((unsigned char)*p)) p++;
        if (!*p) break;
        char *end = NULL;
        long v = strtol(p, &end, 10);
        if (end == p || v < 1) {
            fprintf(stderr, "invalid capture frame near: %s\n", p);
            exit(1);
        }
        if (v >= cap) {
            while (v >= cap) cap *= 2;
            set = realloc(set, (size_t)cap + 1);
            if (!set) { perror("realloc"); exit(1); }
            memset(set + max + 1, 0, (size_t)(cap - max));
        }
        set[v] = 1;
        if ((int)v > max) max = (int)v;
        p = end;
    }
    *out_max = max;
    return set;
}

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb"); if (!f) { perror(path); exit(1); }
    static u8 rom[1 << 20]; size_t n = fread(rom, 1, sizeof rom, f); fclose(f); (void)n;
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    for (unsigned a = 0; a < 0x0800; a++)
        LOTW_MEMORY[a] = (a & 4) ? 0xFF : 0x00;
    ppu_load_prg(rom + 16, prg);
    ppu_load_chr(rom + 16 + prg, chr);
    ppu_reset(); apu_reset(); apu_write_hook = g_apu_trace ? apu_write_traced : apu_write;
    memcpy(&LOTW_MEMORY[0xC000], rom + 16 + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12);
    ppu_map_prg(0xA000, 13);
    ppu_set_vblank(1);
}

static void write_ram(const char *path)
{
    FILE *f = fopen(path, "wb");
    if (!f) { perror(path); exit(1); }
    fwrite(LOTW_MEMORY, 1, 0x800, f);
    fclose(f);
}

static void write_ppu_state(const char *path)
{
    FILE *f = fopen(path, "wb");
    if (!f) { perror(path); exit(1); }

    int mirror = ppu_mirror_dbg();
    for (int nt = 0; nt < 4; nt++) {
        int phys = (mirror == 0) ? (nt >> 1) : (nt & 1);
        fwrite(ppu_vram + phys * 0x400, 1, 0x400, f);
    }
    fwrite(ppu_pal, 1, 0x20, f);
    fwrite(ppu_oam, 1, 0x100, f);
    fputc(ppu_ctrl, f);
    fputc(ppu_mask, f);
    fputc(ppu_scroll_x, f);
    fputc(ppu_scroll_y, f);
    fputc(mirror, f);
    fclose(f);
}

int main(int argc, char **argv)
{
    const char *rom     = argc > 1 ? argv[1] : "rom/lotw.nes";
    const char *replay  = argc > 2 ? argv[2] : "fixtures/reference/outside_walk.replay";
    const char *out_dir = argc > 3 ? argv[3] : "build/port_capture/replay";
    const char *frames  = argc > 4 ? argv[4] : "1,60,120,180";

    int replay_len = 0, max_capture = 0;
    u8 *input = parse_replay(replay, &replay_len);
    unsigned char *capture = parse_capture_set(frames, &max_capture);
    int max_frame = replay_len > max_capture ? replay_len : max_capture;

    const char *apu_trace_path = getenv("LOTW_ROUTINE_APU_TRACE");
    if (apu_trace_path && *apu_trace_path) {
        g_apu_trace = fopen(apu_trace_path, "wb");
        if (!g_apu_trace) { perror(apu_trace_path); exit(1); }
        fprintf(g_apu_trace, "frame\taddr\tvalue\n");
    }

    mkdir_p(out_dir);
    load_rom(rom);

    LotwFrameRunner *runner = lotw_frame_runner_create(reset);
    if (!runner) {
        fprintf(stderr, "failed to create frame runner\n");
        return 1;
    }




    if (!lotw_frame_runner_start(runner)) {
        fprintf(stderr, "game loop returned during boot\n");
        lotw_frame_runner_destroy(runner);
        return 1;
    }
    RoutineContext *regs = lotw_frame_runner_context(runner);

    static u8 fb[PPU_W * PPU_H * 3];
    for (int frame = 1; frame <= max_frame; frame++) {
        g_frame = frame;
        ppu_set_buttons(frame <= replay_len ? input[frame] : 0);
        vblank_commit(regs);
        if (!lotw_frame_runner_resume_until_wait(runner)) {
            fprintf(stderr, "game loop returned at frame %d\n", frame);
            break;
        }
        ppu_render(fb);

        if (frame <= max_capture && capture[frame]) {
            char p[512];
            snprintf(p, sizeof p, "%s/frame_%06d.ppm", out_dir, frame);
            ppm_write(p, fb, PPU_W, PPU_H);
            snprintf(p, sizeof p, "%s/ram_%06d.bin", out_dir, frame);
            write_ram(p);
            snprintf(p, sizeof p, "%s/ppu_%06d.bin", out_dir, frame);
            write_ppu_state(p);
            fprintf(stderr,
                    "captured f%d char=%02X map=%02X,%02X px=%02X py=%02X scroll=%02X,%02X song=%02X item=%02X inv0=%02X mirror=%d\n",
                    frame, GAME_MEM8(0x40), GAME_MEM8(0x47), GAME_MEM8(0x48), GAME_MEM8(0x44), GAME_MEM8(0x45),
                    GAME_MEM8(0x1C), GAME_MEM8(0x1E), GAME_MEM8(0x8E), GAME_MEM8(0x55), GAME_MEM8(0x60), ppu_mirror_dbg());
        }
    }

    free(input);
    free(capture);
    lotw_frame_runner_destroy(runner);
    if (g_apu_trace)
        fclose(g_apu_trace);
    return 0;
}
