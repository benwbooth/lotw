/* $DD97 sub_DD97 — tile-interaction dispatcher. Reads the map tile through
 * pointer $0C/$0D indexed by Y, masks to $3F, and dispatches on its value:
 *   == $70 ($70 is the "door/match" tile id)      -> L_DDB3
 *   == $02                                          -> L_DDE0
 *   == $3E                                          -> L_DE1A
 *   else: CMP #$30 (carry = tile >= $30) / RTS
 *
 *   LDA ($0C),Y / AND #$3F / CMP $70 / BNE + / JMP L_DDB3
 * + CMP #$02 / BNE + / JMP L_DDE0
 * + CMP #$3E / BNE + / JMP L_DE1A
 * + CMP #$30 / RTS
 *
 * Sub-calls (already ported): sub_DF37, sub_E99A, sub_E7F0, sub_E86F,
 * sub_CA54, sub_CD70, sub_DF80, sub_DF5E, sub_F7F7.
 */
#include "ram.h"
#include "regs.h"

void sub_DF37(Regs *r);
void sub_E99A(Regs *r);
void sub_E7F0(Regs *r);
void sub_E86F(Regs *r);
void sub_CA54(Regs *r);
void sub_CD70(Regs *r);
void sub_DF80(Regs *r);
void sub_DF5E(Regs *r);
void sub_F7F7(Regs *r);

