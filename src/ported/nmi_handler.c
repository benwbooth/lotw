/* $D1FE nmi_handler — NMI/IRQ vector entry. Saves regs, reads PPUSTATUS (clears
 * the vblank latch) into nmi_scratch, triggers OAM DMA from page $02 ($0200 OAM
 * buffer), then if a VRAM job is queued (nmi_vram_req $28, value 1..6) sets the
 * VRAM dest address + control and dispatches to the matching vram_* uploader via
 * the jump table at $D244; each uploader tail-jumps into nmi_tail. Request 0 or
 * >=7 goes straight to nmi_tail.
 * INSPECTION-PORT (no diff-test spec): interrupt-context entry (PHA.../RTI through
 * nmi_tail), hardware OAM DMA + PPU register access; the indirect JMP ($0006) is
 * the jump table, modelled here as a switch. Integration-verified. */
#include "ram.h"
#include "regs.h"
void nmi_tail(Regs *r);
void vram_fill_run(Regs *r); void vram_upload_palette(Regs *r); void vram_upload_hud(Regs *r);
void vram_blit_stack(Regs *r); void vram_copy_indirect(Regs *r); void vram_poke2(Regs *r);

void nmi_handler(Regs *r)
{
    /* PHA / TXA / PHA / TYA / PHA — save regs (real NMI frame) */
    /* LDA PPUSTATUS -> nmi_scratch (clears the vblank latch on hardware). */
#ifdef LOTW_SHIM
    /* The NMI fires at the start of vblank (scanline 241): set the vblank flag, and
     * sprite-0 hit if the frame just rendered with rendering on (LotW's sprite 0 is
     * the status-bar split marker, always over opaque BG when PPUMASK $24 shows
     * BG/sprites). Then the real $2002 read returns vblank|sprite0|overflow|open-bus
     * into nmi_scratch and clears the vblank latch. */
    {
        extern void ppu_set_vblank(int);
        extern void ppu_set_sprite0(int);
        extern void ppu_eval_sprite_overflow(void);
        ppu_set_vblank(1);
        ppu_set_sprite0((RAM8(0x24) & 0x18) ? 1 : 0);
        ppu_eval_sprite_overflow();       /* bit5 from the just-rendered OAM */
    }
    RAM8(0x26) = REG_R(0x2002);           /* real $2002: vblank|sprite0|overflow|open-bus */
#else
    RAM8(0x26) = 0x00;                    /* nmi_scratch (PPUSTATUS read; 0 in flat host) */
#endif
    REG_W(0x2003, 0x00);                  /* OAMADDR = 0 */
    REG_W(0x4014, 0x02);                  /* OAMDMA from page $02 */

    u8 req = RAM8(0x28);                  /* LDA nmi_vram_req */
    if (req == 0) {                       /* BEQ L_D21E */
        nmi_tail(r);
        return;
    }
    RAM8(0x28) = 0x00;                    /* LDX #$00 / STX nmi_vram_req — the real code
                                           * clears the request for EVERY non-zero value,
                                           * BEFORE the CMP #$07 below. (Clearing only for
                                           * 1..6 left req>=7 jobs — e.g. the A=$FF reveal
                                           * jobs — pending forever, hanging callers that
                                           * spin on $28.) */
    if (req >= 0x07) {                    /* CMP #$07 / BCC L_D221 — else fall to nmi_tail */
        nmi_tail(r);
        return;
    }
    /* JMP ($0006): the real code copies the handler address from the $D244 jump
     * table into $06/$07 before dispatching. We dispatch via the switch below, but
     * still write $06/$07 so they match the hardware byte-for-byte. */
    {
        static const u8 jt_lo[7] = { 0x51, 0x52, 0x5F, 0x90, 0xE5, 0x34, 0x44 };
        static const u8 jt_hi[7] = { 0xD3, 0xD2, 0xD2, 0xD2, 0xD2, 0xD3, 0xD3 };
        RAM8(0x06) = jt_lo[req]; RAM8(0x07) = jt_hi[req];   /* $D244 table[req] */
    }
    (void)REG_R(0x2002);                  /* 2nd LDA PPUSTATUS (discarded; resets the latch) */
    REG_W(0x2006, RAM8(0x17));            /* PPUADDR hi = vram_dst_hi */
    REG_W(0x2006, RAM8(0x16));            /* PPUADDR lo = vram_dst_lo */
    REG_W(0x2000, (u8)(RAM8(0x23) & 0x04)); /* PPUCTRL = ppuctrl_shadow & $04 */
    switch (req) {                        /* JMP ($D244 + req*2) */
        case 1: vram_fill_run(r); break;
        case 2: vram_upload_palette(r); break;
        case 3: vram_upload_hud(r); break;
        case 4: vram_blit_stack(r); break;
        case 5: vram_copy_indirect(r); break;
        case 6: vram_poke2(r); break;
    }
    /* each vram_* tail-calls nmi_tail */
}
