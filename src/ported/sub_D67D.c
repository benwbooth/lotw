/* $D67D:
 *   JSR C375 / LDA #$00/STA $85 / JSR C1D8 / JSR C234
 *   LDA $7C / CMP #$21 / BCC + / LDA #$20 / + STA $7C   ; clamp scroll_x_tile to $20
 *   JSR C76C
 *   LDA $7C / CLC / ADC #$10 / STA $7C / JSR C76C
 *   LDA #$01 / STA $08
 * D6A5: LDX #$0C
 * D6A7: LDA $1C / CLC / ADC $08 / STA $1C / BCC +
 *         LDA $1D / EOR #$01 / STA $1D
 *     + LDA #$FF / JSR queue_ppu_job_and_wait / DEX / BNE D6A7
 *   INC $08 / LDX $08 / CPX #$20 / BCC D6A5
 *   LDA #$18/STA $8F / LDA #$FF/STA $90 / LDX #$08 / JSR C540 / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_C375(Regs *r);
void sub_C1D8(Regs *r);
void sub_C234(Regs *r);
void sub_C76C(Regs *r);
void sub_C540(Regs *r);
void queue_ppu_job_and_wait(Regs *r);

void sub_D67D(Regs *r)
{
    u8 outer;

    sub_C375(r);
    RAM8(0x85) = 0x00;
    sub_C1D8(r);
    sub_C234(r);

    if (RAM8(0x7C) >= 0x21)
        RAM8(0x7C) = 0x20;
    sub_C76C(r);

    RAM8(0x7C) = (u8)(RAM8(0x7C) + 0x10);
    sub_C76C(r);

    RAM8(0x08) = 0x01;
    do {
        u8 x = 0x0C;
        do {
            u16 sum = (u16)(RAM8(0x1C) + RAM8(0x08));
            RAM8(0x1C) = (u8)sum;
            if (sum & 0x100)
                RAM8(0x1D) = (u8)(RAM8(0x1D) ^ 0x01);
            r->a = 0xFF;
            queue_ppu_job_and_wait(r);
        } while (--x != 0);
        RAM8(0x08) = (u8)(RAM8(0x08) + 1);
        outer = RAM8(0x08);
    } while (outer < 0x20);

    RAM8(0x8F) = 0x18;
    RAM8(0x90) = 0xFF;
    r->x = 0x08;
    sub_C540(r);
}
