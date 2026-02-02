pub const B64_URL_ENCODE: [u8; 64] =
    *b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

const fn build_b64_url_decode_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 64 {
        table[B64_URL_ENCODE[i] as usize] = i as u8;
        i += 1;
    }
    table
}

pub const B64_URL_DECODE: [u8; 256] = build_b64_url_decode_table();

pub const B64_URL_PAD: u8 = 0x3d;

const DEFAULT_CONFIG: B64Config = B64Config {
    padding: B64ConfigPadding { omit: false },
};

#[derive(Default)]
pub struct B64ConfigPadding {
    pub omit: bool,
}

#[derive(Default)]
pub struct B64Config {
    pub padding: B64ConfigPadding,
}

#[inline(always)]
pub fn _b64_url_encode_calculate_destination_capacity(length: usize) -> usize {
    length / 3 * 4 + 4
}

#[inline(always)]
pub unsafe fn _b64_url_encode_with_config(
    mut source: *const u8,
    mut source_length: usize,
    mut destination: *mut u8,
    config: &B64Config,
) -> usize {
    let mut bytes = 0;
    while source_length >= 3 {
        let value = ((*source as u32) << 16)
            | ((*source.offset(1) as u32) << 8)
            | (*source.offset(2) as u32);
        *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
        *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
        *destination.offset(2) = B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize];
        *destination.offset(3) = B64_URL_ENCODE[(value & 0b11_1111) as usize];
        source = source.offset(3);
        destination = destination.offset(4);
        source_length -= 3;
        bytes += 4;
    }
    match source_length {
        2 => {
            let value = ((*source as u32) << 16) | ((*source.offset(1) as u32) << 8);
            *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
            *destination.offset(2) = B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize];
            if !config.padding.omit {
                *destination.offset(3) = B64_URL_PAD;
                bytes += 4;
            } else {
                bytes += 3;
            }
        }
        1 => {
            let value = (*source as u32) << 16;
            *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
            if !config.padding.omit {
                *destination.offset(2) = B64_URL_PAD;
                *destination.offset(3) = B64_URL_PAD;
                bytes += 4;
            } else {
                bytes += 2;
            }
        }
        _ => {}
    }
    bytes
}

#[inline(always)]
pub fn b64_url_encode_with_config(bytes: &[u8], config: &B64Config) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(_b64_url_encode_calculate_destination_capacity(length));
    unsafe {
        let bytes = _b64_url_encode_with_config(bytes.as_ptr(), length, vec.as_mut_ptr(), config);
        vec.set_len(bytes);
    }
    vec
}

#[inline(always)]
pub fn b64_url_encode(bytes: &[u8]) -> Vec<u8> {
    b64_url_encode_with_config(bytes, &DEFAULT_CONFIG)
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn unsafe_b64_url_decode(bytes: &[u8]) -> Vec<u8> {
    unsafe { unsafe_b64_url_decode_with_config(bytes, &DEFAULT_CONFIG) }
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn unsafe_b64_url_decode_with_config(bytes: &[u8], config: &B64Config) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(length * 3 / 4);
    let mut index = 0;
    if length > 4 {
        while index < length - 4 {
            let value = ((B64_URL_DECODE[bytes[index] as usize] as u32) << 18)
                | ((B64_URL_DECODE[bytes[index + 1] as usize] as u32) << 12)
                | ((B64_URL_DECODE[bytes[index + 2] as usize] as u32) << 6)
                | (B64_URL_DECODE[bytes[index + 3] as usize] as u32);
            vec.push(((value >> 16) & 0b1111_1111) as u8);
            vec.push(((value >> 8) & 0b1111_1111) as u8);
            vec.push((value & 0b1111_1111) as u8);
            index += 4;
        }
    }
    if config.padding.omit {
        if index + 2 <= length {
            let mut value = ((B64_URL_DECODE[bytes[index] as usize] as u32) << 18)
                | ((B64_URL_DECODE[bytes[index + 1] as usize] as u32) << 12);
            if index + 3 <= length {
                value |= (B64_URL_DECODE[bytes[index + 2] as usize] as u32) << 6;
                if index + 4 <= length {
                    value |= B64_URL_DECODE[bytes[index + 3] as usize] as u32;
                    vec.push(((value >> 16) & 0b1111_1111) as u8);
                    vec.push(((value >> 8) & 0b1111_1111) as u8);
                    vec.push((value & 0b1111_1111) as u8);
                } else {
                    vec.push(((value >> 16) & 0b1111_1111) as u8);
                    vec.push(((value >> 8) & 0b1111_1111) as u8);
                }
            } else {
                vec.push(((value >> 16) & 0b1111_1111) as u8);
            }
        }
        return vec;
    }
    if index + 4 == length {
        let mut value = ((B64_URL_DECODE[bytes[index] as usize] as u32) << 18)
            | ((B64_URL_DECODE[bytes[index + 1] as usize] as u32) << 12);
        if bytes[index + 2] != B64_URL_PAD {
            value |= (B64_URL_DECODE[bytes[index + 2] as usize] as u32) << 6;
            if bytes[index + 3] != B64_URL_PAD {
                value |= B64_URL_DECODE[bytes[index + 3] as usize] as u32;
                vec.push(((value >> 16) & 0b1111_1111) as u8);
                vec.push(((value >> 8) & 0b1111_1111) as u8);
                vec.push((value & 0b1111_1111) as u8);
            } else {
                vec.push(((value >> 16) & 0b1111_1111) as u8);
                vec.push(((value >> 8) & 0b1111_1111) as u8);
            }
        } else {
            vec.push(((value >> 16) & 0b1111_1111) as u8);
        }
    }
    vec
}
