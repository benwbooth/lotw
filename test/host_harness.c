/* Host differential-test harness (gcc -DLOTW_HOST).
 *
 * Reads fixed-size records from stdin and runs the corresponding ported C
 * routine on the supplied RAM image, writing the resulting RAM back to stdout.
 * tools/re/difftest.py feeds the same states to the m6502 oracle (original
 * 6502 bytes) and compares.
 *
 * Record in:  [routine_id u8][reg_a u8][ram 2048]
 * Record out: [reg_a u8][ram 2048]
 */
#include <stdio.h>
#include <string.h>
#include "../src/nes.h"
#include "../src/rng.h"

u8 NES_RAM[0x800];

/* Mock hardware-register writes (host build): ignored for pure-RAM routines. */
void nes_reg_write(u16 addr, u8 val) { (void)addr; (void)val; }

int main(void)
{
    unsigned char in[1 + 1 + 0x800];
    unsigned char out[1 + 0x800];
    while (fread(in, 1, sizeof in, stdin) == sizeof in) {
        u8 id = in[0], a = in[1];
        memcpy(NES_RAM, in + 2, 0x800);
        switch (id) {
        case 0: a = rng_update(a); break;       /* $CC64 */
        default: fprintf(stderr, "bad routine id %u\n", id); return 2;
        }
        out[0] = a;
        memcpy(out + 1, NES_RAM, 0x800);
        fwrite(out, 1, sizeof out, stdout);
    }
    return 0;
}
