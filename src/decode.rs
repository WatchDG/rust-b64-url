use super::{B64_URL_DECODE, B64_URL_PAD};
#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
use super::{SIMD_THRESHOLD, simd};

#[inline(always)]
pub(crate) fn b64_url_decode_calculate_exact_length(
    bytes: &[u8],
    omit_padding: bool,
) -> Option<usize> {
    let length = bytes.len();
    if omit_padding {
        let full = (length / 4) * 3;
        match length % 4 {
            0 => Some(full),
            2 => Some(full + 1),
            3 => Some(full + 2),
            _ => None,
        }
    } else {
        if length % 4 != 0 {
            return None;
        }
        let mut pad = 0;
        if length >= 1 && bytes[length - 1] == B64_URL_PAD {
            pad += 1;
        }
        if length >= 2 && bytes[length - 2] == B64_URL_PAD {
            pad += 1;
        }
        Some((length / 4) * 3 - pad)
    }
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
pub(crate) unsafe fn unsafe_b64_url_decode_with_omit_padding(bytes: &[u8]) -> Vec<u8> {
    let mut vec = Vec::<u8>::with_capacity(bytes.len() * 3 / 4);
    let out_len =
        unsafe { unsafe_b64_url_decode_with_omit_padding_to_ptr(bytes, vec.as_mut_ptr()) };
    unsafe {
        vec.set_len(out_len);
    }
    vec
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub(crate) unsafe fn unsafe_b64_url_decode_with_omit_padding_to_ptr(
    bytes: &[u8],
    out: *mut u8,
) -> usize {
    let length = bytes.len();
    let mut out_ptr = out;
    let mut out_len = 0usize;
    #[cfg(all(
        any(feature = "simd", simd_env),
        any(target_arch = "x86", target_arch = "x86_64")
    ))]
    let mut processed = 0usize;
    #[cfg(not(all(
        any(feature = "simd", simd_env),
        any(target_arch = "x86", target_arch = "x86_64")
    )))]
    let processed = 0usize;
    #[cfg(all(
        any(feature = "simd", simd_env),
        any(target_arch = "x86", target_arch = "x86_64")
    ))]
    if length >= SIMD_THRESHOLD {
        let mut in_ptr = bytes.as_ptr();
        #[cfg(any(feature = "simd-avx512-decode", simd_avx512_env))]
        if std::arch::is_x86_feature_detected!("avx512f") {
            let simd_blocks = length / 64;
            for _ in 0..simd_blocks {
                out_ptr = unsafe { simd::decode_64_bytes_avx512(in_ptr, out_ptr) };
                in_ptr = unsafe { in_ptr.add(64) };
                out_len += 48;
            }
            processed = simd_blocks * 64;
        } else if (cfg!(feature = "simd-avx2-decode") || cfg!(simd_avx2_env))
            && std::arch::is_x86_feature_detected!("avx2")
        {
            let simd_blocks = length / 32;
            for _ in 0..simd_blocks {
                out_ptr = unsafe { simd::decode_32_bytes_avx2(in_ptr, out_ptr) };
                in_ptr = unsafe { in_ptr.add(32) };
                out_len += 24;
            }
            processed = simd_blocks * 32;
        } else if (cfg!(feature = "simd-ssse3-decode") || cfg!(simd_ssse3_decode_env))
            && std::arch::is_x86_feature_detected!("ssse3")
        {
            let simd_blocks = length / 16;
            for _ in 0..simd_blocks {
                out_ptr = unsafe { simd::decode_16_bytes_ssse3(in_ptr, out_ptr) };
                in_ptr = unsafe { in_ptr.add(16) };
                out_len += 12;
            }
            processed = simd_blocks * 16;
        } else if (cfg!(feature = "simd-sse2-decode") || cfg!(simd_sse2_env))
            && std::arch::is_x86_feature_detected!("sse2")
        {
            let simd_blocks = length / 16;
            for _ in 0..simd_blocks {
                out_ptr = unsafe { simd::decode_16_bytes_sse2(in_ptr, out_ptr) };
                in_ptr = unsafe { in_ptr.add(16) };
                out_len += 12;
            }
            processed = simd_blocks * 16;
        }
    }
    let mut chunks = bytes[processed..].chunks_exact(4);
    for chunk in chunks.by_ref() {
        out_ptr = unsafe { decode_4_to_ptr(chunk, out_ptr) };
        out_len += 3;
    }
    let remainder = chunks.remainder();
    match remainder.len() {
        2 => {
            let b0 = unsafe { *remainder.get_unchecked(0) as usize };
            let b1 = unsafe { *remainder.get_unchecked(1) as usize };
            let value = ((B64_URL_DECODE[b0] as u32) << 18) | ((B64_URL_DECODE[b1] as u32) << 12);
            unsafe {
                *out_ptr = ((value >> 16) & 0b1111_1111) as u8;
            }
            out_len += 1;
        }
        3 => {
            let b0 = unsafe { *remainder.get_unchecked(0) as usize };
            let b1 = unsafe { *remainder.get_unchecked(1) as usize };
            let b2 = unsafe { *remainder.get_unchecked(2) as usize };
            let value = ((B64_URL_DECODE[b0] as u32) << 18)
                | ((B64_URL_DECODE[b1] as u32) << 12)
                | ((B64_URL_DECODE[b2] as u32) << 6);
            unsafe {
                *out_ptr = ((value >> 16) & 0b1111_1111) as u8;
                *out_ptr.add(1) = ((value >> 8) & 0b1111_1111) as u8;
            }
            out_len += 2;
        }
        _ => {}
    }
    out_len
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub(crate) unsafe fn unsafe_b64_url_decode_with_padding(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(length * 3 / 4);
    let out_len = unsafe { unsafe_b64_url_decode_with_padding_to_ptr(bytes, vec.as_mut_ptr()) };
    unsafe {
        vec.set_len(out_len);
    }
    vec
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub(crate) unsafe fn unsafe_b64_url_decode_with_padding_to_ptr(
    bytes: &[u8],
    out: *mut u8,
) -> usize {
    let length = bytes.len();
    let mut out_ptr = out;
    let mut out_len = 0usize;
    let chunk_count = length / 4;
    if chunk_count > 0 {
        let bulk_chunks = chunk_count.saturating_sub(1);
        #[cfg(all(
            any(feature = "simd", simd_env),
            any(target_arch = "x86", target_arch = "x86_64")
        ))]
        let mut processed = 0usize;
        #[cfg(not(all(
            any(feature = "simd", simd_env),
            any(target_arch = "x86", target_arch = "x86_64")
        )))]
        let processed = 0usize;
        #[cfg(all(
            any(feature = "simd", simd_env),
            any(target_arch = "x86", target_arch = "x86_64")
        ))]
        if bulk_chunks * 4 >= SIMD_THRESHOLD {
            let mut in_ptr = bytes.as_ptr();
            #[cfg(any(feature = "simd-avx512-decode", simd_avx512_env))]
            if std::arch::is_x86_feature_detected!("avx512f") {
                let simd_blocks = (bulk_chunks * 4) / 64;
                for _ in 0..simd_blocks {
                    out_ptr = unsafe { simd::decode_64_bytes_avx512(in_ptr, out_ptr) };
                    in_ptr = unsafe { in_ptr.add(64) };
                    out_len += 48;
                }
                processed = simd_blocks * 64;
            } else if (cfg!(feature = "simd-avx2-decode") || cfg!(simd_avx2_env))
                && std::arch::is_x86_feature_detected!("avx2")
            {
                let simd_blocks = (bulk_chunks * 4) / 32;
                for _ in 0..simd_blocks {
                    out_ptr = unsafe { simd::decode_32_bytes_avx2(in_ptr, out_ptr) };
                    in_ptr = unsafe { in_ptr.add(32) };
                    out_len += 24;
                }
                processed = simd_blocks * 32;
            } else if (cfg!(feature = "simd-ssse3-decode") || cfg!(simd_ssse3_decode_env))
                && std::arch::is_x86_feature_detected!("ssse3")
            {
                let simd_blocks = (bulk_chunks * 4) / 16;
                for _ in 0..simd_blocks {
                    out_ptr = unsafe { simd::decode_16_bytes_ssse3(in_ptr, out_ptr) };
                    in_ptr = unsafe { in_ptr.add(16) };
                    out_len += 12;
                }
                processed = simd_blocks * 16;
            } else if (cfg!(feature = "simd-sse2-decode") || cfg!(simd_sse2_env))
                && std::arch::is_x86_feature_detected!("sse2")
            {
                let simd_blocks = (bulk_chunks * 4) / 16;
                for _ in 0..simd_blocks {
                    out_ptr = unsafe { simd::decode_16_bytes_sse2(in_ptr, out_ptr) };
                    in_ptr = unsafe { in_ptr.add(16) };
                    out_len += 12;
                }
                processed = simd_blocks * 16;
            }
        }
        let mut chunks = bytes[processed..(bulk_chunks * 4)].chunks_exact(4);
        for chunk in chunks.by_ref() {
            out_ptr = unsafe { decode_4_to_ptr(chunk, out_ptr) };
            out_len += 3;
        }
        let last_start = bulk_chunks * 4;
        let last = unsafe { bytes.get_unchecked(last_start..last_start + 4) };
        let b0 = unsafe { *last.get_unchecked(0) as usize };
        let b1 = unsafe { *last.get_unchecked(1) as usize };
        let b2 = unsafe { *last.get_unchecked(2) };
        let b3 = unsafe { *last.get_unchecked(3) };
        let mut value = ((B64_URL_DECODE[b0] as u32) << 18) | ((B64_URL_DECODE[b1] as u32) << 12);
        match (b2 == B64_URL_PAD, b3 == B64_URL_PAD) {
            (true, _) => {
                unsafe {
                    *out_ptr = ((value >> 16) & 0b1111_1111) as u8;
                }
                out_len += 1;
            }
            (false, true) => {
                value |= (B64_URL_DECODE[b2 as usize] as u32) << 6;
                unsafe {
                    *out_ptr = ((value >> 16) & 0b1111_1111) as u8;
                    *out_ptr.add(1) = ((value >> 8) & 0b1111_1111) as u8;
                }
                out_len += 2;
            }
            (false, false) => {
                value |= (B64_URL_DECODE[b2 as usize] as u32) << 6;
                value |= B64_URL_DECODE[b3 as usize] as u32;
                unsafe {
                    *out_ptr = ((value >> 16) & 0b1111_1111) as u8;
                    *out_ptr.add(1) = ((value >> 8) & 0b1111_1111) as u8;
                    *out_ptr.add(2) = (value & 0b1111_1111) as u8;
                }
                out_len += 3;
            }
        }
    }
    out_len
}
