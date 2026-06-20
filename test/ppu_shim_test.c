








#include "ppu.h"
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

u8 LOTW_MEMORY[0x10000];

int main(int argc, char **argv)
{
    const char *path = argc > 1 ? argv[1] : "rom/lotw.nes";
    FILE *f = fopen(path, "rb");
    if (!f) { perror("rom"); return 1; }
    static u8 rom[1 << 20];
    size_t n = fread(rom, 1, sizeof rom, f);
    fclose(f);
    if (n < 16 || rom[0] != 'N' || rom[1] != 'E' || rom[2] != 'S') {
        fprintf(stderr, "not an iNES file\n"); return 1;
    }
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    unsigned chr_off = 16 + prg;
    printf("iNES: PRG=%uKB CHR=%uKB  CHR at file offset 0x%X\n", prg/1024, chr/1024, chr_off);

    ppu_reset();
    ppu_load_chr(rom + chr_off, chr);


    static u8 sheet[128 * 128 * 3];
    ppu_debug_tilesheet(0, sheet); ppm_write("build/ppu_tiles_0.ppm", sheet, 128, 128);
    ppu_debug_tilesheet(1, sheet); ppm_write("build/ppu_tiles_1.ppm", sheet, 128, 128);
    printf("wrote build/ppu_tiles_0.ppm, build/ppu_tiles_1.ppm (128x128)\n");




    for (int ty = 0; ty < 30; ty++)
        for (int tx = 0; tx < 32; tx++)
            ppu_vram[ty * 32 + tx] = (u8)((ty * 32 + tx) & 0xFF);
    for (int i = 0; i < 64; i++)
        ppu_vram[0x3C0 + i] = (u8)(0x00 | (0x55 * (i & 3)));

    static const u8 testpal[0x20] = {
        0x0F, 0x00,0x10,0x30,   0x0F, 0x06,0x16,0x26,   0x0F, 0x09,0x19,0x29,   0x0F, 0x01,0x11,0x21,
        0x0F, 0x12,0x22,0x32,   0x0F, 0x14,0x24,0x34,   0x0F, 0x1A,0x2A,0x3A,   0x0F, 0x05,0x15,0x25,
    };
    memcpy(ppu_pal, testpal, sizeof testpal);


    memset(ppu_oam, 0xFF, sizeof ppu_oam);
    for (int i = 0; i < 8; i++) {
        u8 *o = ppu_oam + i * 4;
        o[0] = 112; o[1] = (u8)(0x10 + i); o[2] = (u8)(i & 3); o[3] = (u8)(40 + i * 24);
    }

    ppu_ctrl = 0x00;
    ppu_mask = 0x18;
    ppu_scroll_x = ppu_scroll_y = 0;

    static u8 frame[PPU_W * PPU_H * 3];
    ppu_render(frame);
    ppm_write("build/ppu_frame.ppm", frame, PPU_W, PPU_H);
    printf("wrote build/ppu_frame.ppm (%dx%d)\n", PPU_W, PPU_H);


    long lit = 0;
    for (int i = 0; i < PPU_W * PPU_H; i++)
        if (frame[i*3] || frame[i*3+1] || frame[i*3+2]) lit++;
    printf("frame: %ld/%d non-black pixels\n", lit, PPU_W * PPU_H);
    return 0;
}
