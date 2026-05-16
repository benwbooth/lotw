use std::fmt;

const INES_HEADER_LEN: usize = 16;
const TRAINER_LEN: usize = 512;
const PRG_BANK_16K: usize = 16 * 1024;
const CHR_BANK_8K: usize = 8 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RomFormat {
    INes,
    Nes2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mirroring {
    Horizontal,
    Vertical,
    FourScreen,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RomHeader {
    pub format: RomFormat,
    pub mapper: u16,
    pub submapper: u8,
    pub prg_rom_size: usize,
    pub chr_rom_size: usize,
    pub mirroring: Mirroring,
    pub has_trainer: bool,
    pub battery_backed_ram: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InesRom {
    header: RomHeader,
    prg_rom: Vec<u8>,
    chr_rom: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RomError {
    TooSmall { actual: usize },
    BadMagic,
    MissingTrainer { needed: usize, actual: usize },
    MissingPrg { needed: usize, actual: usize },
    MissingChr { needed: usize, actual: usize },
}

impl fmt::Display for RomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TooSmall { actual } => {
                write!(f, "ROM is too small for an iNES header: {actual} bytes")
            }
            Self::BadMagic => write!(f, "ROM does not start with the iNES magic bytes"),
            Self::MissingTrainer { needed, actual } => {
                write!(
                    f,
                    "ROM header declares a trainer but only {actual}/{needed} bytes are present"
                )
            }
            Self::MissingPrg { needed, actual } => {
                write!(
                    f,
                    "ROM is truncated in PRG ROM: only {actual}/{needed} bytes are present"
                )
            }
            Self::MissingChr { needed, actual } => {
                write!(
                    f,
                    "ROM is truncated in CHR ROM: only {actual}/{needed} bytes are present"
                )
            }
        }
    }
}

impl std::error::Error for RomError {}

impl InesRom {
    pub fn parse(bytes: &[u8]) -> Result<Self, RomError> {
        if bytes.len() < INES_HEADER_LEN {
            return Err(RomError::TooSmall {
                actual: bytes.len(),
            });
        }
        if &bytes[0..4] != b"NES\x1a" {
            return Err(RomError::BadMagic);
        }

        let flags6 = bytes[6];
        let flags7 = bytes[7];
        let format = if flags7 & 0x0c == 0x08 {
            RomFormat::Nes2
        } else {
            RomFormat::INes
        };
        let mapper = match format {
            RomFormat::INes => u16::from((flags6 >> 4) | (flags7 & 0xf0)),
            RomFormat::Nes2 => {
                u16::from((flags6 >> 4) | (flags7 & 0xf0)) | (u16::from(bytes[8] & 0x0f) << 8)
            }
        };
        let submapper = match format {
            RomFormat::INes => 0,
            RomFormat::Nes2 => bytes[8] >> 4,
        };
        let prg_rom_size = match format {
            RomFormat::INes => bytes[4] as usize * PRG_BANK_16K,
            RomFormat::Nes2 => nes2_rom_size(bytes[4], bytes[9] & 0x0f, PRG_BANK_16K),
        };
        let chr_rom_size = match format {
            RomFormat::INes => bytes[5] as usize * CHR_BANK_8K,
            RomFormat::Nes2 => nes2_rom_size(bytes[5], bytes[9] >> 4, CHR_BANK_8K),
        };
        let has_trainer = flags6 & 0x04 != 0;
        let trainer_end = INES_HEADER_LEN + if has_trainer { TRAINER_LEN } else { 0 };

        if bytes.len() < trainer_end {
            return Err(RomError::MissingTrainer {
                needed: trainer_end,
                actual: bytes.len(),
            });
        }

        let prg_end = trainer_end + prg_rom_size;
        if bytes.len() < prg_end {
            return Err(RomError::MissingPrg {
                needed: prg_end,
                actual: bytes.len(),
            });
        }

        let chr_end = prg_end + chr_rom_size;
        if bytes.len() < chr_end {
            return Err(RomError::MissingChr {
                needed: chr_end,
                actual: bytes.len(),
            });
        }

        let mirroring = if flags6 & 0x08 != 0 {
            Mirroring::FourScreen
        } else if flags6 & 0x01 != 0 {
            Mirroring::Vertical
        } else {
            Mirroring::Horizontal
        };

        Ok(Self {
            header: RomHeader {
                format,
                mapper,
                submapper,
                prg_rom_size,
                chr_rom_size,
                mirroring,
                has_trainer,
                battery_backed_ram: flags6 & 0x02 != 0,
            },
            prg_rom: bytes[trainer_end..prg_end].to_vec(),
            chr_rom: bytes[prg_end..chr_end].to_vec(),
        })
    }

    pub fn header(&self) -> &RomHeader {
        &self.header
    }

    pub fn prg_rom(&self) -> &[u8] {
        &self.prg_rom
    }

    pub fn chr_rom(&self) -> &[u8] {
        &self.chr_rom
    }

    pub fn fixed_bank_prg_offset(&self, cpu_addr: u16) -> Option<usize> {
        if cpu_addr < 0xc000 || self.prg_rom.len() < PRG_BANK_16K {
            return None;
        }
        let offset = self.prg_rom.len() - PRG_BANK_16K + usize::from(cpu_addr - 0xc000);
        (offset < self.prg_rom.len()).then_some(offset)
    }

    pub fn banked_prg_offset_16k(&self, bank: usize, cpu_addr: u16) -> Option<usize> {
        if !(0x8000..=0xbfff).contains(&cpu_addr) {
            return None;
        }
        let offset = bank
            .checked_mul(PRG_BANK_16K)?
            .checked_add(usize::from(cpu_addr - 0x8000))?;
        (offset < self.prg_rom.len()).then_some(offset)
    }
}

fn nes2_rom_size(lsb_banks: u8, msb_nibble: u8, unit: usize) -> usize {
    if msb_nibble == 0x0f {
        lsb_banks as usize * unit
    } else {
        (((msb_nibble as usize) << 8) | lsb_banks as usize) * unit
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ines_fixture(flags6: u8, flags7: u8) -> Vec<u8> {
        let mut bytes = vec![0u8; INES_HEADER_LEN + PRG_BANK_16K * 2 + CHR_BANK_8K];
        bytes[0..4].copy_from_slice(b"NES\x1a");
        bytes[4] = 2;
        bytes[5] = 1;
        bytes[6] = flags6;
        bytes[7] = flags7;
        bytes[INES_HEADER_LEN] = 0xaa;
        bytes[INES_HEADER_LEN + PRG_BANK_16K] = 0xbb;
        bytes[INES_HEADER_LEN + PRG_BANK_16K * 2] = 0xcc;
        bytes
    }

    #[test]
    fn parses_ines_header_and_banks() {
        let rom = InesRom::parse(&ines_fixture(0x41, 0x00)).unwrap();
        assert_eq!(rom.header().mapper, 4);
        assert_eq!(rom.header().format, RomFormat::INes);
        assert_eq!(rom.header().submapper, 0);
        assert_eq!(rom.header().mirroring, Mirroring::Vertical);
        assert_eq!(rom.header().prg_rom_size, PRG_BANK_16K * 2);
        assert_eq!(rom.header().chr_rom_size, CHR_BANK_8K);
        assert_eq!(rom.prg_rom()[0], 0xaa);
        assert_eq!(rom.prg_rom()[PRG_BANK_16K], 0xbb);
        assert_eq!(rom.chr_rom()[0], 0xcc);
        assert_eq!(rom.banked_prg_offset_16k(1, 0x8000), Some(PRG_BANK_16K));
        assert_eq!(rom.fixed_bank_prg_offset(0xc000), Some(PRG_BANK_16K));
    }

    #[test]
    fn rejects_truncated_prg() {
        let mut bytes = ines_fixture(0x00, 0x00);
        bytes.truncate(INES_HEADER_LEN + PRG_BANK_16K);
        assert!(matches!(
            InesRom::parse(&bytes),
            Err(RomError::MissingPrg { .. })
        ));
    }

    #[test]
    fn parses_nes2_header_sizes_like_c_tooling() {
        let mut bytes = ines_fixture(0x40, 0x08);
        bytes[4] = 8;
        bytes[5] = 8;
        bytes[9] = 0;
        bytes.resize(INES_HEADER_LEN + PRG_BANK_16K * 8 + CHR_BANK_8K * 8, 0);

        let rom = InesRom::parse(&bytes).unwrap();

        assert_eq!(rom.header().format, RomFormat::Nes2);
        assert_eq!(rom.header().mapper, 4);
        assert_eq!(rom.header().submapper, 0);
        assert_eq!(rom.header().prg_rom_size, PRG_BANK_16K * 8);
        assert_eq!(rom.header().chr_rom_size, CHR_BANK_8K * 8);
    }
}
