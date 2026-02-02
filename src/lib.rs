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

const SIMD_THRESHOLD: usize = if cfg!(feature = "simd-threshold-32") {
    32
} else if cfg!(feature = "simd-threshold-128") {
    128
} else if cfg!(feature = "simd-threshold-256") {
    256
} else {
    64
};

#[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
mod simd;

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
    if config.padding.omit {
        return unsafe { unsafe_b64_url_decode_with_omit_padding(bytes) };
    }
    unsafe { unsafe_b64_url_decode_with_padding(bytes) }
}

#[inline(always)]
unsafe fn decode_4_to_ptr(chunk: &[u8], out: *mut u8) -> *mut u8 {
    let b0 = unsafe { *chunk.get_unchecked(0) as usize };
    let b1 = unsafe { *chunk.get_unchecked(1) as usize };
    let b2 = unsafe { *chunk.get_unchecked(2) as usize };
    let b3 = unsafe { *chunk.get_unchecked(3) as usize };
    let value = ((B64_URL_DECODE[b0] as u32) << 18)
        | ((B64_URL_DECODE[b1] as u32) << 12)
        | ((B64_URL_DECODE[b2] as u32) << 6)
        | (B64_URL_DECODE[b3] as u32);
    unsafe {
        *out = ((value >> 16) & 0b1111_1111) as u8;
        *out.add(1) = ((value >> 8) & 0b1111_1111) as u8;
        *out.add(2) = (value & 0b1111_1111) as u8;
        out.add(3)
    }
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
unsafe fn unsafe_b64_url_decode_with_omit_padding(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(length * 3 / 4);
    let mut out = vec.as_mut_ptr();
    let mut out_len = 0usize;
    let mut processed = 0usize;
    #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
    if length >= SIMD_THRESHOLD {
        let mut in_ptr = bytes.as_ptr();
        #[cfg(feature = "simd-avx512")]
        if std::arch::is_x86_feature_detected!("avx512f") {
            let simd_blocks = length / 64;
            for _ in 0..simd_blocks {
                out = unsafe { simd::decode_64_bytes_avx512(in_ptr, out) };
                in_ptr = unsafe { in_ptr.add(64) };
                out_len += 48;
            }
            processed = simd_blocks * 64;
        } else if std::arch::is_x86_feature_detected!("avx2") {
            let simd_blocks = length / 32;
            for _ in 0..simd_blocks {
                out = unsafe { simd::decode_32_bytes_avx2(in_ptr, out) };
                in_ptr = unsafe { in_ptr.add(32) };
                out_len += 24;
            }
            processed = simd_blocks * 32;
        } else if std::arch::is_x86_feature_detected!("sse2") {
            let simd_blocks = length / 16;
            for _ in 0..simd_blocks {
                out = unsafe { simd::decode_16_bytes_sse2(in_ptr, out) };
                in_ptr = unsafe { in_ptr.add(16) };
                out_len += 12;
            }
            processed = simd_blocks * 16;
        }
    }
    let mut chunks = bytes[processed..].chunks_exact(4);
    for chunk in chunks.by_ref() {
        out = unsafe { decode_4_to_ptr(chunk, out) };
        out_len += 3;
    }
    let remainder = chunks.remainder();
    if remainder.len() >= 2 {
        let b0 = unsafe { *remainder.get_unchecked(0) as usize };
        let b1 = unsafe { *remainder.get_unchecked(1) as usize };
        let mut value = ((B64_URL_DECODE[b0] as u32) << 18) | ((B64_URL_DECODE[b1] as u32) << 12);
        if remainder.len() >= 3 {
            let b2 = unsafe { *remainder.get_unchecked(2) as usize };
            value |= (B64_URL_DECODE[b2] as u32) << 6;
            unsafe {
                *out = ((value >> 16) & 0b1111_1111) as u8;
                *out.add(1) = ((value >> 8) & 0b1111_1111) as u8;
            }
            out_len += 2;
        } else {
            unsafe {
                *out = ((value >> 16) & 0b1111_1111) as u8;
            }
            out_len += 1;
        }
    }
    unsafe {
        vec.set_len(out_len);
    }
    vec
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
unsafe fn unsafe_b64_url_decode_with_padding(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(length * 3 / 4);
    let mut out = vec.as_mut_ptr();
    let mut out_len = 0usize;
    let chunk_count = length / 4;
    if chunk_count > 0 {
        let bulk_chunks = chunk_count.saturating_sub(1);
        let mut processed = 0usize;
        #[cfg(all(feature = "simd", any(target_arch = "x86", target_arch = "x86_64")))]
        if bulk_chunks * 4 >= SIMD_THRESHOLD {
            let mut in_ptr = bytes.as_ptr();
            #[cfg(feature = "simd-avx512")]
            if std::arch::is_x86_feature_detected!("avx512f") {
                let simd_blocks = (bulk_chunks * 4) / 64;
                for _ in 0..simd_blocks {
                    out = unsafe { simd::decode_64_bytes_avx512(in_ptr, out) };
                    in_ptr = unsafe { in_ptr.add(64) };
                    out_len += 48;
                }
                processed = simd_blocks * 64;
            } else if std::arch::is_x86_feature_detected!("avx2") {
                let simd_blocks = (bulk_chunks * 4) / 32;
                for _ in 0..simd_blocks {
                    out = unsafe { simd::decode_32_bytes_avx2(in_ptr, out) };
                    in_ptr = unsafe { in_ptr.add(32) };
                    out_len += 24;
                }
                processed = simd_blocks * 32;
            } else if std::arch::is_x86_feature_detected!("sse2") {
                let simd_blocks = (bulk_chunks * 4) / 16;
                for _ in 0..simd_blocks {
                    out = unsafe { simd::decode_16_bytes_sse2(in_ptr, out) };
                    in_ptr = unsafe { in_ptr.add(16) };
                    out_len += 12;
                }
                processed = simd_blocks * 16;
            }
        }
        let mut chunks = bytes[processed..(bulk_chunks * 4)].chunks_exact(4);
        for chunk in chunks.by_ref() {
            out = unsafe { decode_4_to_ptr(chunk, out) };
            out_len += 3;
        }
        let last_start = bulk_chunks * 4;
        let last = unsafe { bytes.get_unchecked(last_start..last_start + 4) };
        let b0 = unsafe { *last.get_unchecked(0) as usize };
        let b1 = unsafe { *last.get_unchecked(1) as usize };
        let b2 = unsafe { *last.get_unchecked(2) };
        let b3 = unsafe { *last.get_unchecked(3) };
        let mut value = ((B64_URL_DECODE[b0] as u32) << 18) | ((B64_URL_DECODE[b1] as u32) << 12);
        if b2 != B64_URL_PAD {
            value |= (B64_URL_DECODE[b2 as usize] as u32) << 6;
            if b3 != B64_URL_PAD {
                value |= B64_URL_DECODE[b3 as usize] as u32;
                unsafe {
                    *out = ((value >> 16) & 0b1111_1111) as u8;
                    *out.add(1) = ((value >> 8) & 0b1111_1111) as u8;
                    *out.add(2) = (value & 0b1111_1111) as u8;
                }
                out_len += 3;
            } else {
                unsafe {
                    *out = ((value >> 16) & 0b1111_1111) as u8;
                    *out.add(1) = ((value >> 8) & 0b1111_1111) as u8;
                }
                out_len += 2;
            }
        } else {
            unsafe {
                *out = ((value >> 16) & 0b1111_1111) as u8;
            }
            out_len += 1;
        }
    }
    unsafe {
        vec.set_len(out_len);
    }
    vec
}
