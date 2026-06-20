/* Playable SDL3 front-end for the decompiled Legacy of the Wizard — REAL BOOT.
 *
 * Runs the actual power-on path (reset -> main_init -> title -> main_loop) on a
 * ucontext coroutine: every vblank-wait yields one frame here, where we fire the
 * NMI, rasterize the software PPU to the window, queue APU audio, and sample
 * keyboard/gamepad input. SDL3 gives native Steam Controller / HIDAPI gamepad
 * support (the SDL2 build only saw Steam Input devices Steam wouldn't feed).
 *
 *   build:  cmake --build build/cmake   (target: play, links sdl3)
 *   run:    ./play rom/lotw.nes
 *   keys:   arrows = D-pad, Z = A, X = B, Enter = Start, RShift = Select, Esc = quit
 *   any SDL gamepad (incl. Steam Controller in Gamepad mode) is used if present.
 *
 *   headless self-test:  ./play rom/lotw.nes <max_frames> [auto]
 */
#include <SDL3/SDL.h>
#include "ppu.h"
#include "apu.h"
#include "regs.h"
#include <stdio.h>
#include <string.h>
#include <stdlib.h>
#include <ucontext.h>

u8 NES_MEM[0x10000];
extern void (*apu_write_hook)(u16, u8);

void reset(Regs*);
void nmi_handler(Regs*);

#define SPF (APU_SR / 60)

/* ---- the game runs on its own stack; vblank-waits swap back to the SDL loop ---- */
static Regs        g_regs;
static ucontext_t  g_main_ctx, g_game_ctx;
static int         g_done;
static char        g_stack[16 * 1024 * 1024];

static void frame_yield(Regs *r)
{
    Regs save = *r;
    swapcontext(&g_game_ctx, &g_main_ctx);
    *r = save;
}

static void game_entry(void)
{
    reset(&g_regs);
    g_done = 1;
    swapcontext(&g_game_ctx, &g_main_ctx);
}

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb"); if (!f) { perror(path); exit(1); }
    static u8 rom[1 << 20]; size_t n = fread(rom, 1, sizeof rom, f); fclose(f); (void)n;
    unsigned prg = rom[4] * 16384u, chr = rom[5] * 8192u;
    ppu_load_prg(rom + 16, prg);
    ppu_load_chr(rom + 16 + prg, chr);
    ppu_reset(); apu_reset(); apu_write_hook = apu_write;
    memcpy(&NES_MEM[0xC000], rom + 16 + (prg - 0x4000), 0x4000);
    ppu_map_prg(0x8000, 12);
    ppu_map_prg(0xA000, 13);
    ppu_set_vblank(1);
}

