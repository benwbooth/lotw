/* Headless software PPU — see ppu.h. Implements the $2000-$2007/$4014 register
 * behavior the decompiled game drives, MMC3 CHR banking + mirroring (tracked off
 * the $8000/$8001/$A000 writes the game also issues through REG_W), and a
 * background + sprite rasterizer to a 256x240 RGB buffer. */
#include "ppu.h"
#include <stdio.h>
#include <string.h>

/* ---- NES master palette (2C02), 64 entries RGB ---- */
static const u8 NES_PAL[64][3] = {
  { 84, 84, 84},{  0, 30,116},{  8, 16,144},{ 48,  0,136},{ 68,  0,100},{ 92,  0, 48},{ 84,  4,  0},{ 60, 24,  0},
  { 32, 42,  0},{  8, 58,  0},{  0, 64,  0},{  0, 60,  0},{  0, 50, 60},{  0,  0,  0},{  0,  0,  0},{  0,  0,  0},
  {152,150,152},{  8, 76,196},{ 48, 50,236},{ 92, 30,228},{136, 20,176},{160, 20,100},{152, 34, 32},{120, 60,  0},
  { 84, 90,  0},{ 40,114,  0},{  8,124,  0},{  0,118, 40},{  0,102,120},{  0,  0,  0},{  0,  0,  0},{  0,  0,  0},
  {236,238,236},{ 76,154,236},{120,124,236},{176, 98,236},{228, 84,236},{236, 88,180},{236,106,100},{212,136, 32},
  {160,170,  0},{116,196,  0},{ 76,208, 32},{ 56,204,108},{ 56,180,204},{ 60, 60, 60},{  0,  0,  0},{  0,  0,  0},
  {236,238,236},{168,204,236},{188,188,236},{212,178,236},{236,174,236},{236,174,212},{236,180,176},{228,196,144},
  {204,210,120},{180,222,120},{168,226,144},{152,226,180},{160,214,228},{160,162,160},{  0,  0,  0},{  0,  0,  0},
};

/* ---- state ---- */
u8 ppu_vram[0x800];
u8 ppu_pal[0x20];
u8 ppu_oam[0x100];
u8 ppu_ctrl, ppu_mask, ppu_scroll_x, ppu_scroll_y;

static u8  s_status;          /* $2002: bit7 vblank, bit6 sprite0, bit5 overflow */
static u8  s_oamaddr;
static u16 s_vaddr;           /* current $2006 VRAM address */
static u8  s_wtoggle;         /* shared $2005/$2006 high/low write latch */
static u8  s_readbuf;         /* $2007 read buffer */

/* CHR: up to 64 KiB, mapped to the PPU $0000-$1FFF pattern space in 8x1 KiB
 * windows per MMC3. */
static u8  s_chr[0x10000];
static unsigned s_chr_len;
static int s_chr_win[8];      /* 1 KiB CHR-bank index for each PPU 1 KiB slot */
static u8  s_mmc3_sel;        /* $8000 last value (bank select + CHR-invert bit7) */
static u8  s_mmc3_bank[8];    /* R0..R7 */
static int s_mirror;          /* 0 = horizontal arrangement, 1 = vertical */

static void recompute_chr(void)
{
    /* MMC3 CHR layout; bit7 of $8000 swaps the $0000 and $1000 halves.
     * R0,R1 are 2 KiB banks (low bit ignored); R2..R5 are 1 KiB banks. */
    int inv = (s_mmc3_sel & 0x80) ? 1 : 0;
    int two[4]; /* the four 1KB slots of the "2KB-bank half" */
    int one[4]; /* the four 1KB slots of the "1KB-bank half" */
    two[0] = (s_mmc3_bank[0] & 0xFE);     two[1] = (s_mmc3_bank[0] & 0xFE) | 1;
    two[2] = (s_mmc3_bank[1] & 0xFE);     two[3] = (s_mmc3_bank[1] & 0xFE) | 1;
    one[0] = s_mmc3_bank[2]; one[1] = s_mmc3_bank[3];
    one[2] = s_mmc3_bank[4]; one[3] = s_mmc3_bank[5];
    if (!inv) { /* $0000=2KB banks, $1000=1KB banks */
        s_chr_win[0]=two[0]; s_chr_win[1]=two[1]; s_chr_win[2]=two[2]; s_chr_win[3]=two[3];
        s_chr_win[4]=one[0]; s_chr_win[5]=one[1]; s_chr_win[6]=one[2]; s_chr_win[7]=one[3];
    } else {
        s_chr_win[0]=one[0]; s_chr_win[1]=one[1]; s_chr_win[2]=one[2]; s_chr_win[3]=one[3];
        s_chr_win[4]=two[0]; s_chr_win[5]=two[1]; s_chr_win[6]=two[2]; s_chr_win[7]=two[3];
    }
}

