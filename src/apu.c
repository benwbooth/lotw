/* Software 2A03 APU — see apu.h. Register-driven phase-accumulator synth.
 * CPU clock 1.789773 MHz (NTSC). Channels: pulse1, pulse2 (duty square),
 * triangle (32-step), noise (15-bit LFSR). Length counters clocked per frame. */
#include "apu.h"
#include <stdio.h>
#include <string.h>
#include <math.h>

#define CPU_HZ 1789773.0

static const u8 LEN_TBL[32] = {
    10,254,20,2,40,4,80,6,160,8,60,10,14,12,26,14,
    12,16,24,18,48,20,96,22,192,24,72,26,16,28,32,30
};
static const u16 NOISE_PER[16] = {
    4,8,16,32,64,96,128,160,202,254,380,508,762,1016,2034,4068
};
static const double DUTY[4] = {0.125, 0.25, 0.5, 0.75};

typedef struct {
    int enabled, len, period, vol, duty, halt;
    double phase;
} Pulse;
typedef struct { int enabled, len, period, linear, control; double phase; } Tri;
typedef struct { int enabled, len, period, vol, mode, halt; unsigned lfsr; double phase; } Noise;

static Pulse p1, p2;
static Tri   tr;
static Noise nz;
static u8    status;          /* $4015 enable bits */

void apu_reset(void)
{
    memset(&p1, 0, sizeof p1); memset(&p2, 0, sizeof p2);
    memset(&tr, 0, sizeof tr); memset(&nz, 0, sizeof nz);
    nz.lfsr = 1; status = 0;
}

void apu_write(u16 addr, u8 val)
{
    switch (addr) {
    /* pulse 1 */
    case 0x4000: p1.duty = val >> 6; p1.halt = (val & 0x20) != 0; p1.vol = val & 0x0F; break;
    case 0x4001: break;                                   /* sweep (ignored) */
    case 0x4002: p1.period = (p1.period & 0x700) | val; break;
    case 0x4003: p1.period = (p1.period & 0xFF) | ((val & 7) << 8);
                 p1.len = LEN_TBL[(val >> 3) & 0x1F]; p1.phase = 0; break;
    /* pulse 2 */
    case 0x4004: p2.duty = val >> 6; p2.halt = (val & 0x20) != 0; p2.vol = val & 0x0F; break;
    case 0x4005: break;
    case 0x4006: p2.period = (p2.period & 0x700) | val; break;
    case 0x4007: p2.period = (p2.period & 0xFF) | ((val & 7) << 8);
                 p2.len = LEN_TBL[(val >> 3) & 0x1F]; p2.phase = 0; break;
    /* triangle */
    case 0x4008: tr.control = (val & 0x80) != 0; tr.linear = val & 0x7F; break;
    case 0x400A: tr.period = (tr.period & 0x700) | val; break;
    case 0x400B: tr.period = (tr.period & 0xFF) | ((val & 7) << 8);
                 tr.len = LEN_TBL[(val >> 3) & 0x1F]; break;
    /* noise */
    case 0x400C: nz.halt = (val & 0x20) != 0; nz.vol = val & 0x0F; break;
    case 0x400E: nz.period = NOISE_PER[val & 0x0F]; nz.mode = (val >> 7) & 1; break;
    case 0x400F: nz.len = LEN_TBL[(val >> 3) & 0x1F]; break;
    /* status / enable */
    case 0x4015:
        status = val;
        p1.enabled = (val & 1) != 0;
        p2.enabled = (val & 2) != 0;
        tr.enabled = (val & 4) != 0;
        nz.enabled = (val & 8) != 0;
        if (!(val & 1)) p1.len = 0;
        if (!(val & 2)) p2.len = 0;
        if (!(val & 4)) tr.len = 0;
        if (!(val & 8)) nz.len = 0;
        break;
    default: break;
    }
}

/* per-frame length-counter clock (~60 Hz; the frame sequencer also runs faster,
 * but the engine reloads each frame so this is enough to gate note-off). */
void apu_frame(void)
{
    if (p1.len && !p1.halt) p1.len--;
    if (p2.len && !p2.halt) p2.len--;
    if (tr.len && !tr.control) tr.len--;
    if (nz.len && !nz.halt) nz.len--;
}

static double pulse_out(Pulse *p)
{
    if (!p->enabled || p->len == 0 || p->period < 8 || p->vol == 0) return 0;
    double f = CPU_HZ / (16.0 * (p->period + 1));
    p->phase += f / APU_SR;
    p->phase -= (int)p->phase;
    double lvl = (p->phase < DUTY[p->duty]) ? 1.0 : -1.0;
    return lvl * (p->vol / 15.0);
}

static double tri_out(void)
{
    if (!tr.enabled || tr.len == 0 || tr.linear == 0 || tr.period < 2) return 0;
    double f = CPU_HZ / (32.0 * (tr.period + 1));
    tr.phase += f / APU_SR;
    tr.phase -= (int)tr.phase;
    /* 32-step triangle: 0..1..0 */
    double t = tr.phase < 0.5 ? tr.phase * 2 : (1 - tr.phase) * 2;
    return (t * 2 - 1);
}

static double noise_out(void)
{
    if (!nz.enabled || nz.len == 0 || nz.vol == 0) return 0;
    double f = CPU_HZ / nz.period;
    nz.phase += f / APU_SR;
    while (nz.phase >= 1.0) {
        nz.phase -= 1.0;
        int b0 = nz.lfsr & 1;
        int b1 = nz.mode ? ((nz.lfsr >> 6) & 1) : ((nz.lfsr >> 1) & 1);
        nz.lfsr = (nz.lfsr >> 1) | (((b0 ^ b1) & 1) << 14);
    }
    return ((nz.lfsr & 1) ? -1.0 : 1.0) * (nz.vol / 15.0);
}

void apu_gen(short *out, int n)
{
    for (int i = 0; i < n; i++) {
        double s = pulse_out(&p1) + pulse_out(&p2) + 0.8 * tri_out() + 0.6 * noise_out();
        s *= 0.22;                       /* mix down to avoid clipping */
        if (s > 1) s = 1; if (s < -1) s = -1;
        out[i] = (short)(s * 30000);
    }
}

/* ---- minimal WAV (PCM 16-bit mono) writer ---- */
static void w32(FILE *f, unsigned v){ fputc(v,f);fputc(v>>8,f);fputc(v>>16,f);fputc(v>>24,f); }
static void w16(FILE *f, unsigned v){ fputc(v,f);fputc(v>>8,f); }
int wav_write(const char *path, const short *s, int n, int rate)
{
    FILE *f = fopen(path, "wb"); if (!f) return -1;
    int data = n * 2;
    fwrite("RIFF",1,4,f); w32(f, 36 + data); fwrite("WAVE",1,4,f);
    fwrite("fmt ",1,4,f); w32(f,16); w16(f,1); w16(f,1);
    w32(f,rate); w32(f,rate*2); w16(f,2); w16(f,16);
    fwrite("data",1,4,f); w32(f,data);
    fwrite(s,2,n,f); fclose(f);
    return 0;
}
