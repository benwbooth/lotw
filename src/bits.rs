//! Named bit and mask constants used in place of raw hex literals in bitwise
//! operations. Each constant is exactly its hex value; the names describe the
//! bit pattern (e.g. `BIT4` == `0x10`, `LOW_NIBBLE` == `0x0F`,
//! `CLEAR_BIT2` == `0xFB`, a mask that clears bit 2 when AND-ed).

// Single bits.
pub const BIT0: i32 = 0x01;
pub const BIT1: i32 = 0x02;
pub const BIT2: i32 = 0x04;
pub const BIT3: i32 = 0x08;
pub const BIT4: i32 = 0x10;
pub const BIT5: i32 = 0x20;
pub const BIT6: i32 = 0x40;
pub const BIT7: i32 = 0x80;
pub const BIT8: i32 = 0x100;

// Contiguous low-bit masks.
pub const LOW_2_BITS: i32 = 0x03;
pub const LOW_3_BITS: i32 = 0x07;
pub const LOW_NIBBLE: i32 = 0x0F;
pub const LOW_5_BITS: i32 = 0x1F;
pub const LOW_6_BITS: i32 = 0x3F;
pub const LOW_7_BITS: i32 = 0x7F;
pub const BYTE_MASK: i32 = 0xFF;
pub const WORD_MASK: i32 = 0xFFFF;

// Wider address masks.
pub const ADDR_10_BITS: i32 = 0x3FF;
pub const ADDR_11_BITS: i32 = 0x7FF;
pub const ADDR_12_BITS: i32 = 0xFFF;
pub const ADDR_13_BITS: i32 = 0x1FFF;
pub const ADDR_14_BITS: i32 = 0x3FFF;
pub const NAMETABLE_X_BIT: i32 = 0x100;
pub const HIGH_BYTE_MASK: i32 = 0xFF00;

// High / mid contiguous masks.
pub const HIGH_NIBBLE: i32 = 0xF0;
pub const HIGH_2_BITS: i32 = 0xC0;
pub const HIGH_3_BITS: i32 = 0xE0;
pub const HIGH_5_BITS: i32 = 0xF8;

// Multi-bit groups.
pub const BITS_1_2: i32 = 0x06;
pub const BITS_2_3: i32 = 0x0C;
pub const BITS_3_4: i32 = 0x18;
pub const BITS_2_3_4: i32 = 0x1C;
pub const BITS_4_5: i32 = 0x30;
pub const BITS_0_6: i32 = 0x41;
pub const BITS_0_7: i32 = 0x81;
pub const BITS_8_9_10: i32 = 0x700;

// Inverted single-bit clear masks (AND clears that bit, keeps the rest).
pub const CLEAR_BIT0: i32 = 0xFE;
pub const CLEAR_BIT1: i32 = 0xFD;
pub const CLEAR_BIT2: i32 = 0xFB;
pub const CLEAR_BIT3: i32 = 0xF7;
pub const CLEAR_BIT4: i32 = 0xEF;
pub const CLEAR_BIT5: i32 = 0xDF;
pub const CLEAR_BIT6: i32 = 0xBF;

// Inverted multi-bit clear masks.
pub const CLEAR_BITS_1_2: i32 = 0xF9;
pub const CLEAR_BITS_2_3: i32 = 0xF3;
pub const CLEAR_BITS_3_4: i32 = 0xE7;
pub const CLEAR_BITS_4_5: i32 = 0xCF;
pub const CLEAR_BITS_2_7: i32 = 0x7B;
