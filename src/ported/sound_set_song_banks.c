/* $FD87 sound_set_song_banks — point MMC3 R6/R7 at the music banks.
 *   LDA #$06 / STA $8000 / LDA $34 / STA $8001
 *   LDA #$07 / STA $8000 / LDA $35 / STA $8001 / RTS
 * Pure hardware side-effect (MMC3 bank registers); reads snd_music_bank0/1.
 * No RAM/reg outputs — compare ["ram"] passes trivially (verified by inspection). */
#include "ram.h"
#include "regs.h"

void sound_set_song_banks(Regs *r)
{
    REG_W(0x8000, 0x06);
    REG_W(0x8001, RAM8(0x34));   /* snd_music_bank0 -> MMC3 R6 */
    REG_W(0x8000, 0x07);
    REG_W(0x8001, RAM8(0x35));   /* snd_music_bank1 -> MMC3 R7 */
    (void)r;
}
