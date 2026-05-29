#ifndef LOTW_LEAVES_H
#define LOTW_LEAVES_H
#include "nes.h"
u8 sub_E41E(void);   /* $E41E: returns X = $F9 & $1F */
u8 sub_F233(u8 y);   /* $F233: carry = (*( ($0C/$0D) + y) & $3F) >= $30 */
u8 inc16_95(void);   /* $FD6B: ++($95:$96); returns X = $02 */
#endif