void ppu_load_chr(const u8 *chr, unsigned len)
{
    if (len > sizeof s_chr) len = sizeof s_chr;
    memcpy(s_chr, chr, len);
    s_chr_len = len;
}

/* PRG-ROM + CPU-side bank mapping: MMC3 swaps 8 KiB PRG banks into $8000/$A000;
 * the shim copies the selected bank into NES_MEM so the game's bank-switched data
 * reads ($8000-$BFFF) hit real data. */
extern u8 NES_MEM[0x10000];
static u8  s_prg[0x40000];
static unsigned s_prg_len;

void ppu_load_prg(const u8 *prg, unsigned len)
{
    if (len > sizeof s_prg) len = sizeof s_prg;
    memcpy(s_prg, prg, len);
    s_prg_len = len;
}

void ppu_map_prg(u16 cpu_base, u8 bank8k)   /* map an 8KiB PRG bank to $8000 or $A000 */
{
    if (!s_prg_len) return;
    unsigned nbanks = s_prg_len / 0x2000;
    unsigned off = (unsigned)(bank8k % nbanks) * 0x2000;
    memcpy(&NES_MEM[cpu_base], &s_prg[off], 0x2000);
}

void ppu_reset(void)
{
    memset(ppu_vram, 0, sizeof ppu_vram);
    memset(ppu_pal, 0, sizeof ppu_pal);
    memset(ppu_oam, 0, sizeof ppu_oam);
    ppu_ctrl = ppu_mask = ppu_scroll_x = ppu_scroll_y = 0;
    s_status = 0; s_oamaddr = 0; s_vaddr = 0; s_wtoggle = 0; s_readbuf = 0;
    s_mmc3_sel = 0; s_mirror = 0;
    for (int i = 0; i < 8; i++) s_mmc3_bank[i] = i;   /* identity default */
    recompute_chr();
}

/* CHR byte at PPU pattern address (0..$1FFF). */
static u8 chr_at(unsigned a)
{
    a &= 0x1FFF;
    unsigned off = (unsigned)s_chr_win[a >> 10] * 0x400 + (a & 0x3FF);
    return (off < sizeof s_chr) ? s_chr[off] : 0;
}

/* Map a nametable tile coord (logical, 0..63 x, 0..59 y) to a $0..$7FF vram
 * offset honoring the mirroring mode. */
static unsigned nt_offset(int tx, int ty)
{
    int ntx = (tx >> 5) & 1;         /* which horizontal nametable (0/1) */
    int nty = (ty >> 5) & 1;         /* which vertical nametable (0/1)  */
    int phys;                         /* physical NT 0 or 1 in the 2KB vram */
    if (s_mirror == 0) phys = nty;    /* horizontal arrangement: vert coord picks */
    else               phys = ntx;    /* vertical arrangement: horiz coord picks */
    int cx = tx & 31, cy = ty & 31;
    return (unsigned)(phys ? 0x400 : 0) + (unsigned)(cy * 32 + cx);
}

static u8 attr_bits(int tx, int ty)
{
    int ntx = (tx >> 5) & 1, nty = (ty >> 5) & 1;
    int phys = (s_mirror == 0) ? nty : ntx;
    int cx = tx & 31, cy = ty & 31;
    unsigned base = (phys ? 0x400 : 0) + 0x3C0;
    u8 ab = ppu_vram[base + (cy >> 2) * 8 + (cx >> 2)];
    int quad = ((cy & 2) ? 2 : 0) + ((cx & 2) ? 1 : 0);
    return (ab >> (quad * 2)) & 3;
}

