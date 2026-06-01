/* $C000 main_init — power-on / soft-reset boot. Reached by JMP from reset.
 * Initialises the stack, silences and configures the PPU and APU, waits two
 * vblanks on PPUSTATUS, parks MMC3 mirroring, seeds banks 0C/0D
 * (farcall_bank_0C0D_seed), zeroes the engine state (ram_state_init), runs the
 * title/character-select driver via a far-call to $AE64, places the player at a
 * default start position, assembles the scene and runs one game_update, then
 * falls through into the main game loop (main_loop_dispatch).
 *
 * Far-calls (via farcall_bank_0C0D $CC9C): the dispatcher saves the r6/r7 bank
 * shadows ($30->$32, $31->$33), maps banks 0C/0D (select_shadow $25=$07), runs the
 * $hi:$lo target ($0E/$0F), then restores the shadows and leaves $25=$06 — modelled
 * here per src/ported/sub_F349.c. The one far-call target is $AE64 (sub_AE64).
 * farcall_bank_0C0D_seed ($CD08) is a direct (fixed-bank) JSR.
 *
 * INSPECTION-PORT (no diff-test spec): boot-time PPU/APU/MMC3 register programming
 * plus two PPUSTATUS vblank-wait loops that never terminate in flat memory
 * (PPUSTATUS reads 0, so bit7 is never set) and an infinite main loop at the tail.
 * Real hardware/integration supplies PPUSTATUS and the NMI. The vblank waits are
 * transcribed faithfully but documented as integration-only so the port stays
 * non-blocking and compiling. Integration-verified.
 *
 * NOTE: the L_C026 / L_C041 re-entry labels (soft-restart targets used by
 * main_loop_dispatch via JMP $C026) live inside this routine; they are not
 * separately reachable as functions here. */
#include "ram.h"
#include "regs.h"

void farcall_bank_0C0D_seed(Regs *r);   /* $CD08 direct */
void ram_state_init(Regs *r);           /* $D1C8 */
void scene_assemble(Regs *r);           /* $C8F2 */
void game_update(Regs *r);              /* $D42B */
void sub_AE64(Regs *r);                 /* far-call $AE64 */
void main_loop_dispatch(Regs *r);       /* fall-through tail */

/* farcall_bank_0C0D ($CC9C) side-effects (plain dispatcher), per sub_F349.c. */
static void farcall_0C0D(Regs *r, u8 lo, u8 hi, void (*target)(Regs *))
{
    u8 old6 = RAM8(0x30), old7 = RAM8(0x31);
    RAM8(0x32) = old6; RAM8(0x33) = old7;
    RAM8(0x0E) = lo; RAM8(0x0F) = hi;
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
    target(r);                                  /* JMP ($000E) */
    RAM8(0x31) = old7; RAM8(0x30) = old6; RAM8(0x25) = 0x06; NES_PRG_SYNC();
}

void main_init(Regs *r)
{
    /* SEI / LDX #$FF / TXS — stack init (implicit in the flat host model). */
    REG_W(0x2000, 0x00);                /* PPUCTRL = 0 */
    REG_W(0x2001, 0x00);                /* PPUMASK = 0 */
    REG_W(0x4010, 0x00);                /* DMC_FREQ = 0 */
    RAM8(0x0027) = 0x1F;                /* STA a:$0027 (apu enable shadow) */
    REG_W(0x4015, 0x1F);                /* APU_STATUS = $1F */
    REG_W(0x4017, 0xC0);                /* APU_FRAME = $C0 */

    /* L_C01C / L_C021 — two PPUSTATUS vblank waits.
     * In flat memory PPUSTATUS reads 0 so bit7 never sets; integration supplies a
     * real PPUSTATUS. Transcribed but left non-blocking (see header). */
    /* do { a = REG_R(PPUSTATUS); } while (!(a & 0x80));   x2 (integration-only) */

    /* L_C026 — soft-restart re-entry */
    /* LDX #$FF / TXS */
    REG_W(0xA000, 0x00);                /* MMC3_MIRROR = 0 */
    farcall_bank_0C0D_seed(r);          /* JSR farcall_bank_0C0D_seed ($CD08) */

    /* L_C041 */
    ram_state_init(r);                  /* JSR ram_state_init ($D1C8) */
    farcall_0C0D(r, 0x64, 0xAE, sub_AE64);      /* far-call $AE64 */

    /* L_C04F — default start position */
    RAM8(0x46) = 0x00;
    RAM8(0x7B) = 0x00;                  /* scroll_x_fine */
    RAM8(0x43) = 0x00;                  /* player_x_fine */
    RAM8(0x7C) = 0x30;                  /* scroll_x_tile */
    RAM8(0x44) = 0x3C;                  /* player_x_tile */
    RAM8(0x45) = 0xA0;                  /* player_y */
    scene_assemble(r);                  /* JSR scene_assemble ($C8F2) */
    RAM8(0x20) = 0x08;
    game_update(r);                     /* JSR game_update ($D42B) */

    main_loop_dispatch(r);              /* fall through into main_loop_dispatch */
}
