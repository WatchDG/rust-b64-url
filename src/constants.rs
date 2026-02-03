pub(crate) const B64_URL_ENCODE: [u8; 64] =
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

#[cfg(feature = "encode-lut")]
const fn build_b64_url_encode_lut() -> [u16; 4096] {
    let mut table = [0u16; 4096];
    let mut i = 0usize;
    while i < 4096 {
        let value = i as u16;
        let c0 = B64_URL_ENCODE[((value >> 6) & 0x3f) as usize] as u16;
        let c1 = B64_URL_ENCODE[(value & 0x3f) as usize] as u16;
        table[i] = (c0 << 8) | c1;
        i += 1;
    }
    table
}

#[cfg(feature = "encode-lut")]
pub(crate) const B64_URL_ENCODE_LUT: [u16; 4096] = build_b64_url_encode_lut();

const fn build_b64_url_decode_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 64 {
        table[B64_URL_ENCODE[i] as usize] = i as u8;
        i += 1;
    }
    table
}

pub(crate) const B64_URL_DECODE: [u8; 256] = build_b64_url_decode_table();

const fn build_b64_url_decode_valid_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 64 {
        table[B64_URL_ENCODE[i] as usize] = 1;
        i += 1;
    }
    table
}

pub(crate) const B64_URL_DECODE_VALID: [u8; 256] = build_b64_url_decode_valid_table();

pub(crate) const B64_URL_PAD: u8 = 0x3d;

#[cfg(feature = "encode-parallel")]
pub(crate) const PARALLEL_ENCODE_THRESHOLD: usize = 1 << 20;

#[cfg(feature = "encode-parallel")]
pub(crate) const PARALLEL_ENCODE_CHUNK: usize = 3 * 4096;

#[cfg(feature = "decode-parallel")]
pub(crate) const PARALLEL_DECODE_THRESHOLD: usize = 1 << 20;
