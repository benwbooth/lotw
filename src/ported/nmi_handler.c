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
    /* At the NMI (start of vblank, scanline 241): bit7 vblank is set; bit6 sprite-0
     * hit is set iff sprite 0 overlapped opaque BG during the frame just rendered.
     * In LotW sprite 0 is the status-bar split marker, present (and overlapping the
     * HUD background) on every frame that rendering is enabled (PPUMASK shadow $24
     * shows BG or sprites). The game branches on bit6 (sub_A3E3 / sub_ABBC); the low
     * bits are PPU open bus (unmodelled, so $26 is masked in the lockstep diff).
     * Hardcoding 0 made every sprite-0-hit branch take the wrong path. */
    RAM8(0x26) = (u8)(0x80 | ((RAM8(0x24) & 0x18) ? 0x40 : 0x00));
#else
    RAM8(0x26) = 0x00;                    /* nmi_scratch (PPUSTATUS read; 0 in flat host) */
#endif
    REG_W(0x2003, 0x00);                  /* OAMADDR = 0 */
    REG_W(0x4014, 0x02);                  /* OAMDMA from page $02 */

    u8 req = RAM8(0x28);                  /* nmi_vram_req */
    if (req == 0 || req >= 0x07) {        /* BEQ L_D21E / CMP #$07 BCC else */
        nmi_tail(r);
        return;
    }
    RAM8(0x28) = 0x00;                    /* clear the request */
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
