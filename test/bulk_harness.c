



















#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <setjmp.h>
#include <signal.h>
#include <sys/time.h>
#include "routine_context.h"

u8 LOTW_MEMORY[0x10000];
void lotw_device_write(u16 addr, u8 val) { (void)addr; (void)val; }

extern RoutineFn ROUTINE_FNS[];
extern int ROUTINE_N;

static sigjmp_buf g_jb;
static void on_alarm(int sig) { (void)sig; siglongjmp(g_jb, 1); }

#define WATCHDOG_USEC 150000

static void arm_watchdog(void)
{
    struct itimerval it = {{0, 0}, {0, WATCHDOG_USEC}};
    setitimer(ITIMER_REAL, &it, NULL);
}
static void disarm_watchdog(void)
{
    struct itimerval it = {{0, 0}, {0, 0}};
    setitimer(ITIMER_REAL, &it, NULL);
}

static void load_rom(const char *path)
{
    FILE *f = fopen(path, "rb");
    static unsigned char rom[196624];
    if (!f || fread(rom, 1, sizeof rom, f) != sizeof rom) { perror("rom"); exit(3); }
    fclose(f);
    memcpy(LOTW_MEMORY + 0xC000, rom + 0x10 + 14 * 0x2000, 0x4000);
    memcpy(LOTW_MEMORY + 0xA000, rom + 0x10 + 13 * 0x2000, 0x2000);
}

#define HDR 8
int main(int argc, char **argv)
{
    if (argc < 2) { fprintf(stderr, "usage: bulk_harness rom.nes\n"); return 1; }
    load_rom(argv[1]);
    signal(SIGALRM, on_alarm);

    unsigned char in[HDR + 0x800], out[HDR + 0x800];
    while (fread(in, 1, sizeof in, stdin) == sizeof in) {
        u8 id = in[0];
        RoutineContext r;
        r.value = in[1]; r.index = in[2]; r.offset = in[3];
        r.carry = in[4]; r.zero = in[5]; r.negative = in[6]; r.overflow = in[7];



        memset(LOTW_MEMORY, 0, 0xA000);
        memcpy(LOTW_MEMORY, in + HDR, 0x800);
        if (id >= ROUTINE_N) { fprintf(stderr, "bad id %u\n", id); return 2; }

        if (sigsetjmp(g_jb, 1) == 0) {
            arm_watchdog();
            ROUTINE_FNS[id](&r);
            disarm_watchdog();
            out[0] = 0;
            out[1] = r.value; out[2] = r.index; out[3] = r.offset;
            out[4] = r.carry; out[5] = r.zero; out[6] = r.negative; out[7] = r.overflow;
            memcpy(out + HDR, LOTW_MEMORY, 0x800);
        } else {


            disarm_watchdog();
            out[0] = 1;
            out[1] = r.value; out[2] = r.index; out[3] = r.offset;
            out[4] = r.carry; out[5] = r.zero; out[6] = r.negative; out[7] = r.overflow;
            memcpy(out + HDR, in + HDR, 0x800);
        }
        fwrite(out, 1, sizeof out, stdout);
    }
    return 0;
}
