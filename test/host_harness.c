/* Host differential-test harness (gcc -DLOTW_HOST).
 *
 * Maps the ROM into the full 64 KiB address space (fixed banks 14+15 at $C000,
 * bank 13 at $A000) so pointer dereferences match the m6502 oracle, then runs
 * each ported routine on injected RAM + input registers and reports the
 * resulting RAM, registers, and carry.
 *
 * argv[1] = path to rom/lotw.nes
 * Record in:  [routine_id u8][a u8][x u8][y u8][ram 2048]
 * Record out: [a u8][x u8][y u8][carry u8][ram 2048]
 */
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "../src/nes.h"
#include "../src/rng.h"
#include "../src/leaves.h"

u8 NES_MEM[0x10000];
void nes_reg_write(u16 addr, u8 val) { (void)addr; (void)val; }

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb");
    static unsigned char rom[196624];
    if (!f || fread(rom, 1, sizeof rom, f) != sizeof rom) { perror("rom"); exit(3); }
    fclose(f);
    /* PRG: file 0x10.. ; fixed banks 14+15 -> $C000-$FFFF, bank 13 -> $A000 */
    memcpy(NES_MEM + 0xC000, rom + 0x10 + 14 * 0x2000, 0x4000);
    memcpy(NES_MEM + 0xA000, rom + 0x10 + 13 * 0x2000, 0x2000);
}

int main(int argc, char **argv)
{
    if (argc < 2) { fprintf(stderr, "usage: host_harness rom.nes\n"); return 1; }
    load_rom(argv[1]);
    unsigned char in[4 + 0x800], out[4 + 0x800];
    while (fread(in, 1, sizeof in, stdin) == sizeof in) {
        u8 id = in[0], a = in[1], x = in[2], y = in[3], carry = 0;
        memcpy(NES_MEM, in + 4, 0x800);            /* inject RAM ($0000-$07FF) */
        switch (id) {
        case 0: a = rng_update(a); break;          /* $CC64 */
        case 1: x = sub_E41E(); break;             /* $E41E */
        case 2: carry = sub_F233(y); break;        /* $F233 */
        case 3: x = inc16_95(); break;             /* $FD6B */
        default: fprintf(stderr, "bad routine id %u\n", id); return 2;
        }
        out[0] = a; out[1] = x; out[2] = y; out[3] = carry;
        memcpy(out + 4, NES_MEM, 0x800);
        fwrite(out, 1, sizeof out, stdout);
    }
    return 0;
}
