/* $D6D4: player walked off a screen edge -> scroll/transition to the adjacent
 * screen. Branches on player_y ($45):
 *   player_y < $10  : exited top    -> L_D739 (move up a map row, or warp/D866)
 *   player_y >= $A1 : exited bottom -> L_D750 (move down a map row, or wrap/D883)
 *   else            : exited left/right edge -> horizontal screen change (L_D772)
 * Vertical edge changes do a plain rebuild (L_D761). Horizontal changes do the
 * scrolling sprite animation (L_D772 + the two L_D7A1/L_D810 wipe loops).
 * The JMP D866 / JMP D883 tails (fixed-coord warps that fall into D895) are
 * inlined.  Returns C=1 on a completed transition, C=0 if nothing happened.
 */
#include "ram.h"
#include "regs.h"

#define equipped_item RAM8(0x0055)
#define carried_item0 0x0051
#define map_screen_x  RAM8(0x0047)
#define map_screen_y  RAM8(0x0048)
#define scroll_x_tile RAM8(0x007C)
#define scroll_x_fine RAM8(0x007B)
#define player_x_tile RAM8(0x0044)
#define player_x_fine RAM8(0x0043)
#define player_y      RAM8(0x0045)
#define vram_dst_lo   RAM8(0x0016)
#define vram_dst_hi   RAM8(0x0017)

void sub_DBDD(Regs *r);
void sub_DC87(Regs *r);
void sub_C1D8(Regs *r);
void sub_D08A(Regs *r);
void sub_D07C(Regs *r);
void scene_assemble(Regs *r);
void sub_C5CB(Regs *r);
void sub_C569(Regs *r);
void sub_C76C(Regs *r);
void queue_ppu_job_and_wait(Regs *r);
void farcall_bank_09_r7(Regs *r);
void sub_C3E5(Regs *r);
void sub_C1C7(Regs *r);
void sub_C492(Regs *r);

/* L_D895: the common screen-rebuild tail after a fixed-coord warp (D866/D883). */
static void scene_rebuild_full(Regs *r)
{
    sub_C3E5(r);
    sub_D08A(r);
    scene_assemble(r);
    sub_C5CB(r);
    sub_D07C(r);
    sub_C1C7(r);
    sub_C1D8(r);
    sub_C492(r);
    RAM8(0x36) = 0;     /* oracle NMI sync_clear leaves $36=0 after frame waits */
    r->c = 1;                                /* SEC */
}

/* L_D761: vertical-transition rebuild. */
static void scene_rebuild_vert(Regs *r)
{
    sub_D08A(r);
    sub_D07C(r);
    scene_assemble(r);
    sub_C5CB(r);
    sub_C569(r);
    RAM8(0x36) = 0;     /* oracle NMI sync_clear leaves $36=0 after frame waits */
    r->c = 1;                                /* SEC */
}