void sub_DD97(Regs *r)
{
    u16 ptr = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
    u8 y = r->y;
    u8 tile = RAM8((u16)(ptr + y)) & 0x3F;

    if (tile == RAM8(0x70)) {
        /* L_DDB3 */
        if (RAM8(0x0491) == 0) {            /* BNE L_DDD9 */
            RAM8(0x0B) = y;                  /* STY $0B */
            RAM8(0xED) = 0xE1;
            RAM8(0xEE) = 0x01;
            RAM8(0xEF) = 0x01;
            RAM8(0xF0) = RAM8(0x71);
            RAM8(0xF3) = 0x0A;
            sub_DF37(r);
            sub_E99A(r);
            RAM8(0x8F) = 0x06;
        }
        /* L_DDD9 */
        {
            u8 v = RAM8(0x71) & 0x3F;
            r->a = v;
            r->c = (u8)(v >= 0x30);          /* CMP #$30 */
        }
        return;
    }

    if (tile == 0x02) {
        /* L_DDE0 */
        if (RAM8(0x0491) == 0) {            /* BNE L_DE18 */
            RAM8(0x0B) = y;                  /* STY $0B */
            r->x = RAM8(0x55);
            {
                u8 item = RAM8((u16)(0x0051 + r->x));   /* carried_item0,X */
                r->a = item;
                if (item == 0x07) {          /* CMP #$07 / BNE L_DDF4 */
                    r->x = RAM8(0x55);    /* preserved through E7F0 */
                    sub_E7F0(r);
                    if (r->c) {              /* BCC L_DDF9 (carry clear -> consume) */
                        goto L_DE18_seal;    /* carry set: cannot consume RAM8(0x59) */
                    }
                } else {
                    /* L_DDF4 */
                    sub_E86F(r);
                    if (r->c)                /* BCS L_DE18 */
                        goto L_DE18_seal;
                }
            }
            /* L_DDF9 */
            RAM8(0xED) = 0xE1;
            RAM8(0xEE) = 0x01;
            RAM8(0xEF) = 0x01;
            RAM8(0xF0) = RAM8(0x74);
            RAM8(0xF3) = 0x0F;
            sub_DF37(r);
            sub_E99A(r);
            RAM8(0x8F) = 0x06;
        }
    L_DE18_seal:
        /* L_DE18 */
        r->c = 1;                            /* SEC */
        return;
    }

    if (tile == 0x3E) {
        /* L_DE1A */
        if ((RAM8(0x20) & 0x80) &&           /* BIT $20 / BPL L_DE37 (bit7 set) */
            RAM8(0x0491) == 0) {             /* BNE L_DE37 */
            u8 idx;
            RAM8(0x0B) = y;                  /* STY $0B */
            RAM8(0xF4) = 0x01;
            r->y = RAM8(0x55);            /* LDY RAM8(0x55) */
            r->x = RAM8((u16)(0x0051 + r->y));  /* LDX carried_item0,Y */
            idx = r->x;
            /* DEX/BEQ L_DE3F (idx==1); DEX/BEQ L_DE39 (idx==2);
             * DEX/BEQ L_DE3C (idx==3); else fall to L_DE37 (SEC/RTS) */
            if (idx == 1) {
                /* L_DE3F */
                if (RAM8(0x59) != 0) {            /* BEQ L_DE9D */
                    u8 t = RAM8(0x45) & 0x0F;
                    t |= RAM8(0x43);
                    if (t == 0) {            /* BNE L_DE9D */
                        u8 x2 = (u8)((RAM8(0xFD) & 0x0F) << 1);  /* AND#$0F/ASL/TAX */
                        u8 lo = (u8)(RAM8(0x44) + RAM8((u16)(0xFEAB + x2)));
                        RAM8(0x049D) = lo;
                        RAM8(0x0C) = lo;
                        RAM8(0x049C) = 0x00;
                        {
                            u8 hi = (u8)(RAM8(0x45) + RAM8((u16)(0xFEAC + x2)));
                            RAM8(0x049E) = hi;
                            RAM8(0x0D) = hi;
                        }
                        sub_CA54(r);          /* mutates $0C/$0D, builds $10/$11 */
                        r->y = 0x00;
                        RAM8(0x0B) = 0x00;    /* STY $0B */
                        {
                            u16 p = (u16)(RAM8(0x0C) | (RAM8(0x0D) << 8));
                            u8 b = RAM8(p) & 0x3F;
                            if (b == 0x3E) {  /* CMP #$3E / BNE L_DE9D */
                                RAM8(0x0490) = 0xE1;
                                RAM8(0x0491) = 0x01;
                                RAM8(0x0492) = 0x01;
                                RAM8(0x0496) = 0x0F;
                                sub_DF80(r);
                                RAM8(0x0493) = r->a;
                                sub_E7F0(r);
                                RAM8(0x8F) = 0x14;
                            }
                        }
                    }
                }
                /* L_DE9D */
                r->c = 1;                    /* SEC */
                return;
            }
            if (idx == 2) {
                /* L_DE39 -> L_DE9F */
                if ((RAM8(0xFD) & 0x0F) != 0) {  /* BEQ L_DEE0 */
                    u8 b;
                    r->y = 0x01;             /* LDY #$01 */
                    sub_CD70(r);
                    r->y = 0xF8;
                    {
                        u16 p79 = (u16)(RAM8(0x79) | (RAM8(0x7A) << 8));
                        RAM8(0xED) = (u8)(RAM8((u16)(p79 + 0xF8)) & 0xFE);
                    }
                    RAM8(0xEE) = 0x01;
                    RAM8(0xEF) = 0x03;
                    r->y = RAM8(0x0B);       /* LDY $0B */
                    b = RAM8((u16)(ptr + r->y));  /* LDA ($0C),Y */
                    RAM8(0xF0) = b;
                    RAM8(0xF3) = 0x10;
                    sub_DF80(r);
                    RAM8((u16)(ptr + r->y)) = r->a;   /* STA ($0C),Y */
                    sub_DF37(r);
                    sub_DF5E(r);
                    sub_F7F7(r);
                    RAM8(0xE3) = 0xFF;
                    if (RAM8(0x0491) != 0)   /* BEQ L_DEE0 */
                        RAM8(0x8F) = 0x06;
                }
                /* L_DEE0 */
                RAM8(0x4B) = 0x00;
                RAM8(0x4E) = 0x00;
                r->c = 1;
                return;
            }
            if (idx == 3) {
                /* L_DE3C -> L_DEE8 */
                if (RAM8(0x59) != 0) {            /* BEQ L_DE9D */
                    if ((RAM8(0xFD) & 0x0F) != 0) {  /* BEQ L_DF2F */
                        u8 b;
                        r->y = 0x08;         /* LDY #$08 */
                        sub_CD70(r);
                        r->y = 0xF8;
                        {
                            u16 p79 = (u16)(RAM8(0x79) | (RAM8(0x7A) << 8));
                            RAM8(0xED) = (u8)(RAM8((u16)(p79 + 0xF8)) & 0xFE);
                        }
                        RAM8(0xEE) = 0x01;
                        RAM8(0xEF) = 0x03;
                        r->y = RAM8(0x0B);
                        b = RAM8((u16)(ptr + r->y));
                        RAM8(0xF0) = b;
                        RAM8(0xF3) = 0x00;
                        sub_DF80(r);
                        RAM8((u16)(ptr + r->y)) = r->a;
                        sub_DF37(r);
                        sub_DF5E(r);
                        sub_F7F7(r);
                        RAM8(0xE3) = 0xFF;
                        if (RAM8(0xEE) != 0) {   /* BEQ L_DF2F */
                            RAM8(0x8F) = 0x14;
                            sub_E7F0(r);
                        }
                        /* L_DF2F */
                        RAM8(0x4B) = 0x00;
                        RAM8(0x4E) = 0x00;
                        r->c = 1;
                        return;
                    }
                    /* L_DF2F (FD&0F == 0) */
                    RAM8(0x4B) = 0x00;
                    RAM8(0x4E) = 0x00;
                    r->c = 1;
                    return;
                }
                /* RAM8(0x59) == 0 -> L_DE9D */
                r->c = 1;
                return;
            }
        }
        /* L_DE37 */
        r->c = 1;                            /* SEC */
        return;
    }

    /* else: CMP #$30 / RTS -> carry = tile >= $30 */
    r->c = (u8)(tile >= 0x30);
}
