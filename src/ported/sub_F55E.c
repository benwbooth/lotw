/* $F55E — build the 3-row boss sprite block in OAM ($0401-$043F) from the boss
 * state ($F9-$FC position, $EE/$ED/$EF flags, boss_life), pick the displayed
 * life as min(boss_life,$0415,$0425,$0435) -> $0405, set per-row attributes from
 * $ED, conditionally swap the sprite-table attribute bytes by $EF bits, then
 * far-call the meter draw ($CB53). Pure OAM/data + CB53. */
#include "ram.h"
#include "regs.h"

void sub_CB53(Regs *r);

static void swap(u16 a, u16 b) { u8 t = RAM8(a); RAM8(a) = RAM8(b); RAM8(b) = t; }

void sub_F55E(Regs *r)
{
    RAM8(0x041F) = RAM8(0xFC); RAM8(0x042F) = RAM8(0xFC); RAM8(0x043F) = RAM8(0xFC);
    {
        u8 fb = RAM8(0xFB);
        RAM8(0x041E) = fb;
        RAM8(0x042E) = (u8)(fb + 0x10); RAM8(0x043E) = (u8)(fb + 0x10);   /* CLC/ADC #$10 */
    }
    RAM8(0x041C) = RAM8(0xF9); RAM8(0x042C) = RAM8(0xF9); RAM8(0x043C) = RAM8(0xF9);
    {
        u8 fa = RAM8(0xFA);
        RAM8(0x042D) = fa;                                  /* STX $042D */
        RAM8(0x041D) = (u8)(fa + 1); RAM8(0x043D) = (u8)(fa + 1);   /* INX / STX */
    }
    {
        u8 xv = RAM8(0xEE);                                 /* LDX $EE */
        if (!(xv & 0x80)) {                                 /* BMI L_F59F */
            if ((RAM8(0x0411) | RAM8(0x0421) | RAM8(0x0431)) & 0x80)  /* ORA.. / BPL */
                xv = 0x80;
        }
        RAM8(0x0401) = xv; RAM8(0x0411) = xv; RAM8(0x0421) = xv; RAM8(0x0431) = xv;
    }
    {
        u8 a = RAM8(0xF2);                                  /* boss_life */
        if (a >= RAM8(0x0415)) a = RAM8(0x0415);            /* min(...) */
        if (a >= RAM8(0x0425)) a = RAM8(0x0425);
        if (a >= RAM8(0x0435)) a = RAM8(0x0435);
        RAM8(0x0405) = a;
    }
    {
        u8 ed = RAM8(0xED);                                 /* LDA $ED */
        u8 a = (u8)(ed | 0x04); RAM8(0x0410) = a;           /* ORA #$04 / STA $0410 */
        a = (u8)(a | 0x20); RAM8(0x0430) = a;               /* ORA #$20 / STA $0430 */
        a = (u8)(a & 0xFB); RAM8(0x0420) = a;               /* AND #$FB / STA $0420 */
    }
    {
        u8 ef = RAM8(0xEF);                                 /* LDA $EF */
        RAM8(0x0412) = ef; RAM8(0x0422) = ef; RAM8(0x0432) = ef;
        if (ef & 0x40) {                                    /* AND #$40 / BEQ L_F600 */
            swap(0x0400, 0x0410);                           /* sprite_tables <-> $0410 */
            swap(0x0420, 0x0430);
        }
        if (ef & 0x80) {                                    /* LDA $EF / BPL L_F61C */
            swap(0x0400, 0x0420);                           /* sprite_tables <-> $0420 */
            swap(0x0410, 0x0430);
        }
    }
    /* JSR farcall_bank_0C0D -> $CB53. The dispatcher saves the bank shadows
     * (mmc3_r6/r7_shadow $30/$31 -> $32/$33), maps banks 12/13 ($30=$0C,$31=$0D),
     * runs the target, then restores the shadows and leaves select_shadow $25=$06.
     * Hardware MMC3 register writes ($8000/$8001) are outside compared RAM. */
    {
        u8 old6 = RAM8(0x30), old7 = RAM8(0x31);
        RAM8(0x32) = old6; RAM8(0x33) = old7;
        RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
        RAM8(0x0E) = 0x53; RAM8(0x0F) = 0xCB;
        sub_CB53(r);                                        /* JMP ($000E) */
        RAM8(0x31) = old7; RAM8(0x30) = old6; RAM8(0x25) = 0x06; NES_PRG_SYNC();
    }
}
