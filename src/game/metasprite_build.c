














#include "game_memory.h"
#include "routine_context.h"

void queue_ppu_job_and_wait(RoutineContext *r);

void metasprite_build(RoutineContext *r)
{
    u16 p0C = (u16)(GAME_MEM8(0x0C) | (GAME_MEM8(0x0D) << 8));
    u16 p79 = (u16)(GAME_MEM8(0x79) | (GAME_MEM8(0x7A) << 8));
    int x, y;
    u8 dst_lo, mask2;

#ifdef LOTW_SHIM
    { extern int printf(const char*,...); static int n=0;
      if (p79 == 0xAD00 && n++ < 6) printf("[msb] p0C=$%04X p79=$%04X e0=$%02X ty=$%02X t0=$%02X t1=$%02X R6=%02X R7=%02X\n",
          p0C, p79, GAME_MEM8(p0C), (u8)(GAME_MEM8(p0C)<<2),
          GAME_MEM8((u16)(p79+((GAME_MEM8(p0C)<<2)&0xFF))), GAME_MEM8((u16)(p79+(((GAME_MEM8(p0C)<<2)+1)&0xFF))),
          GAME_MEM8(0x30), GAME_MEM8(0x31)); }
#endif
    GAME_MEM8(0x0B) = 0x00;
    for (x = 0x16; x >= 0; x -= 2) {
        u8 e = GAME_MEM8((u16)(p0C + GAME_MEM8(0x0B)));
        u16 ty = (u16)((u8)(e << 2));
        GAME_MEM8((u16)(0x0141 + x)) = GAME_MEM8((u16)(p79 + ((ty + 0) & 0xFF)));
        GAME_MEM8((u16)(0x0140 + x)) = GAME_MEM8((u16)(p79 + ((ty + 1) & 0xFF)));
        GAME_MEM8((u16)(0x0159 + x)) = GAME_MEM8((u16)(p79 + ((ty + 2) & 0xFF)));
        GAME_MEM8((u16)(0x0158 + x)) = GAME_MEM8((u16)(p79 + ((ty + 3) & 0xFF)));
        GAME_MEM8(0x0B) += 1;
    }

    GAME_MEM8(0x19) = (u8)(GAME_MEM8(0x17) + 0x03);
    dst_lo = GAME_MEM8(0x16);
    GAME_MEM8(0x0B) = (u8)((dst_lo >> 2) + 0xC0);
    mask2 = (u8)(dst_lo & 0x02);
    GAME_MEM8(0x18) = mask2 ? 0x33 : 0xCC;

    y = 0x00;
    for (x = 0x0A; x >= 0; x -= 2) {
        u8 b0, b1, v;
        GAME_MEM8((u16)(0x0170 + x)) = GAME_MEM8(0x0B);
        GAME_MEM8(0x0B) = (u8)(GAME_MEM8(0x0B) + 0x08);

        b0 = GAME_MEM8((u16)(p0C + (y++)));
        v = (u8)((b0 & 0xC0) >> 4);
        GAME_MEM8((u16)(0x0171 + x)) = v;

        b1 = GAME_MEM8((u16)(p0C + (y++)));
        v = (u8)((b1 & 0xC0) | GAME_MEM8((u16)(0x0171 + x)));
        GAME_MEM8((u16)(0x0171 + x)) = v;

        if (mask2 == 0) {
            GAME_MEM8((u16)(0x0171 + x)) = (u8)(GAME_MEM8((u16)(0x0171 + x)) >> 1);
            GAME_MEM8((u16)(0x0171 + x)) = (u8)(GAME_MEM8((u16)(0x0171 + x)) >> 1);
        }
    }

    r->value = 0x03;
    queue_ppu_job_and_wait(r);
}
