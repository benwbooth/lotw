



#include "ppu.h"
#include "apu.h"
#include "routine_context.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
u8 LOTW_MEMORY[0x10000];
extern void (*apu_write_hook)(u16,u8);
void ram_state_init(RoutineContext*); void farcall_bank_0C0D_seed(RoutineContext*);
void scene_assemble(RoutineContext*); void game_update(RoutineContext*); void song_init(RoutineContext*); void sound_tick(RoutineContext*);
void routine_0081(RoutineContext*); void routine_0060(RoutineContext*); void routine_0076(RoutineContext*); void routine_0131(RoutineContext*);
void read_controllers(RoutineContext*);
void routine_0266(RoutineContext*); void routine_0212(RoutineContext*); void routine_0271(RoutineContext*);
void routine_0059(RoutineContext*); void routine_0061(RoutineContext*); void routine_0063(RoutineContext*); void routine_0058(RoutineContext*);

int main(int argc, char **argv){
    const char *path = argc>1?argv[1]:"rom/lotw.nes";
    u8 btn = argc>2 ? (u8)strtol(argv[2],0,16) : 0x80;
    int N = argc>3 ? atoi(argv[3]) : 48;
    FILE *f=fopen(path,"rb"); if(!f){perror("rom");return 1;}
    static u8 rom[1<<20]; size_t n=fread(rom,1,sizeof rom,f); fclose(f);
    unsigned prg=rom[4]*16384u, chr=rom[5]*8192u;
    ppu_load_prg(rom+16,prg); ppu_load_chr(rom+16+prg,chr); ppu_reset(); apu_reset();
    apu_write_hook=apu_write;
    memcpy(&LOTW_MEMORY[0xC000], rom+16+(prg-0x4000), 0x4000);
    ppu_map_prg(0x8000,12); ppu_map_prg(0xA000,13); ppu_set_vblank(1);
    RoutineContext r; memset(&r,0,sizeof r);
    ram_state_init(&r); farcall_bank_0C0D_seed(&r);

    GAME_MEM8(0x8E)=0x09; GAME_MEM8(0x41)=0xFF; GAME_MEM8(0x39)=0xC5; GAME_MEM8(0x3A)=0x17; GAME_MEM8(0x3B)=0x42;
    GAME_MEM8(0x47)=0x01; GAME_MEM8(0x48)=0x05; GAME_MEM8(0x40)=0x00;
    for(int i=0;i<4;i++) GAME_MEM8(0x5C+i)=LOTW_MEMORY[0xFFA7+i];
    GAME_MEM8(0x51)=LOTW_MEMORY[0xB0AC]; GAME_MEM8(0x55)=0; GAME_MEM8(0x2C)=0x38;
    GAME_MEM8(0x2E)=0x3E; GAME_MEM8(0x2F)=0x20; GAME_MEM8(0x56)=0x0D; GAME_MEM8(0x57)=0; GAME_MEM8(0x42)=1;
    GAME_MEM8(0x58)=0x64; GAME_MEM8(0x59)=0x64; GAME_MEM8(0xEB)=0;
    GAME_MEM8(0x44)=0x20; GAME_MEM8(0x45)=0x80; GAME_MEM8(0x43)=0; GAME_MEM8(0x7C)=0x18; GAME_MEM8(0x7B)=0;
    scene_assemble(&r);
    GAME_MEM8(0x7C)=0x10; routine_0081(&r); routine_0060(&r);
    GAME_MEM8(0x7C)=0x20; routine_0081(&r); routine_0060(&r);
    routine_0076(&r); routine_0131(&r);
    GAME_MEM8(0x8E)=0x00; GAME_MEM8(0x8D)=0; song_init(&r);
    static u8 frame[PPU_W*PPU_H*3];
    for(int fr=0; fr<N; fr++){
        ppu_set_buttons(btn);
        GAME_MEM8(0x36)=1;
        read_controllers(&r);
        game_update(&r);
        routine_0266(&r); routine_0212(&r); routine_0271(&r); routine_0059(&r);
        routine_0061(&r); routine_0063(&r); routine_0058(&r);
        sound_tick(&r);
        if(!(ppu_mask&0x18)) ppu_mask=0x1E;
        ppu_ctrl|=0x08;
        ppu_render(frame);
        if(LOTW_MEMORY[0x29]) ppu_render_statusbar(frame,40);
        char p[64]; snprintf(p,sizeof p,"build/anim/f%03d.ppm",fr);
        ppm_write(p,frame,PPU_W,PPU_H);
        fprintf(stderr,"frame %d: player x_tile=%02X y=%02X input20=%02X\n",fr,GAME_MEM8(0x44),GAME_MEM8(0x45),GAME_MEM8(0x20));
    }
    return 0;
}
