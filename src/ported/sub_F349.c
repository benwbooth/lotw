/* $F349 — spawn/init a boss. Reads the boss descriptor via the ($E7) pointer
 * (low/high tile at +2/+3, type at +5, life at +4), validates/places it through
 * F275 (RTS on failure), then sets the boss state ($F9/$FA/$FB position, $EE/$ED/
 * $EF flags, boss_life into $0415/$0425/$0435), and far-calls the meter sprite
 * load ($A7E1) and meter draw ($CB53).
 *
 * farcall_0C0D side effects modelled: save mmc3_r6/r7_shadow ($30/$31) to $32/$33,
 * map banks 12/13 during the target, restore, leave select_shadow $25=$06. */
#include "ram.h"
#include "regs.h"

void sub_F275(Regs *r); void sub_A7E1(Regs *r); void sub_CB53(Regs *r);

static void farcall_0C0D(Regs *r, void (*target)(Regs *))
{
    u8 old6 = RAM8(0x30), old7 = RAM8(0x31);
    RAM8(0x32) = old6; RAM8(0x33) = old7;
    RAM8(0x30) = 0x0C; RAM8(0x31) = 0x0D; RAM8(0x25) = 0x07; NES_PRG_SYNC();
    target(r);                                  /* JMP ($000E) */
    RAM8(0x31) = old7; RAM8(0x30) = old6; RAM8(0x25) = 0x06; NES_PRG_SYNC();
}

void sub_F349(Regs *r)
{
    u16 e7 = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));

    RAM8(0x2E) = 0x3D;                          /* mmc3_r4_shadow */
    RAM8(0x0A) = RAM8((u16)(e7 + 3));           /* LDY #$03 / LDA ($E7),Y / STA $0A */
    RAM8(0x0F) = RAM8((u16)(e7 + 2));           /* DEY / LDA ($E7),Y / STA $0F */
    RAM8(0x0E) = 0x00;
    RAM8(0x0B) = 0x00;
    sub_F275(r);                                /* JSR L_F275 */
    if (r->c)                                   /* BCC L_F364; SEC -> RTS */
        return;

    /* L_F364 */
    RAM8(0xF9) = RAM8(0x0E);
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xFB) = RAM8(0x0A);
    RAM8(0xF1) = 0x00; RAM8(0xF0) = 0x00; RAM8(0xF4) = 0x00;
    RAM8(0xEE) = 0x01;
    RAM8(0xED) = 0x81;
    RAM8(0xEF) = 0x02;
    RAM8(0xF8) = RAM8((u16)(e7 + 5));           /* type */
    {
        u8 bl = RAM8((u16)(e7 + 4));            /* life */
        RAM8(0xF2) = bl;                        /* boss_life */
        RAM8(0x0415) = bl; RAM8(0x0425) = bl; RAM8(0x0435) = bl;
    }

    RAM8(0x0E) = 0xE1; RAM8(0x0F) = 0xA7;       /* far-call $A7E1 (meter load) */
    farcall_0C0D(r, sub_A7E1);
    RAM8(0x0E) = 0x53; RAM8(0x0F) = 0xCB;       /* far-call $CB53 (meter draw) */
    farcall_0C0D(r, sub_CB53);
}
