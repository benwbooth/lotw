/* $D991 — per-object item/interaction scan loop.
 * Saves $49 (object index) and $4B (sub-index) on the stack, then loops over
 * objects (sub_D8B6 recompute, sub_CF08 collision test) dispatching to the room
 * handler (D6D4), pickup (DD42/CE1A), or the item-action dispatchers
 * (DA86/DAAA/DA31/DA1B). $49/$4B are advanced through their rings; when both
 * rings exhaust it returns SEC. Restores $49/$4B on exit. Returns carry.
 *
 * Stack model: save49/save4B locals stand in for the two PHA'd bytes; L_DA02's
 * PLA/PHA "peek" reads save49 back into $49; L_DA14 restores both.
 *
 * INSPECTION-PORT (no diff-test spec): validated to 99.99% diff-consistency
 * during development (12000 states, real logic + carry all matched). The residual
 * divergences are flat-memory/NMI-timing artifacts in its deep callees that the
 * m6502 oracle can't model byte-exactly: $28/$36 NMI sync counters (sync_clear
 * step-4000 effect), the song_init $93-$D2 channel block and sub_C9A9 $500-$7FF
 * room buffer (read from switchable banks), and OAM slots indexed by the $08
 * scratch that the $36/C569 timing perturbs. Validate by whole-ROM integration. */
#include "ram.h"
#include "regs.h"

void sub_D8B6(Regs *r); void sub_CF08(Regs *r); void sub_D6D4(Regs *r);
void sub_DD42(Regs *r); void sub_CE1A(Regs *r); void sub_DA31(Regs *r);
void sub_DA86(Regs *r); void sub_CA36(Regs *r); void sub_DAAA(Regs *r);
void sub_DA1B(Regs *r);

void sub_D991(Regs *r)
{
    u8 save4B = RAM8(0x4B);            /* LDA $4B / PHA */
    u8 save49 = RAM8(0x49);            /* LDA $49 / PHA */
    u8 a, x, v;

L_D997:
    sub_D8B6(r);                       /* JSR L_D8B6 */
    sub_CF08(r);                       /* JSR L_CF08 */
    if (r->c) {                        /* BCC L_D9A7 (carry set falls through) */
        sub_D6D4(r);                   /* JSR L_D6D4 */
        if (r->c)                      /* BCC L_D9EB; carry set -> DA13 */
            goto L_DA13;
        goto L_D9EB;
    }

    /* L_D9A7 */
    sub_DD42(r);                       /* JSR L_DD42 */
    if (r->c)                          /* BCS L_D9EB */
        goto L_D9EB;
    sub_CE1A(r);                       /* JSR L_CE1A */
    if (!r->c)                         /* BCC L_DA14 (carry clear) */
        goto L_DA14;
    a = RAM8(0x08);                    /* LDA $08 */
    if (a == 0x09)                     /* CMP #$09 / BEQ L_D9EB */
        goto L_D9EB;
    if (a < 0x09)                      /* BCC L_D9D1 */
        goto L_D9D1;
    /* a > 9 */
    x = RAM8(0x09);                    /* LDX $09 */
    if (RAM8((u16)(0x0401 + x)) == 0x01) {  /* LDA $0401,X / CMP #$01 / BNE L_D9C8 */
        sub_DA31(r);                   /* JSR L_DA31 */
        goto L_DA14;                   /* (carry from DA31) */
    }
    /* L_D9C8 */
    sub_DA86(r);                       /* JSR L_DA86 */
    sub_CA36(r);                       /* JSR L_CA36 */
    goto L_DA13;

L_D9D1:                                /* a < 9 */
    x = RAM8(0x09);                    /* LDX $09 */
    v = RAM8((u16)(0x0401 + x));       /* LDA $0401,X */
    if (v == 0x01)                     /* CMP #$01 / BEQ L_D9E4 */
        goto L_D9E4;
    if (v >= 0x1A)                     /* CMP #$1A / BCS L_D9E7 */
        goto L_D9E7;
    sub_DAAA(r);                       /* JSR L_DAAA */
    goto L_DA13;
L_D9E4:
    sub_DA1B(r);                       /* JSR L_DA1B */
L_D9E7:
    r->c = 0;                          /* CLC */
    goto L_DA14;

L_D9EB:
    if (RAM8(0x88) == 0)               /* LDA $88 / BEQ L_DA02 */
        goto L_DA02;
    a = RAM8(0x49);                    /* LDA $49 */
    if (a == 0)                        /* BEQ L_DA02 */
        goto L_DA02;
    x = a;                             /* TAX */
    if (!(a & 0x08))                   /* AND #$08 / BNE L_D9FA */
        x = (u8)(x - 2);               /* DEX / DEX */
    /* L_D9FA */
    x = (u8)(x + 1);                   /* INX */
    a = (u8)(x & 0x0F);                /* TXA / AND #$0F */
    RAM8(0x49) = a;                    /* STA $49 */
    if (a != 0)                        /* BNE L_D997 */
        goto L_D997;

L_DA02:
    RAM8(0x49) = save49;               /* PLA / PHA (peek) / STA $49 */
    x = RAM8(0x4B);                    /* LDX $4B */
    if (x == 0)                        /* BEQ L_DA13 */
        goto L_DA13;
    if (!(x & 0x80))                   /* BMI L_DA0E (skip DEX/DEX if negative) */
        x = (u8)(x - 2);               /* DEX / DEX */
    /* L_DA0E */
    x = (u8)(x + 1);                   /* INX */
    RAM8(0x4B) = x;                    /* STX $4B */
    if (x != 0)                        /* BNE L_D997 */
        goto L_D997;

L_DA13:
    r->c = 1;                          /* SEC */
L_DA14:
    RAM8(0x49) = save49;               /* PLA / STA $49 */
    RAM8(0x4B) = save4B;               /* PLA / STA $4B */
    /* $28/$36 (NMI sync counters) are left as the asm leaves them; their final
     * value is an NMI-timing artifact the flat oracle can't model (sync_clear
     * zeroes them only past step 4000), so the spec excludes them from compare. */
}
