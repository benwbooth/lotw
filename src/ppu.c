



#include "ppu.h"
#include <stdio.h>
#include <string.h>


static const u8 LOTW_PALETTE[64][3] = {
  { 84, 84, 84},{  0, 30,116},{  8, 16,144},{ 48,  0,136},{ 68,  0,100},{ 92,  0, 48},{ 84,  4,  0},{ 60, 24,  0},
  { 32, 42,  0},{  8, 58,  0},{  0, 64,  0},{  0, 60,  0},{  0, 50, 60},{  0,  0,  0},{  0,  0,  0},{  0,  0,  0},
  {152,150,152},{  8, 76,196},{ 48, 50,236},{ 92, 30,228},{136, 20,176},{160, 20,100},{152, 34, 32},{120, 60,  0},
  { 84, 90,  0},{ 40,114,  0},{  8,124,  0},{  0,118, 40},{  0,102,120},{  0,  0,  0},{  0,  0,  0},{  0,  0,  0},
  {236,238,236},{ 76,154,236},{120,124,236},{176, 98,236},{228, 84,236},{236, 88,180},{236,106,100},{212,136, 32},
  {160,170,  0},{116,196,  0},{ 76,208, 32},{ 56,204,108},{ 56,180,204},{ 60, 60, 60},{  0,  0,  0},{  0,  0,  0},
  {236,238,236},{168,204,236},{188,188,236},{212,178,236},{236,174,236},{236,174,212},{236,180,176},{228,196,144},
  {204,210,120},{180,222,120},{168,226,144},{152,226,180},{160,214,228},{160,162,160},{  0,  0,  0},{  0,  0,  0},
};


u8 ppu_vram[0x800];
u8 ppu_pal[0x20];
u8 ppu_oam[0x100];
u8 ppu_ctrl, ppu_mask, ppu_scroll_x, ppu_scroll_y;

static u8  s_status;
static u8  s_openbus;

static u8  s_oamaddr;
static u16 s_vaddr;
static u8  s_wtoggle;
static u8  s_readbuf;



static u8  s_chr[0x10000];
static unsigned s_chr_len;
static int s_chr_win[8];
static u8  s_mmc3_sel;
static u8  s_mmc3_bank[8];
static int s_mirror;

