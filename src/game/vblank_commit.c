



#include "game_memory.h"
#include "routine_context.h"
void vblank_commit_tail(RoutineContext *r);
void vram_fill_run(RoutineContext *r); void vram_upload_palette(RoutineContext *r); void vram_upload_hud(RoutineContext *r);
void vram_blit_stack(RoutineContext *r); void vram_copy_indirect(RoutineContext *r); void vram_poke2(RoutineContext *r);

void vblank_commit(RoutineContext *r)
{

    RoutineContext save = *r;
#define COMMIT_RETURN() do { *r = save; return; } while (0)


#ifdef LOTW_SHIM



    {
        extern void ppu_set_vblank(int);
        extern void ppu_set_sprite0(int);
        extern void ppu_eval_sprite_overflow(void);
        ppu_set_vblank(1);
        ppu_set_sprite0((GAME_MEM8(0x24) & 0x18) ? 1 : 0);
        ppu_eval_sprite_overflow();
    }
    GAME_MEM8(0x26) = REG_R(0x2002);
#else
    GAME_MEM8(0x26) = 0x00;
#endif
    REG_W(0x2003, 0x00);
    REG_W(0x4014, 0x02);

    u8 req = GAME_MEM8(0x28);
    if (req == 0) {
        vblank_commit_tail(r);
        COMMIT_RETURN();
    }
    GAME_MEM8(0x28) = 0x00;





    if (req >= 0x07) {
        vblank_commit_tail(r);
        COMMIT_RETURN();
    }



    {
        static const u8 jt_lo[7] = { 0x51, 0x52, 0x5F, 0x90, 0xE5, 0x34, 0x44 };
        static const u8 jt_hi[7] = { 0xD3, 0xD2, 0xD2, 0xD2, 0xD2, 0xD3, 0xD3 };
        GAME_MEM8(0x06) = jt_lo[req]; GAME_MEM8(0x07) = jt_hi[req];
    }
    (void)REG_R(0x2002);
    REG_W(0x2006, GAME_MEM8(0x17));
    REG_W(0x2006, GAME_MEM8(0x16));
    REG_W(0x2000, (u8)(GAME_MEM8(0x23) & 0x04));
    switch (req) {
        case 1: vram_fill_run(r); break;
        case 2: vram_upload_palette(r); break;
        case 3: vram_upload_hud(r); break;
        case 4: vram_blit_stack(r); break;
        case 5: vram_copy_indirect(r); break;
        case 6: vram_poke2(r); break;
    }

    *r = save;
#undef COMMIT_RETURN
}
