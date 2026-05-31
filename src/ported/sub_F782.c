/* $F782 sub_F782 — per-frame "active scroll/build job" tick.
 *
 *   LDA $0491 / BNE + / RTS            ; no active job -> return
 * + LDA #$90 / STA $E5 / LDA #$04 / STA $E6   ; pointer $E5/$E6 = $0490
 *   JSR L_E98F                          ; copy 16 bytes $0490.. -> $00ED..
 *   DEC $F3 / BNE L_F7F7                ; counter still running -> sub_F7F7 tick
 *   LDA $ED / AND #$01 / BNE L_F7AA     ; finished: odd phase -> L_F7AA finalize
 *   LDA $FB / AND #$0F / ORA $F9 / BEQ L_F7AA ; nothing left -> L_F7AA finalize
 *   INC $F3 / JMP L_F7F7                ; otherwise restart counter, sub_F7F7 tick
 *
 * L_F7AA (finalize): $EE=0; if $F0!=0 build a tile via CA54 pointer + maybe a
 * bank-9 nametable far-call; then L_F896 copies $00ED.. back through $E5/$E6.
 *
 * The L_F7F7 jumps are tail-calls to the ported sub_F7F7 (its entry IS L_F7F7).
 */
#include "ram.h"
#include "regs.h"

void sub_E98F(Regs *r);
void sub_E99A(Regs *r);
void sub_F7F7(Regs *r);
void sub_CA54(Regs *r);
void farcall_bank_09_r7(Regs *r);

void sub_F782(Regs *r)
{
    if (RAM8(0x0491) == 0)              /* LDA $0491 / BNE + / RTS */
        return;

    RAM8(0xE5) = 0x90;                  /* pointer $E5/$E6 = $0490 */
    RAM8(0xE6) = 0x04;
    sub_E98F(r);                        /* copy $0490.. -> $00ED.. */

    RAM8(0xF3) = (u8)(RAM8(0xF3) - 1);  /* DEC $F3 */
    if (RAM8(0xF3) != 0) {              /* BNE L_F7F7 */
        sub_F7F7(r);
        return;
    }

    /* $F3 == 0 */
    if ((RAM8(0xED) & 0x01) == 0) {     /* LDA $ED / AND #$01 / BNE L_F7AA */
        /* even phase: check whether anything remains */
        if ((u8)((RAM8(0xFB) & 0x0F) | RAM8(0xF9)) != 0) {  /* BEQ L_F7AA */
            RAM8(0xF3) = (u8)(RAM8(0xF3) + 1);   /* INC $F3 */
            sub_F7F7(r);                          /* JMP L_F7F7 */
            return;
        }
        /* fall through to L_F7AA */
    }

    /* L_F7AA: finalize */
    RAM8(0xEE) = 0x00;
    if (RAM8(0xF0) != 0) {              /* LDA $F0 / BNE L_F7B5 (else JMP L_F896) */
        u16 ptr;
        u8 diff;
        RAM8(0x0C) = RAM8(0xFA);
        RAM8(0x0D) = RAM8(0xFB);
        sub_CA54(r);                    /* mutates $0C/$0D, builds $10/$11 */
        ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
        RAM8(ptr) = RAM8(0xF0);         /* LDY #$00 / STA ($0C),Y */

        diff = (u8)(RAM8(0xFA) - RAM8(0x7C));   /* SEC / SBC scroll_x_tile ($7C) */
        /* CMP #$11 / BCC L_F7D3 ; CMP #$FE / BCC L_F7F4 */
        if (diff < 0x11 || diff >= 0xFE) {       /* L_F7D3 path */
            u8 fa = RAM8(0xFA);
            RAM8(0x0C) = fa;
            RAM8(0x16) = (u8)((fa << 1) & 0x1F);          /* vram_dst_lo */
            RAM8(0x17) = (u8)((RAM8(0xFA) & 0x10) >> 2);  /* vram_dst_hi */
            RAM8(0x16) = (u8)(0x00 + RAM8(0x16));
            RAM8(0x17) = (u8)(0x20 + RAM8(0x17));
            farcall_bank_09_r7(r);
        }
        /* L_F7F4 -> fall to L_F896 */
    }

    /* L_F896: JSR L_E99A — copy $00ED.. back through $E5/$E6 */
    sub_E99A(r);
}
