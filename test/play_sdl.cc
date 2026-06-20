













#include <SDL3/SDL.h>
#include "ppu.h"
#include "apu.h"
#include "routine_context.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include "native/frame_runner_c.h"

extern "C" {
u8 LOTW_MEMORY[0x10000];
extern void (*apu_write_hook)(u16, u8);
void reset(RoutineContext*);
void vblank_commit(RoutineContext*);
}

#define SPF (APU_SR / 60)

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb"); if (!f) { perror(path); exit(1); }
    static u8 rom[1 << 20]; size_t n = fread(rom, 1, sizeof rom, f); fclose(f); (void)n;
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    ppu_load_prg(rom + 16, prg);
    ppu_load_chr(rom + 16 + prg, chr);
    ppu_reset(); apu_reset(); apu_write_hook = apu_write;
    memcpy(&LOTW_MEMORY[0xC000], rom + 16 + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12);
    ppu_map_prg(0xA000, 13);
    ppu_set_vblank(1);
}

int main(int argc, char **argv)
{
    const char *path = argc > 1 ? argv[1] : "rom/lotw.nes";
    int  max_frames  = argc > 2 ? atoi(argv[2]) : 0;
    int  autostart   = argc > 3 && !strcmp(argv[3], "auto");




    { const char *h = SDL_getenv("LOTW_HIDAPI_STEAM");
      SDL_SetHint(SDL_HINT_JOYSTICK_HIDAPI_STEAM, h ? h : "1"); }
    if (!SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMEPAD)) {
        fprintf(stderr, "SDL_Init: %s\n", SDL_GetError()); return 1;
    }
    SDL_Window *win = SDL_CreateWindow("Legacy of the Wizard",
        PPU_W * 3, PPU_H * 3, SDL_WINDOW_RESIZABLE);
    SDL_Renderer *ren = SDL_CreateRenderer(win, NULL);
    SDL_SetRenderVSync(ren, 1);
    SDL_SetRenderLogicalPresentation(ren, PPU_W, PPU_H, SDL_LOGICAL_PRESENTATION_LETTERBOX);
    SDL_Texture *tex = SDL_CreateTexture(ren, SDL_PIXELFORMAT_RGB24,
        SDL_TEXTUREACCESS_STREAMING, PPU_W, PPU_H);
    SDL_SetTextureScaleMode(tex, SDL_SCALEMODE_NEAREST);

    SDL_AudioSpec spec = { SDL_AUDIO_S16, 1, APU_SR };
    SDL_AudioStream *audio_stream =
        SDL_OpenAudioDeviceStream(SDL_AUDIO_DEVICE_DEFAULT_PLAYBACK, &spec, NULL, NULL);
    if (audio_stream) {
        bool ok = SDL_ResumeAudioStreamDevice(audio_stream);
        SDL_AudioDeviceID dev = SDL_GetAudioStreamDevice(audio_stream);
        fprintf(stderr, "audio: stream opened, resume=%d devid=%u paused=%d  (driver=%s)\n",
                ok, dev, SDL_AudioDevicePaused(dev), SDL_GetCurrentAudioDriver());
    } else {
        fprintf(stderr, "audio: OPEN FAILED: %s\n", SDL_GetError());
    }

    SDL_Gamepad *pad = NULL;
    {
        int count = 0;
        SDL_JoystickID *ids = SDL_GetGamepads(&count);
        fprintf(stderr, "%d gamepad(s) detected\n", count);
        if (ids) {
            for (int i = 0; i < count; i++) {
                pad = SDL_OpenGamepad(ids[i]);
                if (pad) {
                    fprintf(stderr, "gamepad: %s  type=%d path=%s\n",
                            SDL_GetGamepadName(pad), SDL_GetGamepadType(pad),
                            SDL_GetGamepadPath(pad) ? SDL_GetGamepadPath(pad) : "?");
                    break;
                }
            }
            SDL_free(ids);
        }
    }

    SDL_Joystick *joy = NULL;
    { int jc = 0; SDL_JoystickID *jids = SDL_GetJoysticks(&jc);
      if (jids && jc > 0) { joy = SDL_OpenJoystick(jids[0]);
        if (joy) fprintf(stderr, "raw joystick: %s  %d btn %d axes %d hats\n",
            SDL_GetJoystickName(joy), SDL_GetNumJoystickButtons(joy),
            SDL_GetNumJoystickAxes(joy), SDL_GetNumJoystickHats(joy)); }
      if (jids) SDL_free(jids); }
    SDL_SetGamepadEventsEnabled(true);
    SDL_SetJoystickEventsEnabled(true);

    load_rom(path);

    LotwFrameRunner *runner = lotw_frame_runner_create(reset);
    if (!runner) {
        fprintf(stderr, "failed to create frame runner\n");
        return 1;
    }



    if (!lotw_frame_runner_start(runner)) {
        fprintf(stderr, "game loop returned during boot\n");
        lotw_frame_runner_destroy(runner);
        return 1;
    }
    RoutineContext *regs = lotw_frame_runner_context(runner);

    static u8 fb[PPU_W * PPU_H * 3];
    static short audio[SPF];
    int running = 1, frames = 0;
    Uint64 next = SDL_GetTicks();
    while (running) {
        SDL_Event e;
        while (SDL_PollEvent(&e)) {
            if (e.type == SDL_EVENT_QUIT ||
                (e.type == SDL_EVENT_KEY_DOWN && e.key.key == SDLK_ESCAPE))
                running = 0;
            else if (e.type == SDL_EVENT_GAMEPAD_ADDED && !pad) {
                pad = SDL_OpenGamepad(e.gdevice.which);
                if (pad) fprintf(stderr, "gamepad connected: %s\n", SDL_GetGamepadName(pad));
            }

            else if (e.type == SDL_EVENT_GAMEPAD_BUTTON_DOWN)
                fprintf(stderr, "[gp] button %d down\n", e.gbutton.button);
            else if (e.type == SDL_EVENT_GAMEPAD_AXIS_MOTION && SDL_abs(e.gaxis.value) > 12000)
                fprintf(stderr, "[gp] axis %d = %d\n", e.gaxis.axis, e.gaxis.value);
            else if (e.type == SDL_EVENT_JOYSTICK_BUTTON_DOWN)
                fprintf(stderr, "[joy] button %d down\n", e.jbutton.button);
            else if (e.type == SDL_EVENT_JOYSTICK_HAT_MOTION)
                fprintf(stderr, "[joy] hat %d = %d\n", e.jhat.hat, e.jhat.value);
            else if (e.type == SDL_EVENT_JOYSTICK_AXIS_MOTION && SDL_abs(e.jaxis.value) > 12000)
                fprintf(stderr, "[joy] axis %d = %d\n", e.jaxis.axis, e.jaxis.value);
        }

        const bool *k = SDL_GetKeyboardState(NULL);
        u8 b = 0;
        if (k[SDL_SCANCODE_RIGHT]) b |= 0x80;
        if (k[SDL_SCANCODE_LEFT])  b |= 0x40;
        if (k[SDL_SCANCODE_DOWN])  b |= 0x20;
        if (k[SDL_SCANCODE_UP])    b |= 0x10;
        if (k[SDL_SCANCODE_RETURN])b |= 0x08;
        if (k[SDL_SCANCODE_RSHIFT])b |= 0x04;
        if (k[SDL_SCANCODE_X])     b |= 0x02;
        if (k[SDL_SCANCODE_Z])     b |= 0x01;
        if (pad) {
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_RIGHT)) b |= 0x80;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_LEFT))  b |= 0x40;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_DOWN))  b |= 0x20;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_UP))    b |= 0x10;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_START))      b |= 0x08;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_BACK))       b |= 0x04;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_EAST))       b |= 0x02;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_SOUTH))      b |= 0x01;
            int lx = SDL_GetGamepadAxis(pad, SDL_GAMEPAD_AXIS_LEFTX);
            int ly = SDL_GetGamepadAxis(pad, SDL_GAMEPAD_AXIS_LEFTY);
            if (lx >  8000) b |= 0x80; if (lx < -8000) b |= 0x40;
            if (ly >  8000) b |= 0x20; if (ly < -8000) b |= 0x10;
        }
        if (autostart) {
            if (frames >= 150 && frames < 168) b |= 0x08;
            else if (frames >= 200) {
                int seg = (frames - 200) / 45;
                switch (seg % 6) {
                    case 0: b |= 0x40; break;
                    case 1: b |= 0x80; break;
                    case 2: b |= 0x10; break;
                    case 3: b |= 0x20; break;
                    case 4: if (((frames-200)%45) < 6) b |= 0x01; break;
                    case 5: if (((frames-200)%45) < 6) b |= 0x08; break;
                }
            }
        }
        ppu_set_buttons(b);

        vblank_commit(regs);
        if (!lotw_frame_runner_resume_until_wait(runner)) {
            fprintf(stderr, "game loop returned at frame %d\n", frames);
            break;
        }

        if (SDL_getenv("LOTW_TRACE")) {
            static u8 pc=0xFF, px=0xFF, py2=0xFF, ps=0xFF; static int dn=0;
            u8 c=LOTW_MEMORY[0x40], mx=LOTW_MEMORY[0x47], my=LOTW_MEMORY[0x48], so=LOTW_MEMORY[0x8E];
            if (c!=pc || mx!=px || my!=py2 || so!=ps) {
                fprintf(stderr, "f%-5d char40=%02X map47=%02X map48=%02X song8E=%02X health58=%02X "
                        "banks30=%02X banks31=%02X tiletbl79=%02X%02X\n",
                        frames, c, mx, my, so, LOTW_MEMORY[0x58], LOTW_MEMORY[0x30], LOTW_MEMORY[0x31],
                        LOTW_MEMORY[0x7A], LOTW_MEMORY[0x79]);
                pc=c; px=mx; py2=my; ps=so;
            }


            if (LOTW_MEMORY[0x40] < 6 && frames % 20 == 0 && dn < 40) {
                char nm[80]; snprintf(nm, sizeof nm, "build/live_%02d_f%05d_m%02X%02X_s%02X.ppm",
                                      dn++, frames, LOTW_MEMORY[0x47], LOTW_MEMORY[0x48], LOTW_MEMORY[0x8E]);
                ppm_write(nm, fb, PPU_W, PPU_H);
            }

            static int paldumped = 0;
            if (!paldumped && LOTW_MEMORY[0x40] < 6 && LOTW_MEMORY[0x47]==0 && LOTW_MEMORY[0x48]==5 && frames > 1380) {
                paldumped = 1;
                fprintf(stderr, "MINE palbuf 0x0180:"); for (int i=0;i<32;i++) fprintf(stderr," %02X",LOTW_MEMORY[0x180+i]);
                fprintf(stderr, "\nMINE ppu_pal:     "); for (int i=0;i<32;i++) fprintf(stderr," %02X",ppu_pal[i]);
                fprintf(stderr, "\nMINE NT0 row5:    "); for (int i=0;i<32;i++) fprintf(stderr," %02X",ppu_vram[5*32+i]);
                fprintf(stderr, "\n");
            }
        }

        ppu_render(fb);

        SDL_UpdateTexture(tex, NULL, fb, PPU_W * 3);
        SDL_RenderClear(ren);
        SDL_RenderTexture(ren, tex, NULL, NULL);
        SDL_RenderPresent(ren);

        if (audio_stream) {
            apu_frame();
            apu_gen(audio, SPF);
            static int apk = 0; for (int i=0;i<SPF;i++){int v=audio[i]<0?-audio[i]:audio[i]; if(v>apk)apk=v;}
            if (max_frames && frames == max_frames-1) fprintf(stderr, "audio peak=%d (%s)\n", apk, apk>500?"AUDIBLE":"SILENT");
            if (SDL_GetAudioStreamQueued(audio_stream) < (int)(sizeof audio * 4))
                SDL_PutAudioStreamData(audio_stream, audio, sizeof audio);
        }

        if (max_frames && (frames % 30 == 0))
            fprintf(stderr, "f%-4d  titleTmr8C=%02X health58=%02X char40=%02X "
                    "px44=%02X py45=%02X in20=%02X\n",
                    frames, LOTW_MEMORY[0x8C], LOTW_MEMORY[0x58], LOTW_MEMORY[0x40],
                    LOTW_MEMORY[0x44], LOTW_MEMORY[0x45], LOTW_MEMORY[0x20]);

        if (max_frames && frames == max_frames - 1) {
            fprintf(stderr, "map47=%02X map48=%02X  CHR 2A-2F:",
                    LOTW_MEMORY[0x47], LOTW_MEMORY[0x48]);
            for (int i = 0x2A; i <= 0x2F; i++) fprintf(stderr, " %02X", LOTW_MEMORY[i]);
            fprintf(stderr, "\nNT0 row0:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[i]);
            fprintf(stderr, "\nphysNT0 0x0000 row0:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[i]);
            fprintf(stderr, "\nphysNT0 0x0000 row2:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[64 + i]);
            fprintf(stderr, "\nphysNT1 0x0400 row0:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[0x400 + i]);
            fprintf(stderr, "\nphysNT1 0x0400 row2:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[0x400 + 64 + i]);
            fprintf(stderr, "\n");
        }

        if (++frames == max_frames) running = 0;
        next += 1000 / 60;
        Uint64 now = SDL_GetTicks();
        if (next > now) SDL_Delay((Uint32)(next - now)); else next = now;
    }
    if (max_frames) ppm_write("build/boot_last.ppm", fb, PPU_W, PPU_H);
    fprintf(stderr, "ran %d frames\n", frames);
    lotw_frame_runner_destroy(runner);
    SDL_Quit();
    return 0;
}
