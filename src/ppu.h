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
#include "regs.h"

#define PPU_W 256
#define PPU_H 240

/* Frame-sync hook (LOTW_SHIM builds). The decompiled engine blocks on the NMI at
 * each vblank-wait ($36 spin / queue_ppu_job_and_wait / the C135 commit). This
 * pointer is what those sites call. The default fires one NMI inline (so the
 * headless drivers advance a frame per wait); a coroutine front-end overrides it
 * to yield one frame to its window/audio/input loop, then resume. */
extern void (*nes_vblank_wait)(Regs *r);

/* Physically map the R6/R7 PRG banks from the $30/$31 shadow bytes into NES_MEM
 * ($8000/$A000). The far-call helpers change the shadows but, unlike the hardware
 * dispatcher, don't switch the mapper — call this so far-called code reads its
 * bank's data (palettes/tiles/tables) instead of a stale bank's. */
void nes_prg_map_shadow(void);

/* CHR pattern data (tile bitmaps). The cartridge has up to 64 KiB of CHR-ROM
 * banked into the PPU's $0000-$1FFF pattern space by the MMC3. Load it once. */
void ppu_load_chr(const u8 *chr, unsigned len);
void ppu_load_prg(const u8 *prg, unsigned len);
void ppu_map_prg(u16 cpu_base, u8 bank8k);     /* map an 8KiB PRG bank to $8000/$A000 */
void ppu_set_vblank(int on);
void ppu_set_buttons(u8 b);
void ppu_set_sprite0(int on);

/* Optional per-read input hook (lockstep co-sim): when non-NULL, read_controllers
 * calls it at entry to fetch the next controller byte, so input is indexed by
 * controller-READ count (content-aligned) rather than wall-frame — making the
 * co-sim robust to frame-timing slips between the port and the real ROM. NULL =>
 * use whatever ppu_set_buttons last latched (the interactive/SDL path). */
extern u8 (*nes_next_input)(void);

/* Frame-yield for faithful "wait for button" spin-loops (E00F/E27D/AE11/E424/...).
 * On real hardware those loops re-read the LIVE controller every iteration, so the
 * player's press/release ends them. In the interactive/SDL build the $4016 latch
 * only refreshes when the coroutine yields a frame, so a non-yielding poll spins
 * forever on stale input (a hang on this effectively-infinite-speed CPU). This
 * yields one frame so the latch refreshes — BUT only in the live-input build: in
 * the lockstep co-sim (nes_next_input set) input advances per read and the loops
 * must keep matching the real ROM's many-reads-per-frame, so it's a no-op there. */
void nes_input_poll_yield(Regs *r);

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
