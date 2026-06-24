//! Named bit and mask constants used in place of raw hex literals in bitwise
//! operations. Each constant is exactly its hex value; the names describe the
//! bit pattern (e.g. `BIT4` == `0x10`, `LOW_NIBBLE` == `0x0F`,
//! `CLEAR_BIT2` == `0xFB`, a mask that clears bit 2 when AND-ed).

// Single bits.
/// Bit 0 (`0x01`).
pub const BIT0: i32 = 0x01;
/// Bit 1 (`0x02`).
pub const BIT1: i32 = 0x02;
/// Bit 2 (`0x04`).
pub const BIT2: i32 = 0x04;
/// Bit 3 (`0x08`).
pub const BIT3: i32 = 0x08;
/// Bit 4 (`0x10`).
pub const BIT4: i32 = 0x10;
/// Bit 5 (`0x20`).
pub const BIT5: i32 = 0x20;
/// Bit 6 (`0x40`).
pub const BIT6: i32 = 0x40;
/// Bit 7 (`0x80`).
pub const BIT7: i32 = 0x80;
/// Bit 8 (`0x100`).
pub const BIT8: i32 = 0x100;

// Contiguous low-bit masks.
/// Mask of the low 2 bits (`0x03`).
pub const LOW_2_BITS: i32 = 0x03;
/// Mask of the low 3 bits (`0x07`).
pub const LOW_3_BITS: i32 = 0x07;
/// Mask of the low nibble, i.e. the low 4 bits (`0x0F`).
pub const LOW_NIBBLE: i32 = 0x0F;
/// Mask of the low 5 bits (`0x1F`).
pub const LOW_5_BITS: i32 = 0x1F;
/// Mask of the low 6 bits (`0x3F`).
pub const LOW_6_BITS: i32 = 0x3F;
/// Mask of the low 7 bits (`0x7F`).
pub const LOW_7_BITS: i32 = 0x7F;
/// Mask of a full byte, the low 8 bits (`0xFF`).
pub const BYTE_MASK: i32 = 0xFF;
/// Mask of a full 16-bit word, the low 16 bits (`0xFFFF`).
pub const WORD_MASK: i32 = 0xFFFF;

// Wider address masks.
/// Mask of the low 10 address bits (`0x3FF`).
pub const ADDR_10_BITS: i32 = 0x3FF;
/// Mask of the low 11 address bits (`0x7FF`).
pub const ADDR_11_BITS: i32 = 0x7FF;
/// Mask of the low 12 address bits (`0xFFF`).
pub const ADDR_12_BITS: i32 = 0xFFF;
/// Mask of the low 13 address bits (`0x1FFF`).
pub const ADDR_13_BITS: i32 = 0x1FFF;
/// Mask of the low 14 address bits (`0x3FFF`).
pub const ADDR_14_BITS: i32 = 0x3FFF;
/// Bit 8, the nametable-X select bit of a PPU address (`0x100`).
pub const NAMETABLE_X_BIT: i32 = 0x100;
/// Mask of the high byte of a 16-bit word, bits 8-15 (`0xFF00`).
pub const HIGH_BYTE_MASK: i32 = 0xFF00;

// High / mid contiguous masks.
/// Mask of the high nibble of a byte, bits 4-7 (`0xF0`).
pub const HIGH_NIBBLE: i32 = 0xF0;
/// Mask of the high 2 bits of a byte, bits 6-7 (`0xC0`).
pub const HIGH_2_BITS: i32 = 0xC0;
/// Mask of the high 3 bits of a byte, bits 5-7 (`0xE0`).
pub const HIGH_3_BITS: i32 = 0xE0;
/// Mask of the high 5 bits of a byte, bits 3-7 (`0xF8`).
pub const HIGH_5_BITS: i32 = 0xF8;

// Multi-bit groups.
/// Mask of bits 1 and 2 (`0x06`).
pub const BITS_1_2: i32 = 0x06;
/// Mask of bits 2 and 3 (`0x0C`).
pub const BITS_2_3: i32 = 0x0C;
/// Mask of bits 3 and 4 (`0x18`).
pub const BITS_3_4: i32 = 0x18;
/// Mask of bits 2, 3 and 4 (`0x1C`).
pub const BITS_2_3_4: i32 = 0x1C;
/// Mask of bits 4 and 5 (`0x30`).
pub const BITS_4_5: i32 = 0x30;
/// Mask of bits 0 and 6 (`0x41`).
pub const BITS_0_6: i32 = 0x41;
/// Mask of bits 0 and 7 (`0x81`).
pub const BITS_0_7: i32 = 0x81;
/// Mask of bits 8, 9 and 10 (`0x700`).
pub const BITS_8_9_10: i32 = 0x700;

// Inverted single-bit clear masks (AND clears that bit, keeps the rest).
/// AND-mask that clears bit 0, keeping the rest (`0xFE`).
pub const CLEAR_BIT0: i32 = 0xFE;
/// AND-mask that clears bit 1, keeping the rest (`0xFD`).
pub const CLEAR_BIT1: i32 = 0xFD;
/// AND-mask that clears bit 2, keeping the rest (`0xFB`).
pub const CLEAR_BIT2: i32 = 0xFB;
/// AND-mask that clears bit 3, keeping the rest (`0xF7`).
pub const CLEAR_BIT3: i32 = 0xF7;
/// AND-mask that clears bit 4, keeping the rest (`0xEF`).
pub const CLEAR_BIT4: i32 = 0xEF;
/// AND-mask that clears bit 5, keeping the rest (`0xDF`).
pub const CLEAR_BIT5: i32 = 0xDF;
/// AND-mask that clears bit 6, keeping the rest (`0xBF`).
pub const CLEAR_BIT6: i32 = 0xBF;

// Inverted multi-bit clear masks.
/// AND-mask that clears bits 1 and 2, keeping the rest (`0xF9`).
pub const CLEAR_BITS_1_2: i32 = 0xF9;
/// AND-mask that clears bits 2 and 3, keeping the rest (`0xF3`).
pub const CLEAR_BITS_2_3: i32 = 0xF3;
/// AND-mask that clears bits 3 and 4, keeping the rest (`0xE7`).
pub const CLEAR_BITS_3_4: i32 = 0xE7;
/// AND-mask that clears bits 4 and 5, keeping the rest (`0xCF`).
pub const CLEAR_BITS_4_5: i32 = 0xCF;
/// AND-mask that clears bits 2 and 7, keeping the rest (`0x7B`).
pub const CLEAR_BITS_2_7: i32 = 0x7B;