int main(int argc, char **argv)
{
    const char *path = argc > 1 ? argv[1] : "rom/lotw.nes";
    int  max_frames  = argc > 2 ? atoi(argv[2]) : 0;
    int  autostart   = argc > 3 && !strcmp(argv[3], "auto");

    /* SDL3's HIDAPI Steam driver reads the controller directly — but if Steam is
     * running it owns the device, so set LOTW_HIDAPI_STEAM=0 to instead use the
     * virtual/evdev gamepad (works when launched via Steam, or with hid-steam). */
    { const char *h = SDL_getenv("LOTW_HIDAPI_STEAM");
      SDL_SetHint(SDL_HINT_JOYSTICK_HIDAPI_STEAM, h ? h : "1"); }
    if (!SDL_Init(SDL_INIT_VIDEO | SDL_INIT_AUDIO | SDL_INIT_GAMEPAD)) {
        fprintf(stderr, "SDL_Init: %s\n", SDL_GetError()); return 1;
    }
    SDL_Window *win = SDL_CreateWindow("Legacy of the Wizard (decomp)",
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
    {   /* open the first connected gamepad */
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
    /* also open the raw joystick so we can see input even if the gamepad layer is mute */
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

    getcontext(&g_game_ctx);
    g_game_ctx.uc_stack.ss_sp   = g_stack;
    g_game_ctx.uc_stack.ss_size = sizeof g_stack;
    g_game_ctx.uc_link          = &g_main_ctx;
    makecontext(&g_game_ctx, game_entry, 0);
    nes_vblank_wait = frame_yield;

    /* Prime reset/main until the game reaches its first frame wait. Each host
     * frame then injects NMI before resuming the game, so tight polling loops are
     * interrupted through the same scheduler order as the replay/lockstep paths. */
    swapcontext(&g_main_ctx, &g_game_ctx);
    if (g_done) {
        fprintf(stderr, "game loop returned during boot\n");
        return 1;
    }

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
            /* --- input diagnostics: show whatever the device actually sends --- */
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
        if (k[SDL_SCANCODE_RETURN])b |= 0x08;   /* Start */
        if (k[SDL_SCANCODE_RSHIFT])b |= 0x04;   /* Select */
        if (k[SDL_SCANCODE_X])     b |= 0x02;   /* B */
        if (k[SDL_SCANCODE_Z])     b |= 0x01;   /* A */
        if (pad) {
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_RIGHT)) b |= 0x80;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_LEFT))  b |= 0x40;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_DOWN))  b |= 0x20;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_DPAD_UP))    b |= 0x10;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_START))      b |= 0x08;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_BACK))       b |= 0x04;
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_EAST))       b |= 0x02;  /* B */
            if (SDL_GetGamepadButton(pad, SDL_GAMEPAD_BUTTON_SOUTH))      b |= 0x01;  /* A */
            int lx = SDL_GetGamepadAxis(pad, SDL_GAMEPAD_AXIS_LEFTX);
            int ly = SDL_GetGamepadAxis(pad, SDL_GAMEPAD_AXIS_LEFTY);
            if (lx >  8000) b |= 0x80; if (lx < -8000) b |= 0x40;
            if (ly >  8000) b |= 0x20; if (ly < -8000) b |= 0x10;
        }
        if (autostart) {
            if (frames >= 150 && frames < 168) b |= 0x08;        /* Start -> house */
            else if (frames >= 200) {                            /* sweep to find+select a member, then exit */
                int seg = (frames - 200) / 45;
                switch (seg % 6) {
                    case 0: b |= 0x40; break;   /* left  */
                    case 1: b |= 0x80; break;   /* right */
                    case 2: b |= 0x10; break;   /* up    */
                    case 3: b |= 0x20; break;   /* down  */
                    case 4: if (((frames-200)%45) < 6) b |= 0x01; break;  /* A tap */
                    case 5: if (((frames-200)%45) < 6) b |= 0x08; break;  /* Start tap */
                }
            }
        }
        ppu_set_buttons(b);

        nmi_handler(&g_regs);
        swapcontext(&g_main_ctx, &g_game_ctx);
        if (g_done) { fprintf(stderr, "game loop returned at frame %d\n", frames); break; }

        if (SDL_getenv("LOTW_TRACE")) {   /* opt-in: log transitions + dump frames */
            static u8 pc=0xFF, px=0xFF, py2=0xFF, ps=0xFF; static int dn=0;
            u8 c=NES_MEM[0x40], mx=NES_MEM[0x47], my=NES_MEM[0x48], so=NES_MEM[0x8E];
            if (c!=pc || mx!=px || my!=py2 || so!=ps) {
                fprintf(stderr, "f%-5d char$40=%02X map $47=%02X $48=%02X song$8E=%02X health$58=%02X "
                        "banks$30=%02X$31=%02X tiletbl$79=%02X%02X\n",
                        frames, c, mx, my, so, NES_MEM[0x58], NES_MEM[0x30], NES_MEM[0x31],
                        NES_MEM[0x7A], NES_MEM[0x79]);
                pc=c; px=mx; py2=my; ps=so;
            }
            /* snapshot once a real character is active (post-selection) — catches the
             * overworld after it builds, skipping the attract loop (char 6). */
            if (NES_MEM[0x40] < 6 && frames % 20 == 0 && dn < 40) {
                char nm[80]; snprintf(nm, sizeof nm, "build/live_%02d_f%05d_m%02X%02X_s%02X.ppm",
                                      dn++, frames, NES_MEM[0x47], NES_MEM[0x48], NES_MEM[0x8E]);
                ppm_write(nm, fb, PPU_W, PPU_H);
            }
            /* one-shot palette/nametable dump at the demo overworld (0,5), settled */
            static int paldumped = 0;
            if (!paldumped && NES_MEM[0x40] < 6 && NES_MEM[0x47]==0 && NES_MEM[0x48]==5 && frames > 1380) {
                paldumped = 1;
                fprintf(stderr, "MINE palbuf $0180:"); for (int i=0;i<32;i++) fprintf(stderr," %02X",NES_MEM[0x180+i]);
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
            fprintf(stderr, "f%-4d  titleTmr$8C=%02X health$58=%02X char$40=%02X "
                    "px$44=%02X py$45=%02X in$20=%02X\n",
                    frames, NES_MEM[0x8C], NES_MEM[0x58], NES_MEM[0x40],
                    NES_MEM[0x44], NES_MEM[0x45], NES_MEM[0x20]);

        if (max_frames && frames == max_frames - 1) {
            fprintf(stderr, "map $47=%02X $48=%02X  CHR $2A-$2F:",
                    NES_MEM[0x47], NES_MEM[0x48]);
            for (int i = 0x2A; i <= 0x2F; i++) fprintf(stderr, " %02X", NES_MEM[i]);
            fprintf(stderr, "\nNT0 row0:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[i]);
            fprintf(stderr, "\nphysNT0 $0000 row0:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[i]);
            fprintf(stderr, "\nphysNT0 $0000 row2:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[64 + i]);
            fprintf(stderr, "\nphysNT1 $0400 row0:");
            for (int i = 0; i < 32; i++) fprintf(stderr, " %02X", ppu_vram[0x400 + i]);
            fprintf(stderr, "\nphysNT1 $0400 row2:");
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
    SDL_Quit();
    return 0;
}
