/* $DBB7:
 *   LDX #$09 / LDY #$00
 * DBBB: LDA $0401,Y / CMP #$01 / BNE DBC7 / LDA #$80 / STA $0401,Y
 * DBC7: TYA / CLC / ADC #$10 / TAY / DEX / BNE DBBB
 *   LDA #$18 / STA $8F / LDA #$FF / STA $90 / LDX #$02 / JSR C540 / RTS
 * Scans 9 slots at $0401 stride $10: any ==1 -> $80. Then $8F=$18,$90=$FF,C540(x=2).
 */
#include "ram.h"
#include "regs.h"

void sub_C540(Regs *r);

void sub_DBB7(Regs *r)
{
    u8 x = 0x09;
    u8 y = 0x00;
    do {
        if (RAM8((u16)(0x0401 + y)) == 0x01)
            RAM8((u16)(0x0401 + y)) = 0x80;
        y = (u8)(y + 0x10);
    } while (--x != 0);
    RAM8(0x8F) = 0x18;
    RAM8(0x90) = 0xFF;
    r->x = 0x02;
    sub_C540(r);
}