static void recompute_chr(void)
{


    int inv = (s_mmc3_sel & 0x80) ? 1 : 0;
    int two[4];
    int one[4];
    two[0] = (s_mmc3_bank[0] & 0xFE);     two[1] = (s_mmc3_bank[0] & 0xFE) | 1;
    two[2] = (s_mmc3_bank[1] & 0xFE);     two[3] = (s_mmc3_bank[1] & 0xFE) | 1;
    one[0] = s_mmc3_bank[2]; one[1] = s_mmc3_bank[3];
    one[2] = s_mmc3_bank[4]; one[3] = s_mmc3_bank[5];
    if (!inv) {
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




extern u8 LOTW_MEMORY[0x10000];
static u8  s_prg[0x40000];
static unsigned s_prg_len;

void ppu_load_prg(const u8 *prg, unsigned len)
{
    if (len > sizeof s_prg) len = sizeof s_prg;
    memcpy(s_prg, prg, len);
    s_prg_len = len;
}

void ppu_map_prg(u16 cpu_base, u8 bank8k)
{
    if (!s_prg_len) return;
    unsigned nbanks = s_prg_len / 0x2000;
    unsigned off = (unsigned)(bank8k % nbanks) * 0x2000;
    memcpy(&LOTW_MEMORY[cpu_base], &s_prg[off], 0x2000);
}

void ppu_reset(void)
{
    memset(ppu_vram, 0, sizeof ppu_vram);
    memset(ppu_pal, 0, sizeof ppu_pal);
    memset(ppu_oam, 0, sizeof ppu_oam);
    ppu_ctrl = ppu_mask = ppu_scroll_x = ppu_scroll_y = 0;
    s_status = 0; s_openbus = 0; s_oamaddr = 0; s_vaddr = 0; s_wtoggle = 0; s_readbuf = 0;
    s_mmc3_sel = 0; s_mirror = 0;
    for (int i = 0; i < 8; i++) s_mmc3_bank[i] = i;
    recompute_chr();
}


static u8 chr_at(unsigned a)
{
    a &= 0x1FFF;
    unsigned off = (unsigned)s_chr_win[a >> 10] * 0x400 + (a & 0x3FF);
    return (off < sizeof s_chr) ? s_chr[off] : 0;
}



static unsigned nt_offset(int tx, int ty)
{
    int ntx = (tx >> 5) & 1;
    int nty = (ty >> 5) & 1;
    int phys;
    if (s_mirror == 0) phys = nty;
    else               phys = ntx;
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

static unsigned nt_addr_offset(u16 addr)
{
    unsigned a = (unsigned)(addr - 0x2000) & 0x0FFF;
    unsigned nt = (a >> 10) & 3;
    unsigned off = a & 0x03FF;
    unsigned phys = (s_mirror == 0) ? (nt >> 1) : (nt & 1);
    return phys * 0x400 + off;
}

static void put(u8 *out, int x, int y, u8 palidx)
{
    const u8 *c = LOTW_PALETTE[palidx & 0x3F];
    u8 *p = out + (y * PPU_W + x) * 3;
    p[0] = c[0]; p[1] = c[1]; p[2] = c[2];
}





static u8 bg_pixel(int wx, int wy, int bg_pt)
{
    wx &= 0x1FF;
    wy %= 480; if (wy < 0) wy += 480;
    int ntx = (wx >> 8) & 1, nty = (wy >= 240) ? 1 : 0;
    int lx = wx & 0xFF, ly = (wy >= 240) ? wy - 240 : wy;
    int phys = (s_mirror == 0) ? nty : ntx;
    unsigned nt = phys ? 0x400 : 0;
    int cx = lx >> 3, cy = ly >> 3, fx = lx & 7, fy = ly & 7;
    u8 tile = ppu_vram[nt + cy * 32 + cx];
    u8 ab = ppu_vram[nt + 0x3C0 + (cy >> 2) * 8 + (cx >> 2)];
    int pal = (ab >> ((((cy & 2) ? 2 : 0) + ((cx & 2) ? 1 : 0)) * 2)) & 3;
    unsigned a = bg_pt + tile * 16 + fy;
    int v = ((chr_at(a) >> (7 - fx)) & 1) | (((chr_at(a + 8) >> (7 - fx)) & 1) << 1);
    return v ? ppu_pal[pal * 4 + v] : ppu_pal[0];
}

void ppu_render(u8 *out)
{
    int bg_pt = (ppu_ctrl & 0x10) ? 0x1000 : 0x0000;
    int sp_pt = (ppu_ctrl & 0x08) ? 0x1000 : 0x0000;
    int tall  = (ppu_ctrl & 0x20) ? 1 : 0;




    int split  = LOTW_MEMORY[0x29] != 0;
    int splitY = split ? (ppu_oam[0] + 1) : 0;
    if (splitY < 0) splitY = 0; else if (splitY > PPU_H) splitY = PPU_H;






    if (ppu_mask & 0x08) {
        int bx = (ppu_ctrl & 1) ? 256 : 0, by = (ppu_ctrl & 2) ? 240 : 0;
        for (int sy = splitY; sy < PPU_H; sy++) {
            int wy = by + ppu_scroll_y + (sy - splitY) + (split ? 6 : 0);
            for (int sx = 0; sx < PPU_W; sx++)
                put(out, sx, sy, bg_pixel(bx + ppu_scroll_x + sx, wy, bg_pt));
        }
    } else {
        for (int sy = splitY; sy < PPU_H; sy++)
            for (int sx = 0; sx < PPU_W; sx++) put(out, sx, sy, ppu_pal[0]);
    }


    if (split && (ppu_mask & 0x08)) {
        u8 b1 = s_mmc3_bank[1], b4 = s_mmc3_bank[4], b5 = s_mmc3_bank[5];
        s_mmc3_bank[1] = 0x16; s_mmc3_bank[4] = 0x3E; s_mmc3_bank[5] = 0x3F;
        recompute_chr();
        for (int sy = 0; sy < splitY; sy++)
            for (int sx = 0; sx < PPU_W; sx++)
                put(out, sx, sy, bg_pixel(sx, 0xC4 + sy, bg_pt));
        s_mmc3_bank[1] = b1; s_mmc3_bank[4] = b4; s_mmc3_bank[5] = b5;
        recompute_chr();
    }


    if (ppu_mask & 0x10) {
        for (int i = 63; i >= 0; i--) {
            u8 *o = ppu_oam + i * 4;
            int y = o[0] + 1, at = o[2], x = o[3];
            int pal = 0x10 + (at & 3) * 4, hflip = at & 0x40, vflip = at & 0x80;
            int h = tall ? 16 : 8;
            if (y >= PPU_H || y + h <= 0) continue;
            for (int row = 0; row < h; row++) {
                int py = y + row; if (py < 0 || py >= PPU_H) continue;
                int sr = vflip ? (h - 1 - row) : row;
                unsigned a;
                if (tall) {
                    unsigned base = ((o[1] & 1) ? 0x1000u : 0x0000u) + (o[1] & 0xFE) * 16;
                    a = base + (sr < 8 ? sr : 16 + (sr - 8));
                } else {
                    a = sp_pt + o[1] * 16 + sr;
                }
                u8 p0 = chr_at(a), p1 = chr_at(a + 8);
                for (int col = 0; col < 8; col++) {
                    int px = x + col; if (px < 0 || px >= PPU_W) continue;
                    int sc = hflip ? col : 7 - col;
                    int v = ((p0 >> sc) & 1) | (((p1 >> sc) & 1) << 1);
                    if (!v) continue;
                    put(out, px, py, ppu_pal[pal + v]);
                }
            }
        }
    }
}




void ppu_render_statusbar(u8 *out, int rows)
{
    u8 b1 = s_mmc3_bank[1], b4 = s_mmc3_bank[4], b5 = s_mmc3_bank[5];
    s_mmc3_bank[1] = 0x16; s_mmc3_bank[4] = 0x3E; s_mmc3_bank[5] = 0x3F;
    recompute_chr();
    int bg_pt = (ppu_ctrl & 0x10) ? 0x1000 : 0x0000;
    for (int sy = 0; sy < rows && sy < PPU_H; sy++) {
        int wy = sy + 0xC4, ty = wy >> 3, fy = wy & 7;
        for (int sx = 0; sx < PPU_W; sx++) {
            int tx = sx >> 3, fx = sx & 7;
            u8 tile = ppu_vram[nt_offset(tx, ty)];
            unsigned a = bg_pt + tile * 16 + fy;
            int bit = 7 - fx;
            int v = ((chr_at(a) >> bit) & 1) | (((chr_at(a + 8) >> bit) & 1) << 1);
            u8 idx = v ? ppu_pal[attr_bits(tx, ty) * 4 + v] : ppu_pal[0];
            put(out, sx, sy, idx);
        }
    }
    s_mmc3_bank[1] = b1; s_mmc3_bank[4] = b4; s_mmc3_bank[5] = b5;
    recompute_chr();
}

void ppu_debug_tilesheet(int which, u8 *out)
{
    static const u8 gray[4] = {0, 12, 0x10, 0x30};
    int base = which ? 0x1000 : 0x0000;
    for (int t = 0; t < 256; t++) {
        int ox = (t & 15) * 8, oy = (t >> 4) * 8;
        for (int row = 0; row < 8; row++) {
            u8 p0 = chr_at(base + t * 16 + row), p1 = chr_at(base + t * 16 + 8 + row);
            for (int col = 0; col < 8; col++) {
                int v = ((p0 >> (7 - col)) & 1) | (((p1 >> (7 - col)) & 1) << 1);
                const u8 *c = LOTW_PALETTE[gray[v]];
                u8 *p = out + ((oy + row) * 128 + (ox + col)) * 3;
                p[0]=c[0]; p[1]=c[1]; p[2]=c[2];
            }
        }
    }
}


extern u8 LOTW_MEMORY[0x10000];
void (*apu_write_hook)(u16, u8);


void vblank_commit(RoutineContext *r);


static void lotw_frame_wait_default(RoutineContext *r)
{
    vblank_commit(r);
}
void (*lotw_frame_wait_hook)(RoutineContext *r) = lotw_frame_wait_default;

void lotw_frame_wait(RoutineContext *r)
{
    if (lotw_frame_wait_hook)
        lotw_frame_wait_hook(r);
}

void lotw_prg_map_shadow(void)
{
    ppu_map_prg(0x8000, LOTW_MEMORY[0x30]);
    ppu_map_prg(0xA000, LOTW_MEMORY[0x31]);
}



static u8 s_buttons, s_ctrl_latch, s_strobe;
void ppu_set_buttons(u8 b) { s_buttons = b; }

void lotw_device_write(u16 addr, u8 val)
{
    if (addr >= 0x2000 && addr <= 0x2007) s_openbus = val;
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
        if (a >= 0x3F00) {
            u16 p = a & 0x1F;
            if ((p & 3) == 0) {
                p &= 0x0F;
                ppu_pal[p] = val;
                ppu_pal[p | 0x10] = val;
            } else {
                ppu_pal[p] = val;
            }
        } else {
            ppu_vram[nt_addr_offset(a)] = val;
        }
        s_vaddr += (ppu_ctrl & 0x04) ? 32 : 1;
        break;
    }
    case 0x4014: {
        unsigned base = (unsigned)val << 8;
        for (int i = 0; i < 256; i++) ppu_oam[(u8)(s_oamaddr + i)] = LOTW_MEMORY[base + i];
        break;
    }

    case 0x8000: s_mmc3_sel = val; recompute_chr(); break;
    case 0x8001:
        s_mmc3_bank[s_mmc3_sel & 7] = val;
        recompute_chr();
        if ((s_mmc3_sel & 7) == 6) ppu_map_prg(0x8000, val);
        else if ((s_mmc3_sel & 7) == 7) ppu_map_prg(0xA000, val);
        break;
    case 0xA000: s_mirror = (val & 1) ? 0 : 1; break;
    case 0x4016:
        s_strobe = val & 1;
        if (s_strobe) s_ctrl_latch = s_buttons;
        break;
    default:
        if (addr >= 0x4000 && addr <= 0x4017 && addr != 0x4014 && apu_write_hook)
            apu_write_hook(addr, val);
        break;
    }
}

u8 lotw_device_read(u16 addr)
{
    switch (addr) {
    case 0x2002: {

        u8 s = (u8)((s_status & 0xE0) | (s_openbus & 0x1F));
        s_status &= (u8)~0x80;
        s_wtoggle = 0;
        return s;
    }
    case 0x2004: { u8 ret = ppu_oam[s_oamaddr]; s_openbus = ret; return ret; }
    case 0x2007: {
        u16 a = s_vaddr & 0x3FFF;
        u8 ret;
        if (a >= 0x3F00) {
            u16 p = a & 0x1F;
            if ((p & 3) == 0) p &= 0x0F;
            ret = ppu_pal[p];
        }
        else { ret = s_readbuf; s_readbuf = ppu_vram[nt_addr_offset(a)]; }
        s_vaddr += (ppu_ctrl & 0x04) ? 32 : 1;
        s_openbus = ret;
        return ret;
    }
    case 0x4016: {
        if (s_strobe) return s_buttons & 1;
        u8 bit = s_ctrl_latch & 1;
        s_ctrl_latch >>= 1;
        return bit;
    }
    case 0x4017: return 0;
    default: return 0;
    }
}

int ppu_chr_win_dbg(int i) { return s_chr_win[i & 7]; }
int ppu_mirror_dbg(void) { return s_mirror; }


u8 (*lotw_next_input)(void) = 0;

void ppu_set_vblank(int on) { if (on) s_status |= 0x80; else s_status &= (u8)~0x80; }
void ppu_set_sprite0(int on) { if (on) s_status |= 0x40; else s_status &= (u8)~0x40; }






void ppu_eval_sprite_overflow(void)
{
    int h = (ppu_ctrl & 0x20) ? 16 : 8;
    u8 perline[240];
    int s;
    for (s = 0; s < 240; s++) perline[s] = 0;
    for (s = 0; s < 64; s++) {
        int y = ppu_oam[s * 4];
        if (y >= 0xEF) continue;
        int top = y + 1, bot = y + 1 + h;
        if (bot > 240) bot = 240;
        for (int sl = top; sl < bot; sl++)
            if (++perline[sl] > 8) { s_status |= 0x20; return; }
    }
    s_status &= (u8)~0x20;
}


int ppm_write(const char *path, const u8 *rgb, int w, int h)
{
    FILE *f = fopen(path, "wb");
    if (!f) return -1;
    fprintf(f, "P6\n%d %d\n255\n", w, h);
    fwrite(rgb, 3, (size_t)w * h, f);
    fclose(f);
    return 0;
}
