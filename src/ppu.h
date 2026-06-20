








#ifndef LOTW_PPU_H
#define LOTW_PPU_H
#include "platform.h"
#include "routine_context.h"

#ifdef __cplusplus
extern "C" {
#endif

#define PPU_W 256
#define PPU_H 240






extern void (*lotw_frame_wait_hook)(RoutineContext *r);
void lotw_frame_wait(RoutineContext *r);
void vblank_commit(RoutineContext *r);





void lotw_prg_map_shadow(void);



void ppu_load_chr(const u8 *chr, unsigned len);
void ppu_load_prg(const u8 *prg, unsigned len);
void ppu_map_prg(u16 cpu_base, u8 bank8k);
void ppu_set_vblank(int on);
void ppu_set_buttons(u8 b);
void ppu_set_sprite0(int on);






extern u8 (*lotw_next_input)(void);


void ppu_reset(void);


void ppu_render(u8 *out);




void ppu_render_statusbar(u8 *out, int rows);
void ppu_debug_tilesheet(int which, u8 *out);


extern u8 ppu_vram[0x800];
extern u8 ppu_pal[0x20];
extern u8 ppu_oam[0x100];
extern u8 ppu_ctrl, ppu_mask;
extern u8 ppu_scroll_x, ppu_scroll_y;
int ppu_mirror_dbg(void);


int ppm_write(const char *path, const u8 *rgb, int w, int h);

#ifdef __cplusplus
}
#endif

#endif
