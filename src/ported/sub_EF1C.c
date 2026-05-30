/* $EF1C — enemy projectile/drop spawn step machine.
 *
 *   LDA $EE / AND #$7F / BNE L_EF45
 *   ; first-frame init:
 *   INC $EE / $8F=$0E / $F1=$08 / $F5=$00 / $F6=$00 / $F0=$00
 *   $FC=$FB / Y=6 / $ED=($E7),Y / $EF=($EF&$03)
 * L_EF45:
 *   LDA $F0 / BNE L_EF6E
 *     DEC $F1 / BEQ L_EF63
 *     $F7 = -$F1   ($F1 EOR $FF + 1)
 *     JSR EFF1 / JSR CF08 / BCS L_EF63
 *     $FB=$0A / RTS
 *   L_EF63: $EF |= $80 / $F0=$01 / RTS
 * L_EF6E:
 *   INC $F0 / $F7 = ($F0>>1)+2
 *   JSR EFF1 / JSR CF08 / BCS enemy_drop_choose
 *   $FB=$0A / RTS
 * enemy_drop_choose: pick item index X by resources, then rng, then money split.
 * item_spawn_setup: $EE = X+2 / $ED = (X<<2)|$81 / $EF=$01 / $FB=$FC
 *   $F3=$F0 / $F0=$00 / $F1=$00 / JSR F179 / RTS
 */
#include "ram.h"
#include "regs.h"

void rng_update(Regs *r);
void sub_CF08(Regs *r);
void sub_EFF1(Regs *r);
void sub_F179(Regs *r);

static const u8 drop_item_table[9] =
    { 0x03,0x03,0x03,0x03,0x04,0x04,0x05,0x06,0x07 };

static void item_spawn_setup(Regs *r, u8 x)
{
    RAM8(0xEE) = (u8)(x + 2);            /* TXA / CLC / ADC #$02 */
    RAM8(0xED) = (u8)((x << 2) | 0x81);  /* TXA / ASL / ASL / ORA #$81 */
    RAM8(0xEF) = 0x01;
    RAM8(0xFB) = RAM8(0xFC);
    RAM8(0xF3) = 0xF0;
    RAM8(0xF0) = 0x00;
    RAM8(0xF1) = 0x00;
    sub_F179(r);
}

void sub_EF1C(Regs *r)
{
    if ((RAM8(0xEE) & 0x7F) == 0) {      /* first-frame init */
        RAM8(0xEE) = (u8)(RAM8(0xEE) + 1);
        RAM8(0x8F) = 0x0E;
        RAM8(0xF1) = 0x08;
        RAM8(0xF5) = 0x00;
        RAM8(0xF6) = 0x00;
        RAM8(0xF0) = 0x00;
        RAM8(0xFC) = RAM8(0xFB);
        {
            u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
            RAM8(0xED) = RAM8((u16)(ptr + 6));
        }
        RAM8(0xEF) = (u8)(RAM8(0xEF) & 0x03);
    }

    if (RAM8(0xF0) == 0) {
        RAM8(0xF1) = (u8)(RAM8(0xF1) - 1);   /* DEC $F1 */
        if (RAM8(0xF1) == 0)                  /* BEQ L_EF63 */
            goto L_EF63;
        RAM8(0xF7) = (u8)(0 - RAM8(0xF1));    /* EOR #$FF / CLC / ADC #$01 */
        sub_EFF1(r);
        sub_CF08(r);
        if (r->c) goto L_EF63;
        RAM8(0xFB) = RAM8(0x0A);
        return;
    L_EF63:
        RAM8(0xEF) = (u8)(RAM8(0xEF) | 0x80);
        RAM8(0xF0) = 0x01;
        return;
    }

    /* L_EF6E */
    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
    RAM8(0xF7) = (u8)((RAM8(0xF0) >> 1) + 2);
    sub_EFF1(r);
    sub_CF08(r);
    if (!r->c) {
        RAM8(0xFB) = RAM8(0x0A);
        return;
    }

    /* enemy_drop_choose */
    {
        u8 x = 0x00;
        if (health < 0x14) { item_spawn_setup(r, x); return; }
        x = 0x01;
        if (magic < 0x1E) { item_spawn_setup(r, x); return; }
        x = 0x04;
        if (keys < 0x02) { item_spawn_setup(r, x); return; }

        r->a = 0x14;
        rng_update(r);
        if (r->a >= 0x09) {
            /* drop_money_chooser */
            x = 0x00;
            if (health < magic) {           /* BCC L_EFBE */
                if (health < gold) { item_spawn_setup(r, x); return; }
                x = 0x02;                   /* L_EFC2 */
                item_spawn_setup(r, x);
                return;
            }
            x = 0x01;
            if (magic < gold) { item_spawn_setup(r, x); return; }
            x = 0x02;                       /* JMP L_EFC2 */
            item_spawn_setup(r, x);
            return;
        }
        x = drop_item_table[r->a];
        item_spawn_setup(r, x);
        return;
    }
}
