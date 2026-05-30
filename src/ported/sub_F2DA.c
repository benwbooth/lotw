/* $F2DA: movement/step dispatcher.
 *   LDA #$00 / STA $F6 ; A=0
 *   LDX $F5 / BEQ L_F30F
 *     STA $F5                      ; $F5=0
 *     LDA $FB / AND #$0F / BEQ L_F347
 *     CMP #$06 / BCC L_F302
 *     CMP #$0B / BCS L_F2F5
 *     JMP L_F347
 *   L_F2F5: LDA $F4 / AND #$08 / BNE L_F347
 *           LDA #$01 / STA $F7 / JMP L_F343
 *   L_F302: LDA $F4 / AND #$04 / BNE L_F347
 *           LDA #$FF / STA $F7 / JMP L_F343
 *   L_F30F: LDX $F7 / BEQ L_F347
 *           STA $F7                ; $F7=0
 *           LDA $F9 / BEQ L_F347
 *           CMP #$06 / BCC L_F335
 *           CMP #$0B / BCS L_F324
 *           JMP L_F347
 *   L_F324: LDA $F4 / AND #$02 / BNE L_F347
 *           LDA #$01 / STA $F5 / LDA #$00 / STA $F6 / JMP L_F343
 *   L_F335: LDA $F4 / AND #$01 / BNE L_F347
 *           LDA #$0F / STA $F5 / LDA #$FF / STA $F6
 *   L_F343: JSR L_F0E1 / RTS       ; returns F0E1 carry
 *   L_F347: SEC / RTS              ; C=1
 */
#include "ram.h"
#include "regs.h"

void sub_F0E1(Regs *r);

void sub_F2DA(Regs *r)
{
    u8 v;
    RAM8(0xF6) = 0x00;

    if (RAM8(0xF5) != 0) {            /* LDX $F5 / BNE (not BEQ F30F) */
        RAM8(0xF5) = 0x00;
        v = RAM8(0xFB) & 0x0F;
        if (v == 0) goto sec_ret;     /* L_F347 */
        if (v < 0x06) {               /* L_F302 */
            if (RAM8(0xF4) & 0x04) goto sec_ret;
            RAM8(0xF7) = 0xFF;
            goto call_f0e1;           /* L_F343 */
        }
        if (v >= 0x0B) {              /* L_F2F5 */
            if (RAM8(0xF4) & 0x08) goto sec_ret;
            RAM8(0xF7) = 0x01;
            goto call_f0e1;           /* L_F343 */
        }
        goto sec_ret;                 /* 6..A -> L_F347 */
    }

    /* L_F30F */
    if (RAM8(0xF7) == 0) goto sec_ret;
    RAM8(0xF7) = 0x00;
    v = RAM8(0xF9);
    if (v == 0) goto sec_ret;
    if (v < 0x06) {                   /* L_F335 */
        if (RAM8(0xF4) & 0x01) goto sec_ret;
        RAM8(0xF5) = 0x0F;
        RAM8(0xF6) = 0xFF;
        goto call_f0e1;               /* fall to L_F343 */
    }
    if (v >= 0x0B) {                  /* L_F324 */
        if (RAM8(0xF4) & 0x02) goto sec_ret;
        RAM8(0xF5) = 0x01;
        RAM8(0xF6) = 0x00;
        goto call_f0e1;               /* L_F343 */
    }
    goto sec_ret;                     /* 6..A -> L_F347 */

call_f0e1:
    sub_F0E1(r);                      /* returns its carry */
    return;

sec_ret:
    r->c = 1;
}