void sub_D6D4(Regs *r)
{
    u8 a = player_y;

    if (a < 0x10) {
        /* L_D739: exited top */
        sub_DC87(r);
        if (r->c == 0) { r->c = 0; return; } /* L_D731: CLC RTS */
        if (map_screen_y == 0x00) {
            /* L_D733: JMP D866 */
            map_screen_y = 0x10; map_screen_x = 0x03; scroll_x_tile = 0x12;
            player_y = 0xB0; player_x_tile = 0x1A; player_x_fine = 0x00;
            scroll_x_fine = 0x00;
            scene_rebuild_full(r);
            return;
        }
        if (map_screen_y == 0x10) { r->c = 0; return; }  /* L_D731 */
        map_screen_y = (u8)(map_screen_y - 1);
        player_y = 0xB0;
        scene_rebuild_vert(r);               /* L_D761 */
        return;
    }

    if (a >= 0xA1) {
        /* L_D750: exited bottom */
        if (map_screen_y == 0x10) {
            /* L_D736: JMP D883 */
            map_screen_y = 0x00; map_screen_x = 0x00; scroll_x_tile = 0x00;
            player_y = 0x00; player_x_fine = 0x00; scroll_x_fine = 0x00;
            player_x_tile = 0x01;
            scene_rebuild_full(r);
            return;
        }
        if ((u8)(map_screen_y + 1) >= 0x10) { r->c = 0; return; }  /* L_D731 */
        map_screen_y = (u8)(map_screen_y + 1);
        player_y = 0x00;
        scene_rebuild_vert(r);               /* L_D761 */
        return;
    }

    /* middle: horizontal edge */
    if (map_screen_y == 0x10) { r->c = 0; return; }  /* L_D731 */
    sub_DBDD(r);
    RAM8(0x85) = 0x00;
    RAM8(0x56) = (u8)(RAM8(0x56) & 0x07);

    if (player_x_tile == 0x00) {
        /* L_D714: exited left.  LDX msx / DEX / BMI L_D731 */
        if ((u8)((map_screen_x - 1)) & 0x80) { r->c = 0; return; }
        map_screen_x = (u8)(map_screen_x - 1);
        RAM8(0x57) = 0x00;
        sub_C1D8(r);
        scroll_x_tile = 0x30;
        player_x_tile = 0x3F;
        player_x_fine = 0x00;
        /* JMP L_D772 (player_x_tile != 0 here -> L_D7F8 branch) */
    } else {
        if (player_x_tile < 0x3E) { r->c = 0; return; }  /* L_D731 */
        /* exited right */
        if ((u8)(map_screen_x + 1) >= 0x04) { r->c = 0; return; }  /* L_D731 */
        map_screen_x = (u8)(map_screen_x + 1);
        RAM8(0x57) = 0x40;
        sub_C1D8(r);
        scroll_x_tile = 0x00;
        player_x_fine = 0x00;
        player_x_tile = 0x00;
        /* JMP L_D772 (player_x_tile == 0 here -> L_D7A1 branch) */
    }

    /* L_D772 */
    sub_D08A(r);
    sub_D07C(r);
    scroll_x_fine = 0x00;
    scene_assemble(r);
    sub_C76C(r);
    sub_C569(r);

    if (player_x_tile != 0x00) {
        /* L_D7F8: scroll-left wipe */
        RAM8(0x1D) = 0x01;
        RAM8(0x1C) = 0x00;
        RAM8(0x0213) = 0x00;
        RAM8(0x0217) = 0x08;
        RAM8(0x0A) = 0x0F;                       /* $0A/$0B are real RAM counters */
        do {                                     /* L_D810: outer */
            RAM8(0x0B) = 0x03;
            do {                                 /* L_D814: inner */
                if (RAM8(0x0B) == 0) {           /* BNE skipped only when $0B==0 */
                    RAM8(0x0213) = (u8)(RAM8(0x0213) - 1);
                    RAM8(0x0217) = (u8)(RAM8(0x0217) - 1);
                    if ((RAM8(0x4E) | RAM8(0x4F)) == 0) {
                        RAM8(0x0211) ^= 0x04;
                        RAM8(0x0215) ^= 0x04;
                    }
                }
                /* L_D832 */
                RAM8(0x0213) = (u8)(RAM8(0x0213) + 0x04);
                RAM8(0x0217) = (u8)(RAM8(0x0213) + 0x08);
                RAM8(0x1C) = (u8)(RAM8(0x1C) - 0x04);
                r->a = 0xFF;
                queue_ppu_job_and_wait(r);
                RAM8(0x0B) = (u8)(RAM8(0x0B) - 1);
            } while ((RAM8(0x0B) & 0x80) == 0);  /* DEC $0B / BPL */
            RAM8(0x0A) = (u8)(RAM8(0x0A) - 1);
        } while ((RAM8(0x0A) & 0x80) == 0);      /* DEC $0A / BPL */
        vram_dst_lo = 0x1E;
        vram_dst_hi = 0x20;
        RAM8(0x0C) = 0x2F;
        farcall_bank_09_r7(r);
        RAM8(0x36) = 0; /* oracle NMI sync_clear leaves $36=0 after frame waits */
        r->c = 1;                            /* L_D864: SEC */
        return;
    }

    /* L_D7A1 path: player_x_tile == 0 -> scroll-right wipe */
    RAM8(0x1C) = 0xFC;
    RAM8(0x1D) = 0x01;
    RAM8(0x0213) = 0xF0;
    RAM8(0x0217) = 0xF8;
    RAM8(0x0A) = 0x0F;                          /* $0A/$0B are real RAM counters */
    do {                                        /* L_D7A1: outer */
        RAM8(0x0B) = 0x03;
        do {                                    /* L_D7A5: inner */
            if (RAM8(0x0B) == 0) {              /* INC block only on $0B==0 pass */
                RAM8(0x0213) = (u8)(RAM8(0x0213) + 1);
                RAM8(0x0217) = (u8)(RAM8(0x0217) + 1);
                if ((RAM8(0x4E) | RAM8(0x4F)) == 0) {
                    RAM8(0x0211) ^= 0x04;
                    RAM8(0x0215) ^= 0x04;
                }
            }
            /* L_D7C3 */
            RAM8(0x0213) = (u8)(RAM8(0x0213) - 0x04);
            RAM8(0x0217) = (u8)(RAM8(0x0213) + 0x08);
            RAM8(0x1C) = (u8)(RAM8(0x1C) + 0x04);
            r->a = 0xFF;
            queue_ppu_job_and_wait(r);
            RAM8(0x0B) = (u8)(RAM8(0x0B) - 1);
        } while ((RAM8(0x0B) & 0x80) == 0);     /* DEC $0B / BPL */
        RAM8(0x0A) = (u8)(RAM8(0x0A) - 1);
    } while ((RAM8(0x0A) & 0x80) == 0);         /* DEC $0A / BPL */
    vram_dst_lo = 0x00;
    vram_dst_hi = 0x24;
    RAM8(0x0C) = 0x10;
    farcall_bank_09_r7(r);
    RAM8(0x36) = 0;     /* oracle NMI sync_clear leaves $36=0 after frame waits */
    r->c = 1;                                /* JMP L_D864: SEC */
}
