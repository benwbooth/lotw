/* $F89A:  sound_tick - per-frame audio driver entry.
 *   JSR sound_set_default_banks
 *   LDA #$40 / STA $02 / JSR sfx_overlay_voice
 *   LDA $8D / BEQ L_F8CD                         ; $8D!=0 -> "silenced/paused" path
 *   ; --- $8D != 0 path: force fixed volumes ---
 *   LDA #$00 / BIT $D4 / BMI L_F8B7
 *     LDA $A9 / AND #$C0 / ORA #$30 / STA SQ2_VOL
 *   L_F8B7:
 *     LDA $99 / AND #$C0 / ORA #$30 / STA SQ1_VOL
 *     LDA #$00 / STA TRI_LINEAR
 *     LDA #$30 / STA NOISE_VOL
 *     JMP L_F8EC
 *   L_F8CD ($8D == 0): run the four voice updaters with $02 = 0,$10,$20,$30
 *     JSR sound_set_song_banks
 *     STA $02=#$00 / JSR F8F0 ; =#$10 / JSR F96E ; =#$20 / JSR FA09 ; =#$30 / JSR FB1F
 *   L_F8EC: JSR sound_restore_game_banks / RTS
 */
#include "ram.h"
#include "regs.h"

void sound_set_default_banks(Regs *r);
void sound_set_song_banks(Regs *r);
void sound_restore_game_banks(Regs *r);
void sfx_overlay_voice(Regs *r);
void sub_F8F0(Regs *r);
void sub_F96E(Regs *r);
void sub_FA09(Regs *r);
void sub_FB1F(Regs *r);

#define SQ1_VOL    0x4000
#define SQ2_VOL    0x4004
#define TRI_LINEAR 0x4008
#define NOISE_VOL  0x400C

void sound_tick(Regs *r)
{
    sound_set_default_banks(r);

    RAM8(0x02) = 0x40;
    r->a = 0x40;
    sfx_overlay_voice(r);

    if (RAM8(0x8D) != 0) {
        /* fixed-volume path */
        /* LDA #$00 / BIT $D4 : N = bit7 of $D4; BMI skips SQ2 store */
        if (!(RAM8(0xD4) & 0x80)) {
            REG_W(SQ2_VOL, (RAM8(0xA9) & 0xC0) | 0x30);
        }
        /* L_F8B7 */
        REG_W(SQ1_VOL, (RAM8(0x99) & 0xC0) | 0x30);
        REG_W(TRI_LINEAR, 0x00);
        REG_W(NOISE_VOL, 0x30);
        r->a = 0x30;
    } else {
        /* L_F8CD: run voice updaters */
        sound_set_song_banks(r);
        RAM8(0x02) = 0x00; r->a = 0x00; sub_F8F0(r);
        RAM8(0x02) = 0x10; r->a = 0x10; sub_F96E(r);
        RAM8(0x02) = 0x20; r->a = 0x20; sub_FA09(r);
        RAM8(0x02) = 0x30; r->a = 0x30; sub_FB1F(r);
    }

    /* L_F8EC */
    sound_restore_game_banks(r);
}
