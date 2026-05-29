/* $C9A9: copy 768 bytes from pointer ($75/$76) into $0500-$07FF.
 *   LDA $75 / STA $77 / LDA $76 / STA $78        ; ptr = $75/$76
 *   LDY #$00
 * L_C9B3: LDA ($77),Y / STA $0500,Y / INY / BNE  ; 256 bytes -> $0500
 *   INC $78
 * L_C9BD: LDA ($77),Y / STA $0600,Y / INY / BNE  ; 256 bytes -> $0600
 *   INC $78
 * L_C9C7: LDA ($77),Y / STA $0700,Y / INY / BNE  ; 256 bytes -> $0700
 *   INC $78 / RTS
 * The pointer high byte ($78) is incremented each block so all three loops read
 * a contiguous 768-byte source. $77/$78 are left modified (low=$75, high=$76+3).
 */
#include "ram.h"
#include "regs.h"

void sub_C9A9(Regs *r)
{
    u8 lo, hi;
    u16 ptr;
    int i;

    RAM8(0x77) = RAM8(0x75);
    RAM8(0x78) = RAM8(0x76);
    lo = RAM8(0x77);
    hi = RAM8(0x78);

    /* block to $0500 */
    ptr = (u16)(lo | (hi << 8));
    for (i = 0; i < 256; i++)
        RAM8((u16)(0x0500 + i)) = RAM8((u16)(ptr + i));
    hi++;
    RAM8(0x78) = hi;

    /* block to $0600 */
    ptr = (u16)(lo | (hi << 8));
    for (i = 0; i < 256; i++)
        RAM8((u16)(0x0600 + i)) = RAM8((u16)(ptr + i));
    hi++;
    RAM8(0x78) = hi;

    /* block to $0700 */
    ptr = (u16)(lo | (hi << 8));
    for (i = 0; i < 256; i++)
        RAM8((u16)(0x0700 + i)) = RAM8((u16)(ptr + i));
    hi++;
    RAM8(0x78) = hi;

    r->y = 0; /* Y wrapped to 0 on final BNE fallthrough */
}
