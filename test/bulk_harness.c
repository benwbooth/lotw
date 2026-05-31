/* Generic bulk diff-test harness (gcc -DLOTW_HOST -Isrc).
 *
 * Maps the ROM into the full 64 KiB space, then for each stdin record runs the
 * selected ported routine (uniform `void fn(Regs*)` ABI, dispatched via the
 * generated PORT_FNS table) on injected registers/flags/RAM, and writes the
 * results back. The set of routines is supplied by a generated dispatch.c.
 *
 * argv[1] = rom path.
 * Record in/out: [id u8][a][x][y][c][z][n][v][ram 2048]   (id only meaningful in)
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "regs.h"

u8 NES_MEM[0x10000];
void nes_reg_write(u16 addr, u8 val) { (void)addr; (void)val; }

extern PortFn PORT_FNS[];
extern int PORT_N;

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb");
    static unsigned char rom[196624];
    if (!f || fread(rom, 1, sizeof rom, f) != sizeof rom) { perror("rom"); exit(3); }
    fclose(f);
    memcpy(NES_MEM + 0xC000, rom + 0x10 + 14 * 0x2000, 0x4000);  /* fixed 14+15 */
    memcpy(NES_MEM + 0xA000, rom + 0x10 + 13 * 0x2000, 0x2000);  /* bank 13 */
}

#define HDR 8
int main(int argc, char **argv)
{
    if (argc < 2) { fprintf(stderr, "usage: bulk_harness rom.nes\n"); return 1; }
    load_rom(argv[1]);
    unsigned char in[HDR + 0x800], out[HDR + 0x800];
    while (fread(in, 1, sizeof in, stdin) == sizeof in) {
        u8 id = in[0];
        Regs r;
        r.a = in[1]; r.x = in[2]; r.y = in[3];
        r.c = in[4]; r.z = in[5]; r.n = in[6]; r.v = in[7];
        /* Reset the non-ROM space ($0000-$9FFF) per record so routines that write
         * outside RAM (e.g. into $0800-$9FFF) match the oracle's fresh-per-state
         * memory; ROM at $A000-$FFFF is constant. Then inject the test RAM. */
        memset(NES_MEM, 0, 0xA000);
        memcpy(NES_MEM, in + HDR, 0x800);
        if (id >= PORT_N) { fprintf(stderr, "bad id %u\n", id); return 2; }
        PORT_FNS[id](&r);
        out[1] = r.a; out[2] = r.x; out[3] = r.y;
        out[4] = r.c; out[5] = r.z; out[6] = r.n; out[7] = r.v;
        out[0] = id;
        memcpy(out + HDR, NES_MEM, 0x800);
        fwrite(out, 1, sizeof out, stdout);
    }
    return 0;
}
