/* $ACE0 — pick the player metasprite tile/attribute ($56/$57) from the current
 * input/velocity state. Reads buttons $20, y-velocity $4B, x-velocity $49,
 * climb/jump state $4E/$4F. Selects a base tile index into $56 (low bits) and
 * horizontal-flip attribute into $57, or overwrites $56 wholesale.
 * Pure RAM/buttons, several RTS exits. No callees. */
#include "ram.h"
#include "regs.h"

void sub_ACE0(Regs *r)
{
    r->x = 0x09;                                   /* LDX #$09 */

    if ((u8)(RAM8(0x20) & 0xBF) == 0x80) {         /* AND #$BF / CMP #$80 / BEQ L_AD1F */
        RAM8(0x56) = r->x;                         /* L_AD1F: STX $56 */
        return;                                    /* L_AD21: RTS */
    }

    if (RAM8(0x4B) == 0) goto L_AD06;              /* LDA $4B / BEQ L_AD06 */
    if (RAM8(0x4B) & 0x80) {                       /* BMI L_ACFF */
        /* L_ACFF */
        if (RAM8(0x4F) == 0) {                     /* LDA $4F / BEQ L_AD1F */
            RAM8(0x56) = r->x;                      /* L_AD1F */
            return;
        }
        goto L_AD22;                               /* JMP L_AD22 */
    }
    if (RAM8(0x4E) != 0) goto L_AD22;              /* LDA $4E / BNE L_AD22 */
    if ((RAM8(0x20) & 0x04) == 0) goto L_AD06;     /* AND #$04 / BEQ L_AD06 */
    r->x = 0x0D;                                   /* LDX #$0D */
    RAM8(0x56) = r->x;                             /* JMP L_AD1F */
    return;

L_AD06:
    r->x = 0x01;
    r->y = 0x00;
    if (RAM8(0x49) & 0x80) goto L_AD12;            /* LDA $49 / BMI L_AD12 */
    if (RAM8(0x49) == 0) return;                   /* BEQ L_AD21 (RTS) */
    r->y = 0x40;                                   /* LDY #$40 */
L_AD12:
    RAM8(0x08) = r->x;                             /* STX $08 */
    RAM8(0x56) = (u8)((RAM8(0x56) & 0x07) | RAM8(0x08));  /* AND #$07 / ORA $08 */
    RAM8(0x57) = r->y;                             /* STY $57 */
    return;

L_AD22:
    r->x = 0x39;
    r->y = 0x00;
    if (RAM8(0x49) & 0x80) goto L_AD2E;            /* BMI L_AD2E */
    if (RAM8(0x49) == 0) return;                   /* BEQ L_AD21 */
    r->y = 0x40;
L_AD2E:
    RAM8(0x08) = r->x;
    RAM8(0x56) = (u8)((RAM8(0x56) & 0x03) | RAM8(0x08));  /* AND #$03 / ORA $08 */
    RAM8(0x57) = r->y;
}
