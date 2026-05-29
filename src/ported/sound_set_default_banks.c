/* $FD74:  LDX #$06 / LDY #$0A / STX MMC3_BANK_SELECT ($8000) / STY MMC3_BANK_DATA ($8001)
 *         INX / INY / STX $8000 / STY $8001 / RTS
 * Programs MMC3 R6 (bank $0A) and R7 (bank $0B) for the default CHR/PRG mapping.
 * Pure hardware side effects; X ends $07, Y ends $0B. */
#include "ram.h"
#include "regs.h"

void sound_set_default_banks(Regs *r)
{
    u8 x = 0x06, y = 0x0A;
    REG_W(0x8000, x);              /* MMC3_BANK_SELECT */
    REG_W(0x8001, y);             /* MMC3_BANK_DATA */
    x = (u8)(x + 1);
    y = (u8)(y + 1);
    REG_W(0x8000, x);
    REG_W(0x8001, y);
    r->x = x;
    r->y = y;
}
