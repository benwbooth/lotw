/* $DF90 — auto-step movement decode. If a Y-move is pending ($49!=0), inspect
 * the player's sub-tile Y (player_y & $0F) and the held buttons to choose an
 * X nudge ($4B:$4C = +1 or -1) then run the object scan (D991); else if an
 * X-move is pending ($4B!=0), inspect player_x_fine to choose a Y nudge
 * ($49:$4A). When a move is committed it tail-calls D991 (returning its carry);
 * otherwise it returns SEC (no move). Clears the pending flag it consumed.
 *
 * INSPECTION-PORT (no diff-test spec): the committed path tail-calls D991, which
 * is itself integration-verified (vblank/bank artifacts) — see sub_D991. */
#include "ram.h"
#include "regs.h"

void sub_D991(Regs *r);

void sub_DF90(Regs *r)
{
    u8 v49 = RAM8(0x49);              /* LDA $49 / PHP */
    RAM8(0x49) = 0x00;               /* STA $49 */
    RAM8(0x4A) = 0x00;               /* STA $4A */
    if (v49 == 0)                    /* PLP / BEQ L_DFCF */
        goto L_DFCF;

    /* $49 != 0 — Y-move pending: pick X nudge from sub-tile Y + buttons */
    {
        u8 a = (u8)(RAM8(0x45) & 0x0F);   /* player_y & $0F */
        if (a == 0)                  /* BEQ L_E00D */
            goto L_E00D;
        if (a < 0x06) {              /* CMP #$06 / BCC L_DFBE */
            if (RAM8(0x20) & 0x04)   /* L_DFBE */
                goto L_E00D;
            RAM8(0x4B) = 0xFF;
            RAM8(0x4C) = 0xFF;
            goto L_E009;
        }
        if (a >= 0x0B) {             /* CMP #$0B / BCS L_DFAD */
            if (RAM8(0x20) & 0x08)   /* L_DFAD */
                goto L_E00D;
            RAM8(0x4B) = 0x01;
            RAM8(0x4C) = 0x00;
            goto L_E009;
        }
        goto L_E00D;                 /* [6,$0B): JMP L_E00D */
    }

L_DFCF:
    {
        u8 v4B = RAM8(0x4B);         /* LDA $4B / PHP */
        RAM8(0x4B) = 0x00;           /* STA $4B */
        RAM8(0x4C) = 0x00;           /* STA $4C */
        if (v4B == 0)                /* PLP / BEQ L_E00D */
            goto L_E00D;
        u8 a = RAM8(0x43);           /* player_x_fine */
        if (a == 0)                  /* BEQ L_E00D */
            goto L_E00D;
        if (a < 0x06) {              /* CMP #$06 / BCC L_DFFB */
            if (RAM8(0x20) & 0x01)   /* L_DFFB */
                goto L_E00D;
            RAM8(0x49) = 0x0F;
            RAM8(0x4A) = 0xFF;
            goto L_E009;
        }
        if (a >= 0x0B) {             /* CMP #$0B / BCS L_DFEA */
            if (RAM8(0x20) & 0x02)   /* L_DFEA */
                goto L_E00D;
            RAM8(0x49) = 0x01;
            RAM8(0x4A) = 0x00;
            goto L_E009;
        }
        goto L_E00D;                 /* [6,$0B): JMP L_E00D */
    }

L_E009:
    sub_D991(r);                     /* JSR L_D991 (carry = its return) */
    return;
L_E00D:
    r->c = 1;                        /* SEC */
}
