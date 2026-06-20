














#include "game_memory.h"
#include "routine_context.h"

void routine_0170(RoutineContext *r);
void routine_0214(RoutineContext *r);
void routine_0204(RoutineContext *r);
void routine_0211(RoutineContext *r);
void routine_0090(RoutineContext *r);
void routine_0108(RoutineContext *r);
void routine_0172(RoutineContext *r);
void routine_0171(RoutineContext *r);
void routine_0272(RoutineContext *r);

void routine_0169(RoutineContext *r)
{
    u16 ptr = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
    u8 y = r->offset;
    u8 tile = GAME_MEM8((u16)(ptr + y)) & 0x3F;

    if (tile == GAME_MEM8(0x70)) {

        if (GAME_MEM8(0x0491) == 0) {
            GAME_MEM8(0x0B) = y;
            GAME_MEM8(0xED) = 0xE1;
            GAME_MEM8(0xEE) = 0x01;
            GAME_MEM8(0xEF) = 0x01;
            GAME_MEM8(0xF0) = GAME_MEM8(0x71);
            GAME_MEM8(0xF3) = 0x0A;
            routine_0170(r);
            routine_0214(r);
            GAME_MEM8(0x8F) = 0x06;
        }

        {
            u8 v = GAME_MEM8(0x71) & 0x3F;
            r->value = v;
            r->carry = (u8)(v >= 0x30);
        }
        return;
    }

    if (tile == 0x02) {

        if (GAME_MEM8(0x0491) == 0) {
            GAME_MEM8(0x0B) = y;
            r->index = GAME_MEM8(0x55);
            {
                u8 item = GAME_MEM8((u16)(0x0051 + r->index));
                r->value = item;
                if (item == 0x07) {
                    r->index = GAME_MEM8(0x55);
                    routine_0204(r);
                    if (r->carry) {
                        goto flow_0001_seal;
                    }
                } else {

                    routine_0211(r);
                    if (r->carry)
                        goto flow_0001_seal;
                }
            }

            GAME_MEM8(0xED) = 0xE1;
            GAME_MEM8(0xEE) = 0x01;
            GAME_MEM8(0xEF) = 0x01;
            GAME_MEM8(0xF0) = GAME_MEM8(0x74);
            GAME_MEM8(0xF3) = 0x0F;
            routine_0170(r);
            routine_0214(r);
            GAME_MEM8(0x8F) = 0x06;
        }
    flow_0001_seal:

        r->carry = 1;
        return;
    }

    if (tile == 0x3E) {

        if ((GAME_MEM8(0x20) & 0x80) &&
            GAME_MEM8(0x0491) == 0) {
            u8 idx;
            GAME_MEM8(0x0B) = y;
            GAME_MEM8(0xF4) = 0x01;
            r->offset = GAME_MEM8(0x55);
            r->index = GAME_MEM8((u16)(0x0051 + r->offset));
            idx = r->index;


            if (idx == 1) {

                if (GAME_MEM8(0x59) != 0) {
                    u8 t = GAME_MEM8(0x45) & 0x0F;
                    t |= GAME_MEM8(0x43);
                    if (t == 0) {
                        u8 x2 = (u8)((GAME_MEM8(0xFD) & 0x0F) << 1);
                        u8 lo = (u8)(GAME_MEM8(0x44) + GAME_MEM8((u16)(0xFEAB + x2)));
                        GAME_MEM8(0x049D) = lo;
                        GAME_MEM8(0x0C) = lo;
                        GAME_MEM8(0x049C) = 0x00;
                        {
                            u8 hi = (u8)(GAME_MEM8(0x45) + GAME_MEM8((u16)(0xFEAC + x2)));
                            GAME_MEM8(0x049E) = hi;
                            GAME_MEM8(0x0D) = hi;
                        }
                        routine_0090(r);
                        r->offset = 0x00;
                        GAME_MEM8(0x0B) = 0x00;
                        {
                            u16 p = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
                            u8 b = GAME_MEM8(p) & 0x3F;
                            if (b == 0x3E) {
                                GAME_MEM8(0x0490) = 0xE1;
                                GAME_MEM8(0x0491) = 0x01;
                                GAME_MEM8(0x0492) = 0x01;
                                GAME_MEM8(0x0496) = 0x0F;
                                routine_0172(r);
                                GAME_MEM8(0x0493) = r->value;
                                routine_0204(r);
                                GAME_MEM8(0x8F) = 0x14;
                            }
                        }
                    }
                }

                r->carry = 1;
                return;
            }
            if (idx == 2) {

                if ((GAME_MEM8(0xFD) & 0x0F) != 0) {
                    u8 b;
                    r->offset = 0x01;
                    routine_0108(r);
                    r->offset = 0xF8;
                    {
                        u16 p79 = (u16)(GAME_MEM8(0x79) | (GAME_MEM8(0x7A) << 8));
                        GAME_MEM8(0xED) = (u8)(GAME_MEM8((u16)(p79 + 0xF8)) & 0xFE);
                    }
                    GAME_MEM8(0xEE) = 0x01;
                    GAME_MEM8(0xEF) = 0x03;
                    r->offset = GAME_MEM8(0x0B);
                    b = GAME_MEM8((u16)(ptr + r->offset));
                    GAME_MEM8(0xF0) = b;
                    GAME_MEM8(0xF3) = 0x10;
                    routine_0172(r);
                    GAME_MEM8((u16)(ptr + r->offset)) = r->value;
                    routine_0170(r);
                    routine_0171(r);
                    routine_0272(r);
                    GAME_MEM8(0xE3) = 0xFF;
                    if (GAME_MEM8(0x0491) != 0)
                        GAME_MEM8(0x8F) = 0x06;
                }

                GAME_MEM8(0x4B) = 0x00;
                GAME_MEM8(0x4E) = 0x00;
                r->carry = 1;
                return;
            }
            if (idx == 3) {

                if (GAME_MEM8(0x59) != 0) {
                    if ((GAME_MEM8(0xFD) & 0x0F) != 0) {
                        u8 b;
                        r->offset = 0x08;
                        routine_0108(r);
                        r->offset = 0xF8;
                        {
                            u16 p79 = (u16)(GAME_MEM8(0x79) | (GAME_MEM8(0x7A) << 8));
                            GAME_MEM8(0xED) = (u8)(GAME_MEM8((u16)(p79 + 0xF8)) & 0xFE);
                        }
                        GAME_MEM8(0xEE) = 0x01;
                        GAME_MEM8(0xEF) = 0x03;
                        r->offset = GAME_MEM8(0x0B);
                        b = GAME_MEM8((u16)(ptr + r->offset));
                        GAME_MEM8(0xF0) = b;
                        GAME_MEM8(0xF3) = 0x00;
                        routine_0172(r);
                        GAME_MEM8((u16)(ptr + r->offset)) = r->value;
                        routine_0170(r);
                        routine_0171(r);
                        routine_0272(r);
                        GAME_MEM8(0xE3) = 0xFF;
                        if (GAME_MEM8(0xEE) != 0) {
                            GAME_MEM8(0x8F) = 0x14;
                            routine_0204(r);
                        }

                        GAME_MEM8(0x4B) = 0x00;
                        GAME_MEM8(0x4E) = 0x00;
                        r->carry = 1;
                        return;
                    }

                    GAME_MEM8(0x4B) = 0x00;
                    GAME_MEM8(0x4E) = 0x00;
                    r->carry = 1;
                    return;
                }

                r->carry = 1;
                return;
            }
        }

        r->carry = 1;
        return;
    }


    r->carry = (u8)(tile >= 0x30);
}