static void put(u8 *out, int x, int y, u8 palidx)
{
    const u8 *c = NES_PAL[palidx & 0x3F];
    u8 *p = out + (y * PPU_W + x) * 3;
    p[0] = c[0]; p[1] = c[1]; p[2] = c[2];
}

void ppu_render(u8 *out)
{
    int bg_pt = (ppu_ctrl & 0x10) ? 0x1000 : 0x0000;   /* BG pattern table */
    int sp_pt = (ppu_ctrl & 0x08) ? 0x1000 : 0x0000;   /* 8x8 sprite pattern table */
    u8 backdrop = ppu_pal[0];

    /* ---- background ---- */
    if (ppu_mask & 0x08) {
        for (int sy = 0; sy < PPU_H; sy++) {
            int wy = sy + ppu_scroll_y;
            int ty = (wy >> 3), fy = wy & 7;
            for (int sx = 0; sx < PPU_W; sx++) {
                int wx = sx + ppu_scroll_x;
                int tx = (wx >> 3), fx = wx & 7;
                u8 tile = ppu_vram[nt_offset(tx, ty)];
                unsigned a = bg_pt + tile * 16 + fy;
                int bit = 7 - fx;
                int v = ((chr_at(a) >> bit) & 1) | (((chr_at(a + 8) >> bit) & 1) << 1);
                u8 idx = v ? ppu_pal[attr_bits(tx, ty) * 4 + v] : backdrop;
                put(out, sx, sy, idx);
            }
        }
    } else {
        for (int i = 0; i < PPU_W * PPU_H; i++) {
            const u8 *c = NES_PAL[backdrop & 0x3F];
            out[i*3]=c[0]; out[i*3+1]=c[1]; out[i*3+2]=c[2];
        }
    }

    /* ---- sprites (8x8 mode; draw front-priority on top of BG) ---- */
    if (ppu_mask & 0x10) {
        for (int i = 63; i >= 0; i--) {            /* lower index = higher priority */
            u8 *o = ppu_oam + i * 4;
            int y = o[0] + 1, tile = o[1], at = o[2], x = o[3];
            int pal = 0x10 + (at & 3) * 4;
            int hflip = at & 0x40, vflip = at & 0x80;
            if (y >= PPU_H || y < 0) continue;
            for (int row = 0; row < 8; row++) {
                int py = y + row; if (py < 0 || py >= PPU_H) continue;
                int sr = vflip ? 7 - row : row;
                unsigned a = sp_pt + tile * 16 + sr;
                u8 p0 = chr_at(a), p1 = chr_at(a + 8);
                for (int col = 0; col < 8; col++) {
                    int px = x + col; if (px < 0 || px >= PPU_W) continue;
                    int sc = hflip ? col : 7 - col;
                    int v = ((p0 >> sc) & 1) | (((p1 >> sc) & 1) << 1);
                    if (!v) continue;              /* transparent */
                    put(out, px, py, ppu_pal[pal + v]);
                }
            }
        }
    }
}

void ppu_debug_tilesheet(int which, u8 *out)
{
    static const u8 gray[4] = {0, 12, 0x10, 0x30};   /* master-palette indices */
    int base = which ? 0x1000 : 0x0000;
    for (int t = 0; t < 256; t++) {
        int ox = (t & 15) * 8, oy = (t >> 4) * 8;
        for (int row = 0; row < 8; row++) {
            u8 p0 = chr_at(base + t * 16 + row), p1 = chr_at(base + t * 16 + 8 + row);
            for (int col = 0; col < 8; col++) {
                int v = ((p0 >> (7 - col)) & 1) | (((p1 >> (7 - col)) & 1) << 1);
                const u8 *c = NES_PAL[gray[v]];
                u8 *p = out + ((oy + row) * 128 + (ox + col)) * 3;
                p[0]=c[0]; p[1]=c[1]; p[2]=c[2];
            }
        }
    }
}

/* ---- register hooks (the REG_W/REG_R the game drives) ---- */
extern u8 NES_MEM[0x10000];

