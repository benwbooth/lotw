/* Game-driven PPU frame: run the decompiled C against the software PPU shim and
 * render what the game actually puts on screen. We warp straight into an in-game
 * state (skipping the title/character-select input loops): seed the far-call
 * banks, init engine state, place the player, assemble the scene (which queues
 * VRAM jobs that the shim's queue_ppu_job_and_wait flushes through the NMI into
 * the software PPU), run one game_update, then rasterize to a PPM.
 *
 *   build: gcc -O2 -DLOTW_HOST -DLOTW_SHIM -Isrc src/ppu.c src/ported/*.c \
 *              test/ppu_game_driver.c -o build/game_driver
 */
#include "ppu.h"
#include "regs.h"
#include <stdio.h>
#include <string.h>

u8 NES_MEM[0x10000];

/* engine entry points we drive directly */
void ram_state_init(Regs *r);
void farcall_bank_0C0D_seed(Regs *r);
void scene_assemble(Regs *r);
void game_update(Regs *r);

int main(int argc, char **argv)
{
    const char *path = argc > 1 ? argv[1] : "rom/lotw.nes";
    FILE *f = fopen(path, "rb");
    if (!f) { perror("rom"); return 1; }
    static u8 rom[1 << 20];
    size_t n = fread(rom, 1, sizeof rom, f); fclose(f);
    if (n < 16) { fprintf(stderr, "short rom\n"); return 1; }
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    const u8 *PRG = rom + 16, *CHR = rom + 16 + prg;

    ppu_load_prg(PRG, prg);
    ppu_load_chr(CHR, chr);
    ppu_reset();

    /* fixed banks 14+15 -> $C000-$FFFF; default low banks (seed will remap) */
    memcpy(&NES_MEM[0xC000], PRG + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12);
    ppu_map_prg(0xA000, 13);
    ppu_set_vblank(1);              /* pretend we're in vblank so any wait clears */

    Regs r; memset(&r, 0, sizeof r);

    fprintf(stderr, "ram_state_init...\n");   ram_state_init(&r);
    fprintf(stderr, "seed banks...\n");        farcall_bank_0C0D_seed(&r);

    /* default start position (from main_init L_C04F) */
    RAM8(0x46) = 0x00;             /* $46 */
    RAM8(0x7B) = 0x00;             /* scroll_x_fine */
    RAM8(0x43) = 0x00;             /* player_x_fine */
    RAM8(0x7C) = 0x30;             /* scroll_x_tile */
    RAM8(0x44) = 0x3C;             /* player_x_tile */
    RAM8(0x45) = 0xA0;             /* player_y */

    /* Reproduce the essential character-select / world-entry state that AE64 sets
     * up (it's rng-randomized normally; we pick a valid room + character 0). */
    RAM8(0x8E) = 0x09;             /* level */
    RAM8(0x41) = 0xFF;             /* character roster: all available */
    RAM8(0x39) = 0xC5; RAM8(0x3A) = 0x17; RAM8(0x3B) = 0x42;   /* seed RNG */
    RAM8(0x47) = 0x01; RAM8(0x48) = 0x05;   /* map_screen_x / map_screen_y (a valid room) */
    RAM8(0x40) = 0x00;             /* cur_character = 0 */
    for (int i = 0; i < 4; i++) RAM8(0x5C + i) = NES_MEM[0xFFA7 + i];  /* stat_jump from table */
    RAM8(0x51) = NES_MEM[0xB0AC];  /* carried_item0 from roster table */
    RAM8(0x55) = 0x00;             /* equipped_item */
    RAM8(0x2C) = 0x38;             /* mmc3_r2_shadow = char($40)+$38 (character CHR) */
    RAM8(0x2E) = 0x3E; RAM8(0x2F) = 0x20;
    RAM8(0x56) = 0x0D; RAM8(0x57) = 0x00; RAM8(0x42) = 0x01;
    RAM8(0x58) = 0x64;             /* health */
    RAM8(0x59) = 0x64;             /* magic */
    RAM8(0xEB) = 0x00;             /* reset flag clear */
    /* spawn position */
    RAM8(0x44) = 0x20; RAM8(0x45) = 0x80; RAM8(0x43) = 0x00;
    RAM8(0x7C) = 0x18; RAM8(0x7B) = 0x00;
    fprintf(stderr, "scene_assemble...\n");    scene_assemble(&r);
    /* C7B5 lays out the room: uploads the nametable columns to VRAM (the
     * room/title-entry path our warp-in skipped). */
    void sub_C7B5(Regs*); void sub_C1C7(Regs*);
    fprintf(stderr, "C7B5 (screen layout)...\n");
    RAM8(0x7C) = 0x10; sub_C7B5(&r); sub_C1C7(&r);
    RAM8(0x7C) = 0x20; sub_C7B5(&r); sub_C1C7(&r);

    /* run a few per-frame iterations so the NMI flushes the staged room to VRAM */
    void sub_F628(Regs*); void sub_E87C(Regs*); void sub_F782(Regs*);
    void sub_C15D(Regs*); void sub_C1D8(Regs*); void sub_C2B1(Regs*); void sub_C135(Regs*);
    for (int fr = 0; fr < 4; fr++) {
        fprintf(stderr, "frame %d: game_update...", fr);
        RAM8(0x36) = 0x01;
        game_update(&r);                       fprintf(stderr, " F628");
        sub_F628(&r);                          fprintf(stderr, " E87C");
        sub_E87C(&r);                          fprintf(stderr, " F782");
        sub_F782(&r);  sub_C15D(&r);
        sub_C1D8(&r);  sub_C2B1(&r);  sub_C135(&r);
        fprintf(stderr, " done\n");
    }

    /* scene_assemble has built the room background into the shim's VRAM via the
     * queued PPU jobs. Render that frame now (game_update adds the player sprite +
     * per-frame logic but enters the interactive loop, so skip it for frame 0). */
    /* --- diagnostics: what did scene_assemble actually put in the PPU? --- */
    { int seen[256] = {0}, distinct = 0;
      for (int i = 0; i < 0x3C0; i++) if (!seen[ppu_vram[i]]++) distinct++;
      fprintf(stderr, "NT0 distinct tiles=%d  (sample:", distinct);
      for (int i = 0; i < 8; i++) fprintf(stderr, " %02X", ppu_vram[i]);
      fprintf(stderr, ")\nNT1 distinct: ");
      int s2[256]={0}, d2=0; for (int i=0x400;i<0x7C0;i++) if(!s2[ppu_vram[i]]++) d2++;
      fprintf(stderr, "%d\n", d2);
      fprintf(stderr, "palette:");
      for (int i = 0; i < 0x20; i++) fprintf(stderr, " %02X", ppu_pal[i]);
      fprintf(stderr, "\nctrl=%02X mask=%02X scrollx=%d scrolly=%d\n",
              ppu_ctrl, ppu_mask, ppu_scroll_x, ppu_scroll_y);
      fprintf(stderr, "NT0 attr ($3C0):");
      for (int i = 0; i < 16; i++) fprintf(stderr, " %02X", ppu_vram[0x3C0 + i]);
      fprintf(stderr, "\nNT0 tiles row0:");
      for (int i = 0; i < 16; i++) fprintf(stderr, " %02X", ppu_vram[i]);
      fprintf(stderr, "\nNT0 tiles row8 ($100):");
      for (int i = 0; i < 16; i++) fprintf(stderr, " %02X", ppu_vram[0x100 + i]);
      fprintf(stderr, "\n"); }

    if (!(ppu_mask & 0x18)) ppu_mask = 0x1E;   /* force rendering on to visualize VRAM */
    static u8 frame[PPU_W * PPU_H * 3];
    ppu_render(frame);
    ppm_write("build/game_frame.ppm", frame, PPU_W, PPU_H);

    /* also render at scroll 0 to inspect the raw nametable without the shift */
    ppu_scroll_x = 0; ppu_scroll_y = 0;
    ppu_render(frame);
    ppm_write("build/game_frame_s0.ppm", frame, PPU_W, PPU_H);

    long lit = 0;
    for (int i = 0; i < PPU_W * PPU_H; i++)
        if (frame[i*3] | frame[i*3+1] | frame[i*3+2]) lit++;
    fprintf(stderr, "rendered build/game_frame.ppm  (%ld/%d non-black px)\n", lit, PPU_W*PPU_H);
    return 0;
}
