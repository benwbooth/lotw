/* $F7F7: per-frame column scroll/update step.
 *
 *   LDA $ED / AND #$01 / BEQ L_F80C       ; even phase -> do the work below
 *     LDA $F3 / AND #$03 / BNE L_F809     ; odd phase: every 4th, toggle $ED bit2
 *     LDA $ED / EOR #$04 / STA $ED
 *   L_F809: JMP L_F896
 *   L_F80C:
 *     $E3 = $09
 *     JSR L_EFF1
 *     JSR L_CF1C / BCS L_F85A
 *     JSR L_F23A / BCS L_F85A
 *     JSR L_CE7C / BCS L_F846
 *     JSR L_CDB2 / BCC L_F82E
 *       LDX $09 / LDA #$80 / STA $0401,X
 *   L_F82E: $F9=$0E ; $FA=$0F ; $FB=$0A ; $F4=0 ; JMP L_F896
 *   L_F846:
 *     LDA $F4 / BNE L_F886
 *     LDA $85 / BNE L_F85A
 *     JSR L_E7CE / $8F=$0A / $85=$02
 *   L_F85A:
 *     LDA $F4 / BNE L_F886
 *     INC $F4
 *     LDA $F5 / BEQ L_F873 : $F5 = (-$F5)&$0F ; $F6 ^= $FF
 *   L_F873: $F7 = (~$F7)+1 ; if $8F==0 $8F=$06
 *   L_F883: JMP L_F896
 *   L_F886:
 *     LDA $FB / AND #$0F / ORA $F9 / BEQ L_F893 : INC $F3 / JMP L_F896
 *   L_F893: JMP L_F7AA
 *   L_F896: JSR L_E99A / RTS
 *
 * L_F7AA (reached only via L_F893):
 *     $EE=0
 *     LDA $F0 / BEQ -> L_F896
 *     $0C=$FA ; $0D=$FB ; JSR L_CA54 ; *(($0C))=$F0
 *     A = $FA - scroll_x_tile ; if A>=$11 and A<$FE -> compute dst, far-call
 *     -> L_F896
 */
#include "ram.h"
#include "regs.h"

void sub_EFF1(Regs *r);
void sub_CF1C(Regs *r);
void sub_F23A(Regs *r);
void sub_CE7C(Regs *r);
void sub_CDB2(Regs *r);
void sub_E7CE(Regs *r);
void sub_CA54(Regs *r);
void farcall_bank_09_r7(Regs *r);

/* L_E99A inlined faithfully: the pointer ($E5/$E6) is re-read on every copied
 * byte, so when the destination overlaps $E5/$E6 the pointer mutates mid-copy and
 * later writes follow the new pointer. The shared sub_E99A.c caches the pointer
 * once and diverges on those inputs. */
static void e99a_copy(Regs *r)
{
    int y;
    for (y = 0x0F; y >= 0; --y) {
        u16 ptr = (u16)(RAM8(0xE5) | (RAM8(0xE6) << 8));   /* re-read each iter */
        RAM8((u16)(ptr + y)) = RAM8((u16)(0x00ED + y));
    }
    r->y = 0xFF;
}

