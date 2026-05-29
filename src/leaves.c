/* Small leaf routines (verified byte-for-behaviour against the m6502 oracle).
 * Ported from the addressing-mode-accurate disasm/bankfix.s. Names describe the
 * operation; precise game-semantic names follow as systems are understood. */
#include "ram.h"
#include "leaves.h"

/* $E41E:  LDA $F9 / AND #$1F / TAX / RTS  ->  returns X = $F9 & $1F */
u8 sub_E41E(void)
{
    return RAM8(0xF9) & 0x1F;
}

/* $F233:  LDA ($0C),Y / AND #$3F / CMP #$30 / RTS
 * Reads a byte through pointer $0C/$0D indexed by Y; carry = (byte&$3F) >= $30. */
u8 sub_F233(u8 y)
{
    u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    u8 v = RAM8((u16)(ptr + y)) & 0x3F;
    return (u8)(v >= 0x30);
}

/* $FD6B:  LDX $02 / INC $95,X / BNE +2 / INC $96,X / RTS
 * X = $02; 16-bit little-endian increment of the counter at ($95+X):($96+X)
 * (zero-page,X wraps within the page). Returns X. */
u8 inc16_95(void)
{
    u8 x = RAM8(0x02);
    if (++RAM8((0x95 + x) & 0xFF) == 0)
        ++RAM8((0x96 + x) & 0xFF);
    return x;
}