void nes_reg_write(u16 addr, u8 val)
{
    switch (addr) {
    case 0x2000: ppu_ctrl = val; break;
    case 0x2001: ppu_mask = val; break;
    case 0x2003: s_oamaddr = val; break;
    case 0x2004: ppu_oam[s_oamaddr++] = val; break;
    case 0x2005:
        if (!s_wtoggle) { ppu_scroll_x = val; s_wtoggle = 1; }
        else            { ppu_scroll_y = val; s_wtoggle = 0; }
        break;
    case 0x2006:
        if (!s_wtoggle) { s_vaddr = (u16)((s_vaddr & 0x00FF) | (val << 8)); s_wtoggle = 1; }
        else            { s_vaddr = (u16)((s_vaddr & 0xFF00) | val); s_wtoggle = 0; }
        break;
    case 0x2007: {
        u16 a = s_vaddr & 0x3FFF;
        if (a >= 0x3F00) {                     /* palette RAM (with mirroring) */
            u16 p = a & 0x1F;
            if ((p & 3) == 0) p &= 0x0F;       /* $3F1x/0x mirror of backdrop slots */
            ppu_pal[p] = val;
        } else {                               /* nametables ($2000-$2FFF) -> 2KB */
            ppu_vram[a & 0x7FF] = val;
        }
        s_vaddr += (ppu_ctrl & 0x04) ? 32 : 1;
        break;
    }
    case 0x4014: {                             /* OAM DMA from CPU page (val<<8) */
        unsigned base = (unsigned)val << 8;
        for (int i = 0; i < 256; i++) ppu_oam[(u8)(s_oamaddr + i)] = NES_MEM[base + i];
        break;
    }
    /* ---- MMC3 (CHR banking + mirroring) ---- */
    case 0x8000: s_mmc3_sel = val; recompute_chr(); break;
    case 0x8001:
        s_mmc3_bank[s_mmc3_sel & 7] = val;
        recompute_chr();
        if ((s_mmc3_sel & 7) == 6) ppu_map_prg(0x8000, val);   /* R6 -> $8000 */
        else if ((s_mmc3_sel & 7) == 7) ppu_map_prg(0xA000, val); /* R7 -> $A000 */
        break;
    case 0xA000: s_mirror = (val & 1) ? 0 : 1; break;  /* MMC3: 0=vert,1=horiz arrangement */
    default: break;                            /* APU / other: ignored here */
    }
}

u8 nes_reg_read(u16 addr)
{
    switch (addr) {
    case 0x2002: {
        u8 s = s_status;
        s_status &= (u8)~0x80;     /* reading clears vblank flag ... */
        s_wtoggle = 0;             /* ... and resets the $2005/$2006 write latch */
        return s;
    }
    case 0x2004: return ppu_oam[s_oamaddr];
    case 0x2007: {
        u16 a = s_vaddr & 0x3FFF;
        u8 ret;
        if (a >= 0x3F00) { ret = ppu_pal[a & 0x1F]; }
        else { ret = s_readbuf; s_readbuf = ppu_vram[a & 0x7FF]; }  /* buffered read */
        s_vaddr += (ppu_ctrl & 0x04) ? 32 : 1;
        return ret;
    }
    case 0x4016: case 0x4017: return 0;        /* controllers: none (headless) */
    default: return 0;
    }
}

int ppu_chr_win_dbg(int i) { return s_chr_win[i & 7]; }

/* set/clear the vblank + sprite-0 status bits (frame driver uses these) */
void ppu_set_vblank(int on) { if (on) s_status |= 0x80; else s_status &= (u8)~0x80; }
void ppu_set_sprite0(int on) { if (on) s_status |= 0x40; else s_status &= (u8)~0x40; }

/* ---- PPM (P6) writer ---- */
int ppm_write(const char *path, const u8 *rgb, int w, int h)
{
    FILE *f = fopen(path, "wb");
    if (!f) return -1;
    fprintf(f, "P6\n%d %d\n255\n", w, h);
    fwrite(rgb, 3, (size_t)w * h, f);
    fclose(f);
    return 0;
}
