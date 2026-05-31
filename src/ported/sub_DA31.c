/* $DA31: "use a key" / open-door sequence.
 *   JSR E86F                       ; consume a key; C=1 if none left
 *   BCC DA3C
 *   LDA #$06 / STA $8F / CLC / RTS ; no key: set result code $8F=6, return C=0
 * DA3C:
 *   LDY #$0A / LDA ($77),Y / CMP #$08 / BCS DA49
 *   LDY #$00 / STY $04A2           ; door type < 8: clear $04A2
 * DA49:
 *   PHA / CLC / ADC #$02 / STA $04A1 / PLA
 *   ASL A / ASL A / CLC / ADC #$81 / STA $04A0
 *   LDA #$1F / STA $8F
 *   JSR C2B1
 *   LDA $85 / PHA / LDA #$00 / STA $85 / JSR C1D8     ; sprite refresh w/ $85=0
 *   LDA $8E / PHA / LDA #$0E / STA $8E / JSR song_init ; play door SFX song $0E
 *   LDA #$78 / STA $36 / JSR C135                     ; ~120-frame wait (NMI-synced)
 *   PLA / STA $8E / JSR song_init                     ; restore previous song
 *   PLA / STA $85
 *   SEC / RTS
 *
 * Linear (the C135 frame-wait is a single call; its $36 spin is left 0 by
 * sync_clear). Reads door descriptor through pointer $77/$78 indexed by $0A.
 */
#include "ram.h"
#include "regs.h"

void sub_E86F(Regs *r);
void sub_C2B1(Regs *r);
void sub_C1D8(Regs *r);
void sub_C135(Regs *r);
void song_init(Regs *r);

void sub_DA31(Regs *r)
{
    u16 ptr;
    u8 door, saved85, saved8E;

    sub_E86F(r);
    if (r->c) {                 /* no key left */
        RAM8(0x8F) = 0x06;
        r->c = 0;               /* CLC */
        return;
    }

    ptr = (u16)(RAM8(0x77) | (RAM8(0x78) << 8));
    door = RAM8((u16)(ptr + 0x0A));        /* LDA ($77),Y, Y=$0A */
    if (door < 0x08)                        /* CMP #$08 / BCS skips */
        RAM8(0x04A2) = 0;

    RAM8(0x04A1) = (u8)(door + 0x02);       /* CLC / ADC #$02 */
    RAM8(0x04A0) = (u8)(((door << 2) & 0xFF) + 0x81); /* ASL/ASL / ADC #$81 */
    RAM8(0x8F) = 0x1F;

    sub_C2B1(r);

    saved85 = RAM8(0x85);
    RAM8(0x85) = 0;
    sub_C1D8(r);

    saved8E = RAM8(0x8E);
    RAM8(0x8E) = 0x0E;
    r->a = 0x0E;                            /* song_init reads $8E (its input) */
    song_init(r);

    RAM8(0x36) = 0x78;
    sub_C135(r);                            /* frame wait; $36 -> 0 (sync_clear) */

    RAM8(0x8E) = saved8E;
    r->a = saved8E;
    song_init(r);

    RAM8(0x85) = saved85;

    r->c = 1;                               /* SEC */
}
