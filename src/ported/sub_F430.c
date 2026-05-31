/* $F430:
 *   LDA $EE / AND #$7F / BNE F473
 *     ; init branch (only when ($EE & $7F)==0):
 *     LDA #$18/STA $8F / LDA #$FF/STA $90 / LDX #$03 / JSR C540
 *     LDA #$02/STA $36 / JSR C135 / LDX #$03 / JSR C540
 *     LDA #$05/STA $36 / JSR C135 / LDX #$03 / JSR C540
 *     INC $EE / LDA #$02/STA $8F / LDA #$0F/STA $F1
 *     LDA #$00 / STA $F5/$F6/$F0 / LDA $FB/STA $FC
 * F473: LDA $F0 / BNE F49E
 *     DEC $F1 / BEQ F493
 *     LDA $F1 / LSR/LSR / EOR #$FF / CLC / ADC #$01 / STA $F7
 *     JSR EFF1 / JSR CF08 / BCS F493
 *     LDA $0A / STA $FB / RTS
 * F493: LDA $EF / ORA #$80 / STA $EF / LDA #$01 / STA $F0 / RTS
 * F49E: INC $F0 / LDA $F0 / LSR/LSR / CLC / ADC #$01 / STA $F7
 *     JSR EFF1 / JSR CF08 / BCS F4B6
 *     LDA $0A / STA $FB / RTS
 * F4B6: LDA #$00/STA $EE / LDA #$F0/STA $F3 / LDA #$01/STA $EB / RTS
 */
#include "ram.h"
#include "regs.h"

void sub_C540(Regs *r);
void sub_C135(Regs *r);
void sub_CF08(Regs *r);
void sub_EFF1(Regs *r);

void sub_F430(Regs *r)
{
    if ((RAM8(0xEE) & 0x7F) == 0) {
        RAM8(0x8F) = 0x18;
        RAM8(0x90) = 0xFF;
        r->x = 0x03;
        sub_C540(r);

        RAM8(0x36) = 0x02;
        sub_C135(r);
        r->x = 0x03;
        sub_C540(r);

        RAM8(0x36) = 0x05;
        sub_C135(r);
        r->x = 0x03;
        sub_C540(r);

        RAM8(0xEE) = (u8)(RAM8(0xEE) + 1);
        RAM8(0x8F) = 0x02;
        RAM8(0xF1) = 0x0F;
        RAM8(0xF5) = 0x00;
        RAM8(0xF6) = 0x00;
        RAM8(0xF0) = 0x00;
        RAM8(0xFC) = RAM8(0xFB);
    }

    if (RAM8(0xF0) == 0) {
        RAM8(0xF1) = (u8)(RAM8(0xF1) - 1);
        if (RAM8(0xF1) == 0) {
            /* F493 */
            RAM8(0xEF) = (u8)(RAM8(0xEF) | 0x80);
            RAM8(0xF0) = 0x01;
            return;
        }
        {
            u8 a = (u8)(RAM8(0xF1) >> 2);
            a = (u8)((a ^ 0xFF) + 1);
            RAM8(0xF7) = a;
        }
        sub_EFF1(r);
        sub_CF08(r);
        if (r->c) {
            /* F493 */
            RAM8(0xEF) = (u8)(RAM8(0xEF) | 0x80);
            RAM8(0xF0) = 0x01;
            return;
        }
        RAM8(0xFB) = RAM8(0x0A);
        return;
    }

    /* F49E */
    RAM8(0xF0) = (u8)(RAM8(0xF0) + 1);
    RAM8(0xF7) = (u8)((RAM8(0xF0) >> 2) + 1);
    sub_EFF1(r);
    sub_CF08(r);
    if (r->c) {
        /* F4B6 */
        RAM8(0xEE) = 0x00;
        RAM8(0xF3) = 0xF0;
        RAM8(0xEB) = 0x01;
        return;
    }
    RAM8(0xFB) = RAM8(0x0A);
}
