/* Headless gameplay animation: warp into a room, then run the real per-frame
 * game loop (read_controllers -> game_update -> sprite/scene update -> render)
 * for N frames while holding a scripted input, capturing each frame to a PPM.
 * Assembled into a GIF -> shows the decompiled game actually being played. */
#include "ppu.h"
#include "apu.h"
#include "regs.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
u8 NES_MEM[0x10000];
extern void (*apu_write_hook)(u16,u8);
void ram_state_init(Regs*); void farcall_bank_0C0D_seed(Regs*);
void scene_assemble(Regs*); void game_update(Regs*); void song_init(Regs*); void sound_tick(Regs*);
void sub_C7B5(Regs*); void sub_C1C7(Regs*); void sub_C57A(Regs*); void sub_D0E5(Regs*);
void read_controllers(Regs*);
void sub_F628(Regs*); void sub_E87C(Regs*); void sub_F782(Regs*);
void sub_C15D(Regs*); void sub_C1D8(Regs*); void sub_C2B1(Regs*); void sub_C135(Regs*);

int main(int argc, char **argv){
    const char *path = argc>1?argv[1]:"rom/lotw.nes";
    u8 btn = argc>2 ? (u8)strtol(argv[2],0,16) : 0x80;   /* scripted button */
    int N = argc>3 ? atoi(argv[3]) : 48;
    FILE *f=fopen(path,"rb"); if(!f){perror("rom");return 1;}
    static u8 rom[1<<20]; size_t n=fread(rom,1,sizeof rom,f); fclose(f);
    unsigned prg=rom[4]*16384u, chr=rom[5]*8192u;
    ppu_load_prg(rom+16,prg); ppu_load_chr(rom+16+prg,chr); ppu_reset(); apu_reset();
    apu_write_hook=apu_write;
    memcpy(&NES_MEM[0xC000], rom+16+(prg-0x4000), 0x4000);
    ppu_map_prg(0x8000,12); ppu_map_prg(0xA000,13); ppu_set_vblank(1);
    Regs r; memset(&r,0,sizeof r);
    ram_state_init(&r); farcall_bank_0C0D_seed(&r);
    /* valid char-select state (as ppu_game_driver) */
    RAM8(0x8E)=0x09; RAM8(0x41)=0xFF; RAM8(0x39)=0xC5; RAM8(0x3A)=0x17; RAM8(0x3B)=0x42;
    RAM8(0x47)=0x01; RAM8(0x48)=0x05; RAM8(0x40)=0x00;
    for(int i=0;i<4;i++) RAM8(0x5C+i)=NES_MEM[0xFFA7+i];
    RAM8(0x51)=NES_MEM[0xB0AC]; RAM8(0x55)=0; RAM8(0x2C)=0x38;
    RAM8(0x2E)=0x3E; RAM8(0x2F)=0x20; RAM8(0x56)=0x0D; RAM8(0x57)=0; RAM8(0x42)=1;
    RAM8(0x58)=0x64; RAM8(0x59)=0x64; RAM8(0xEB)=0;
    RAM8(0x44)=0x20; RAM8(0x45)=0x80; RAM8(0x43)=0; RAM8(0x7C)=0x18; RAM8(0x7B)=0;
    scene_assemble(&r);
    RAM8(0x7C)=0x10; sub_C7B5(&r); sub_C1C7(&r);
    RAM8(0x7C)=0x20; sub_C7B5(&r); sub_C1C7(&r);
    sub_C57A(&r); sub_D0E5(&r);
    RAM8(0x8E)=0x00; RAM8(0x8D)=0; song_init(&r);
    static u8 frame[PPU_W*PPU_H*3];
    for(int fr=0; fr<N; fr++){
        ppu_set_buttons(btn);
        RAM8(0x36)=1;
        read_controllers(&r);
        game_update(&r);
        sub_F628(&r); sub_E87C(&r); sub_F782(&r); sub_C15D(&r);
        sub_C1D8(&r); sub_C2B1(&r); sub_C135(&r);
        sound_tick(&r);
        if(!(ppu_mask&0x18)) ppu_mask=0x1E;
        ppu_ctrl|=0x08;
        ppu_render(frame);
        if(NES_MEM[0x29]) ppu_render_statusbar(frame,40);
        char p[64]; snprintf(p,sizeof p,"build/anim/f%03d.ppm",fr);
        ppm_write(p,frame,PPU_W,PPU_H);
        fprintf(stderr,"frame %d: player x_tile=%02X y=%02X $20=%02X\n",fr,RAM8(0x44),RAM8(0x45),RAM8(0x20));
    }
    return 0;
}
