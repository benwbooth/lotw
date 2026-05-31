/* Headless software PPU for the Legacy of the Wizard PC port.
 *
 * The decompiled game logic talks to the picture chip exactly as on hardware:
 * it pokes the $2000-$2007 / $4014 registers (via REG_W) and reads $2002 (via
 * REG_R). This module IS the chip on the other end of those pokes — it keeps the
 * software graphics state (nametables, palette, OAM, scroll, control) and
 * rasterizes a frame to a 256x240 RGB buffer. No windowing/audio library: the
 * output is a plain pixel array, so it can be diffed/saved headlessly.
 */
#ifndef LOTW_PPU_H
#define LOTW_PPU_H
#include "nes.h"

#define PPU_W 256
#define PPU_H 240

/* CHR pattern data (tile bitmaps). The cartridge has up to 64 KiB of CHR-ROM
 * banked into the PPU's $0000-$1FFF pattern space by the MMC3. Load it once. */
void ppu_load_chr(const u8 *chr, unsigned len);
void ppu_load_prg(const u8 *prg, unsigned len);
void ppu_map_prg(u16 cpu_base, u8 bank8k);     /* map an 8KiB PRG bank to $8000/$A000 */
void ppu_set_vblank(int on);
void ppu_set_buttons(u8 b);
void ppu_set_sprite0(int on);

/* Reset PPU state (call once at startup). */
void ppu_reset(void);

/* Rasterize the current PPU state into `out` (PPU_W*PPU_H*3 bytes, RGB). */
void ppu_render(u8 *out);

/* Render one 16-wide tile sheet of a pattern table to `out` for verification:
 * `which` selects the $0000 (0) or $1000 (1) half; uses a fixed grayscale.
 * out must be 128*128*3 bytes. */
void ppu_render_statusbar(u8 *out, int rows);
void ppu_debug_tilesheet(int which, u8 *out);

/* --- direct state access (for tests / the frame driver) --- */
extern u8 ppu_vram[0x800];     /* 2 KiB nametable RAM (NT0 at 0, NT1 at 0x400) */
extern u8 ppu_pal[0x20];       /* palette RAM ($3F00-$3F1F) */
extern u8 ppu_oam[0x100];      /* sprite memory (64 * 4) */
extern u8 ppu_ctrl, ppu_mask;  /* $2000 / $2001 shadows */
extern u8 ppu_scroll_x, ppu_scroll_y;

/* Write a P6 PPM image (RGB) to a file. Returns 0 on success. */
int ppm_write(const char *path, const u8 *rgb, int w, int h);

#endif /* LOTW_PPU_H */
