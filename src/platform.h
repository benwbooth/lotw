





#ifndef LOTW_PLATFORM_H
#define LOTW_PLATFORM_H

typedef unsigned char u8;
typedef unsigned short u16;

#ifdef __cplusplus
extern "C" {
#endif

#ifdef LOTW_HOST
extern u8 LOTW_MEMORY[0x10000];
#define GAME_MEM8(a)  (LOTW_MEMORY[(a) & 0xFFFF])
void lotw_device_write(u16 addr, u8 val);
u8   lotw_device_read(u16 addr);
#define REG_W(a, v) lotw_device_write((u16)(a), (u8)(v))
#define REG_R(a)    lotw_device_read((u16)(a))
#else
#define GAME_MEM8(a)  (*(volatile u8 *)(a))
#define REG_W(a, v) (*(volatile u8 *)(a) = (u8)(v))
#define REG_R(a)    (*(volatile u8 *)(a))
#endif




#ifdef LOTW_SHIM
void lotw_prg_map_shadow(void);
#define LOTW_BANK_SYNC() lotw_prg_map_shadow()
#else
#define LOTW_BANK_SYNC() ((void)0)
#endif

#ifdef __cplusplus
}
#endif

#endif
