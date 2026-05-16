use std::io;

const PNG_SIGNATURE: &[u8; 8] = b"\x89PNG\r\n\x1a\n";

pub fn encode_rgb(width: usize, height: usize, rgb: &[u8]) -> io::Result<Vec<u8>> {
    let expected = width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(3))
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "PNG dimensions overflow"))?;
    if rgb.len() != expected {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("RGB buffer has {} bytes, expected {expected}", rgb.len()),
        ));
    }
    if width > u32::MAX as usize || height > u32::MAX as usize {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "PNG dimensions exceed u32",
        ));
    }

    let mut png = Vec::new();
    png.extend_from_slice(PNG_SIGNATURE);

    let mut ihdr = Vec::with_capacity(13);
    ihdr.extend_from_slice(&(width as u32).to_be_bytes());
    ihdr.extend_from_slice(&(height as u32).to_be_bytes());
    ihdr.extend_from_slice(&[8, 2, 0, 0, 0]);
    write_chunk(&mut png, b"IHDR", &ihdr);

    let row_len = width * 3;
    let mut filtered = Vec::with_capacity(height * (row_len + 1));
    for row in 0..height {
        filtered.push(0);
        let start = row * row_len;
        filtered.extend_from_slice(&rgb[start..start + row_len]);
    }

    let compressed = zlib_store(&filtered);
    write_chunk(&mut png, b"IDAT", &compressed);
    write_chunk(&mut png, b"IEND", &[]);
    Ok(png)
}

fn write_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(kind);
    out.extend_from_slice(data);

    let mut crc_input = Vec::with_capacity(kind.len() + data.len());
    crc_input.extend_from_slice(kind);
    crc_input.extend_from_slice(data);
    out.extend_from_slice(&crc32(&crc_input).to_be_bytes());
}

fn zlib_store(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() + data.len() / 65_535 * 5 + 8);
    out.extend_from_slice(&[0x78, 0x01]);

    if data.is_empty() {
        out.extend_from_slice(&[1, 0, 0, 0xff, 0xff]);
    } else {
        for (index, chunk) in data.chunks(65_535).enumerate() {
            let final_block = index == (data.len() - 1) / 65_535;
            out.push(u8::from(final_block));
            let len = chunk.len() as u16;
            out.extend_from_slice(&len.to_le_bytes());
            out.extend_from_slice(&(!len).to_le_bytes());
            out.extend_from_slice(chunk);
        }
    }

    out.extend_from_slice(&adler32(data).to_be_bytes());
    out
}

fn adler32(data: &[u8]) -> u32 {
    const MOD: u32 = 65_521;
    let mut a = 1u32;
    let mut b = 0u32;
    for byte in data {
        a = (a + u32::from(*byte)) % MOD;
        b = (b + a) % MOD;
    }
    (b << 16) | a
}

fn crc32(data: &[u8]) -> u32 {
    let mut crc = 0xffff_ffffu32;
    for byte in data {
        crc ^= u32::from(*byte);
        for _ in 0..8 {
            let mask = 0u32.wrapping_sub(crc & 1);
            crc = (crc >> 1) ^ (0xedb8_8320 & mask);
        }
    }
    !crc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn writes_minimal_rgb_png() {
        let png = encode_rgb(1, 1, &[0x12, 0x34, 0x56]).unwrap();
        assert!(png.starts_with(PNG_SIGNATURE));
        assert!(png.windows(4).any(|chunk| chunk == b"IHDR"));
        assert!(png.windows(4).any(|chunk| chunk == b"IDAT"));
        assert!(png.windows(4).any(|chunk| chunk == b"IEND"));
    }

    #[test]
    fn rejects_wrong_rgb_length() {
        let err = encode_rgb(2, 1, &[0, 1, 2]).unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidInput);
    }
}
