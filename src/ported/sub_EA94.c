/* $EA94 — boss-state dispatcher ($EAAD jump-table). Reads the boss state byte
 * at ($E7)+8, clamps it to <9 (else 0), scales *2 and indexes the LE pointer
 * table at $EAAD, stores the selected handler address into $0E/$0F, then does
 * JMP ($000E). Ported as a switch over the state index calling each handler.
 *
 *   LDY #$08 / LDA ($E7),Y / CMP #$09 / BCC k / LDA #$00
 * k: ASL A / TAX / LDA $EAAD,X / STA $0E / LDA $EAAE,X / STA $0F / JMP ($000E)
 *
 * table @ $EAAD: EAFD, EB69, EB90, EBD8, EC76, ECA8, ED2A, ED6F, ED9F
 */
#include "ram.h"
#include "regs.h"

void sub_EAFD(Regs *r);
void sub_EB69(Regs *r);
void sub_EB90(Regs *r);
void sub_EBD8(Regs *r);
void sub_EC76(Regs *r);
void sub_ECA8(Regs *r);
void sub_ED2A(Regs *r);
void sub_ED6F(Regs *r);
void sub_ED9F(Regs *r);

/* boss_state_dispatch_table @ $EAAD (little-endian handler addresses) */
static const u16 boss_state_table[9] = {
    0xEAFD, 0xEB69, 0xEB90, 0xEBD8, 0xEC76, 0xECA8, 0xED2A, 0xED6F, 0xED9F,
};

void sub_EA94(Regs *r)
{
    u16 ptr = (u16)(RAM8(0xE7) | (RAM8(0xE8) << 8));
    u8 idx = RAM8((u16)(ptr + 8));      /* LDY #$08 / LDA ($E7),Y */
    if (idx >= 0x09)                    /* CMP #$09 / BCC ; else clamp to 0 */
        idx = 0x00;

    /* STA $0E / STA $0F : record selected handler address before the jump */
    RAM8(0x0E) = (u8)(boss_state_table[idx] & 0xFF);
    RAM8(0x0F) = (u8)(boss_state_table[idx] >> 8);

    switch (idx) {
    case 0: sub_EAFD(r); break;
    case 1: sub_EB69(r); break;
    case 2: sub_EB90(r); break;
    case 3: sub_EBD8(r); break;
    case 4: sub_EC76(r); break;
    case 5: sub_ECA8(r); break;
    case 6: sub_ED2A(r); break;
    case 7: sub_ED6F(r); break;
    case 8: sub_ED9F(r); break;
    }
}