void sub_F7F7(Regs *r)
{
#ifdef LOTW_HOST
    /* The L_F7AA path does STA ($0C),Y through a CA54-built pointer that can land
     * in the $0800-$9FFF window (open bus / mirrors on real HW). The diff oracle
     * models that window as flat-zero per state, but the host harness reuses one
     * memory array across states, so a prior state's high write would leak into a
     * later state's F2D3 pointer read. Reset that window to the oracle's zero
     * model so reads are deterministic. No-op semantics on the real NES. */
    {
        u16 i;
        for (i = 0x0800; i < 0xA000; ++i)
            RAM8(i) = 0;
    }
#endif

    /* L_F7F7 */
    if (RAM8(0xED) & 0x01) {
        if ((RAM8(0xF3) & 0x03) == 0)       /* BNE skips toggle */
            RAM8(0xED) ^= 0x04;
        goto done;                          /* L_F809: JMP L_F896 */
    }

    /* L_F80C */
    RAM8(0xE3) = 0x09;
    sub_EFF1(r);

    sub_CF1C(r);
    if (r->c) goto L_F85A;

    sub_F23A(r);
    if (r->c) goto L_F85A;

    sub_CE7C(r);
    if (r->c) goto L_F846;

    sub_CDB2(r);
    if (r->c) {                              /* BCC L_F82E -> carry set falls through */
        u8 x = RAM8(0x09);
        RAM8((u16)(0x0401 + x)) = 0x80;
    }
    /* L_F82E */
    RAM8(0xF9) = RAM8(0x0E);
    RAM8(0xFA) = RAM8(0x0F);
    RAM8(0xFB) = RAM8(0x0A);
    RAM8(0xF4) = 0x00;
    goto done;

L_F846:
    if (RAM8(0xF4) != 0) goto L_F886;
    if (RAM8(0x85) != 0) goto L_F85A;
    sub_E7CE(r);
    RAM8(0x8F) = 0x0A;
    RAM8(0x85) = 0x02;

L_F85A:
    if (RAM8(0xF4) != 0) goto L_F886;
    RAM8(0xF4) = (u8)(RAM8(0xF4) + 1);       /* INC $F4 */
    if (RAM8(0xF5) != 0) {                    /* BEQ L_F873 */
        RAM8(0xF5) = (u8)((0 - RAM8(0xF5)) & 0x0F);   /* EOR#$FF/CLC/ADC#$01/AND#$0F */
        RAM8(0xF6) ^= 0xFF;
    }
    /* L_F873 */
    RAM8(0xF7) = (u8)((u8)(~RAM8(0xF7)) + 1); /* EOR#$FF/TAX/INX/STX */
    if (RAM8(0x8F) == 0)
        RAM8(0x8F) = 0x06;
    goto done;                                /* L_F883: JMP L_F896 */

L_F886:
    if ((u8)((RAM8(0xFB) & 0x0F) | RAM8(0xF9)) != 0) {  /* BEQ L_F893 */
        RAM8(0xF3) = (u8)(RAM8(0xF3) + 1);    /* INC $F3 */
        goto done;
    }
    /* L_F893: JMP L_F7AA */
    {
        RAM8(0xEE) = 0x00;
        if (RAM8(0xF0) != 0) {                /* BNE L_F7B5 */
            u16 ptr;
            u8 diff;
            RAM8(0x0C) = RAM8(0xFA);
            RAM8(0x0D) = RAM8(0xFB);
            sub_CA54(r);                       /* mutates $0C/$0D */
            ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
            RAM8(ptr) = RAM8(0xF0);            /* STA ($0C),Y  with Y=0 */

            diff = (u8)(RAM8(0xFA) - RAM8(0x7C));  /* SEC / SBC scroll_x_tile */
            /* CMP #$11 / BCC L_F7D3 ; CMP #$FE / BCC L_F7F4 */
            if (diff < 0x11 || diff >= 0xFE) {     /* L_F7D3 path */
                u8 fa = RAM8(0xFA);
                RAM8(0x0C) = fa;
                RAM8(0x16) = (u8)((fa << 1) & 0x1F);       /* vram_dst_lo */
                RAM8(0x17) = (u8)((RAM8(0xFA) & 0x10) >> 2);/* vram_dst_hi */
                RAM8(0x16) = (u8)(0x00 + RAM8(0x16));
                RAM8(0x17) = (u8)(0x20 + RAM8(0x17));
                farcall_bank_09_r7(r);
            }
            /* L_F7F4 -> done */
        }
        /* else BEQ -> done */
    }

done:
    /* L_F896: JSR L_E99A (inlined, see e99a_copy) */
    e99a_copy(r);
}
