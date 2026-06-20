/* $E5FD — shared level-resume tail (reached via JMP from E077/E424 after a
 * warp/transition). Pops the room checkpoint (E642), fades in (C3E5), runs E79D,
 * re-arms the level music ($FE -> D02E), reassembles the scene (C8FF/C5CB), and
 * redraws (C1D8/C2B1/C1C7/C492), ending in the D8AF tail (D8E3 + D94E).
 * INSPECTION-PORT (no diff-test spec): calls E642 (room-checkpoint pop) and the
 * frame-sync C3E5/C492 chain. Integration-verified. */
#include "ram.h"
#include "regs.h"
void sub_E642(Regs *r); void sub_C3E5(Regs *r); void sub_E79D(Regs *r);
void sub_D02E(Regs *r); void sub_C8FF(Regs *r); void sub_C5CB(Regs *r);
void sub_C1D8(Regs *r); void sub_C2B1(Regs *r); void sub_C1C7(Regs *r);
void sub_C492(Regs *r); void sub_D8E3(Regs *r); void sub_D94E(Regs *r);
void sub_E5FD(Regs *r)
{
    sub_E642(r);                /* JSR L_E642 (pop room checkpoint) */
    sub_C3E5(r);                /* JSR L_C3E5 (fade) */
    sub_E79D(r);
    r->a = RAM8(0xFE);          /* LDA $FE (level/song) */
    sub_D02E(r);                /* JSR L_D02E (set music) */
    sub_C8FF(r);
    sub_C5CB(r);
    sub_C1D8(r);
    sub_C2B1(r);
    sub_C1C7(r);
    sub_C492(r);
    sub_D8E3(r);                /* JMP L_D8AF = D8E3 + D94E */
    sub_D94E(r);
}
