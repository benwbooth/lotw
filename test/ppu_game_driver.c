









#include "ppu.h"
#include "routine_context.h"
#include <stdio.h>
#include <string.h>

u8 LOTW_MEMORY[0x10000];


void ram_state_init(RoutineContext *r);
void farcall_bank_0C0D_seed(RoutineContext *r);
void scene_assemble(RoutineContext *r);
void game_update(RoutineContext *r);

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


    memcpy(&LOTW_MEMORY[0xC000], PRG + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12);
    ppu_map_prg(0xA000, 13);
    ppu_set_vblank(1);

    RoutineContext r; memset(&r, 0, sizeof r);

    fprintf(stderr, "ram_state_init...\n");   ram_state_init(&r);
    fprintf(stderr, "seed banks...\n");        farcall_bank_0C0D_seed(&r);


    GAME_MEM8(0x46) = 0x00;
    GAME_MEM8(0x7B) = 0x00;
    GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x7C) = 0x30;
    GAME_MEM8(0x44) = 0x3C;
    GAME_MEM8(0x45) = 0xA0;



    GAME_MEM8(0x8E) = 0x09;
    GAME_MEM8(0x41) = 0xFF;
    GAME_MEM8(0x39) = 0xC5; GAME_MEM8(0x3A) = 0x17; GAME_MEM8(0x3B) = 0x42;
    GAME_MEM8(0x47) = 0x01; GAME_MEM8(0x48) = 0x05;
    GAME_MEM8(0x40) = 0x00;
    for (int i = 0; i < 4; i++) GAME_MEM8(0x5C + i) = LOTW_MEMORY[0xFFA7 + i];
    GAME_MEM8(0x51) = LOTW_MEMORY[0xB0AC];
    GAME_MEM8(0x55) = 0x00;
    GAME_MEM8(0x2C) = 0x38;
    GAME_MEM8(0x2E) = 0x3E; GAME_MEM8(0x2F) = 0x20;
    GAME_MEM8(0x56) = 0x0D; GAME_MEM8(0x57) = 0x00; GAME_MEM8(0x42) = 0x01;
    GAME_MEM8(0x58) = 0x64;
    GAME_MEM8(0x59) = 0x64;
    GAME_MEM8(0xEB) = 0x00;

    GAME_MEM8(0x44) = 0x20; GAME_MEM8(0x45) = 0x80; GAME_MEM8(0x43) = 0x00;
    GAME_MEM8(0x7C) = 0x18; GAME_MEM8(0x7B) = 0x00;
    fprintf(stderr, "scene_assemble...\n");    scene_assemble(&r);


    void routine_0081(RoutineContext*); void routine_0060(RoutineContext*);
    fprintf(stderr, "C7B5 (screen layout)...\n");
    GAME_MEM8(0x7C) = 0x10; routine_0081(&r); routine_0060(&r);
    GAME_MEM8(0x7C) = 0x20; routine_0081(&r); routine_0060(&r);
    void routine_0076(RoutineContext*); void routine_0131(RoutineContext*);
    fprintf(stderr, "C57A (status-bar setup)...\n");
    routine_0076(&r); routine_0131(&r);


    void routine_0266(RoutineContext*); void routine_0212(RoutineContext*); void routine_0271(RoutineContext*);
    void routine_0059(RoutineContext*); void routine_0061(RoutineContext*); void routine_0063(RoutineContext*); void routine_0058(RoutineContext*);
    for (int fr = 0; fr < 4; fr++) {
        fprintf(stderr, "frame %d: game_update...", fr);
        GAME_MEM8(0x36) = 0x01;
        game_update(&r);                       fprintf(stderr, " F628");
        routine_0266(&r);                          fprintf(stderr, " E87C");
        routine_0212(&r);                          fprintf(stderr, " F782");
        routine_0271(&r);  routine_0059(&r);
        routine_0061(&r);  routine_0063(&r);  routine_0058(&r);
        fprintf(stderr, " done\n");
    }





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
      fprintf(stderr, "NT0 attr 0x03C0:");
      for (int i = 0; i < 16; i++) fprintf(stderr, " %02X", ppu_vram[0x3C0 + i]);
      fprintf(stderr, "\nNT0 tiles row0:");
      for (int i = 0; i < 16; i++) fprintf(stderr, " %02X", ppu_vram[i]);
      fprintf(stderr, "\nNT0 tiles row8 0x0100:");
      for (int i = 0; i < 16; i++) fprintf(stderr, " %02X", ppu_vram[0x100 + i]);
      fprintf(stderr, "\n"); }
    { int act = 0;
      fprintf(stderr, "active OAM sprites (Y<0xF0):");
      for (int i = 0; i < 64; i++) {
          u8 *o = ppu_oam + i*4;
          if (o[0] < 0xEF) { act++; if (act <= 10) fprintf(stderr, " [%d:y%d t%02X a%02X x%d]", i,o[0],o[1],o[2],o[3]); }
      }
      fprintf(stderr, "  total=%d\n", act);
      fprintf(stderr, "HUD bufs 0x0140:"); for(int i=0;i<8;i++) fprintf(stderr," %02X",LOTW_MEMORY[0x140+i]);
      fprintf(stderr, "  CHR win:"); extern int ppu_chr_win_dbg(int); for(int i=0;i<8;i++) fprintf(stderr," %d", ppu_chr_win_dbg(i));
      fprintf(stderr, "\n"); }
    { fprintf(stderr, "split flag29=%02X  scroll vars1C=%02X vars1D=%02X vars1E=%02X\n",
              LOTW_MEMORY[0x29], LOTW_MEMORY[0x1C], LOTW_MEMORY[0x1D], LOTW_MEMORY[0x1E]);
      fprintf(stderr, "NT0 row24 0x0300, HUD scrollY=0xC4:");
      for (int i = 0; i < 24; i++) fprintf(stderr, " %02X", ppu_vram[0x300 + i]);
      fprintf(stderr, "\n"); }

    if (!(ppu_mask & 0x18)) ppu_mask = 0x1E;
    ppu_ctrl |= 0x08;
    static u8 frame[PPU_W * PPU_H * 3];
    ppu_render(frame);
    if (LOTW_MEMORY[0x29]) { void ppu_render_statusbar(u8*,int); ppu_render_statusbar(frame, 40); }
    ppm_write("build/game_frame.ppm", frame, PPU_W, PPU_H);


    ppu_scroll_x = 0; ppu_scroll_y = 0;
    ppu_render(frame);
    ppm_write("build/game_frame_s0.ppm", frame, PPU_W, PPU_H);

    long lit = 0;
    for (int i = 0; i < PPU_W * PPU_H; i++)
        if (frame[i*3] | frame[i*3+1] | frame[i*3+2]) lit++;
    fprintf(stderr, "rendered build/game_frame.ppm  (%ld/%d non-black px)\n", lit, PPU_W*PPU_H);
    return 0;
}
