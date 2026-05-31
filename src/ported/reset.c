/* $FFE0 reset — the CPU RESET vector. Masks interrupts, parks the MMC3 in a known
 * state (bank-select / PRG-RAM / IRQ-disable all written $00), then jumps to the
 * boot routine main_init.
 *
 *   SEI
 *   LDA #$00
 *   STA MMC3_BANK_SELECT ($8000)
 *   STA MMC3_PRGRAM      ($A001)
 *   STA MMC3_IRQ_DISABLE ($E000)
 *   JMP main_init
 *
 * INSPECTION-PORT (no diff-test spec): the RESET entry point — pure MMC3 hardware
 * register writes (REG_W) followed by a tail-jump into the boot path; no testable
 * RAM result. Integration-verified. */
#include "ram.h"
#include "regs.h"

void main_init(Regs *r);

void reset(Regs *r)
{
    /* SEI — interrupt disable is implicit in the flat host model. */
    REG_W(0x8000, 0x00);                /* MMC3_BANK_SELECT */
    REG_W(0xA001, 0x00);                /* MMC3_PRGRAM */
    REG_W(0xE000, 0x00);                /* MMC3_IRQ_DISABLE */
    main_init(r);                       /* JMP main_init */
}
