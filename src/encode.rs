#[cfg(feature = "encode-lut")]
use super::B64_URL_ENCODE_LUT;
#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
use super::simd;
use super::{B64_URL_ENCODE, B64_URL_PAD, B64Config};
#[cfg(feature = "encode-parallel")]
use rayon::prelude::*;

#[inline(always)]
pub(crate) fn b64_url_encode_calculate_destination_capacity(length: usize) -> usize {
    length / 3 * 4 + 4
}

#[inline(always)]
pub(crate) fn b64_url_encode_calculate_exact_length(length: usize, omit_padding: bool) -> usize {
    let full = (length / 3) * 4;
    match length % 3 {
        0 => full,
        1 => {
            if omit_padding {
                full + 2
            } else {
                full + 4
            }
        }
        2 => {
            if omit_padding {
                full + 3
            } else {
                full + 4
            }
        }
        _ => full,
    }
}

#[cfg(feature = "encode-parallel")]
const PARALLEL_ENCODE_THRESHOLD: usize = 1 << 20;
#[cfg(feature = "encode-parallel")]
const PARALLEL_ENCODE_CHUNK: usize = 3 * 4096;

#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[inline(always)]
fn simd_threshold_encode_avx512() -> usize {
    option_env!("B64_URL__SIMD_THRESHOLD_ENCODE_AVX512")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(48)
}

#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[inline(always)]
fn simd_threshold_encode_avx2() -> usize {
    option_env!("B64_URL__SIMD_THRESHOLD_ENCODE_AVX2")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(24)
}

#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[inline(always)]
fn simd_threshold_encode_ssse3() -> usize {
    option_env!("B64_URL__SIMD_THRESHOLD_ENCODE_SSSE3")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(12)
}

#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
#[inline(always)]
fn simd_threshold_encode_sse2() -> usize {
    option_env!("B64_URL__SIMD_THRESHOLD_ENCODE_SSE2")
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(12)
}
#[inline(always)]
pub(crate) unsafe fn encode_tail_2_with_padding(source: *const u8, destination: *mut u8) {
    let value = unsafe { ((*source as u32) << 16) | ((*source.offset(1) as u32) << 8) };
    unsafe {
        #[cfg(feature = "encode-lut")]
        {
            let pair_hi = B64_URL_ENCODE_LUT[(value >> 12) as usize];
            *destination = (pair_hi >> 8) as u8;
            *destination.offset(1) = pair_hi as u8;
        }
        #[cfg(not(feature = "encode-lut"))]
        {
            *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
        }
        *destination.offset(2) = B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize];
        *destination.offset(3) = B64_URL_PAD;
    }
}

#[inline(always)]
pub(crate) unsafe fn encode_tail_2_without_padding(source: *const u8, destination: *mut u8) {
    let value = unsafe { ((*source as u32) << 16) | ((*source.offset(1) as u32) << 8) };
    unsafe {
        #[cfg(feature = "encode-lut")]
        {
            let pair_hi = B64_URL_ENCODE_LUT[(value >> 12) as usize];
            *destination = (pair_hi >> 8) as u8;
            *destination.offset(1) = pair_hi as u8;
        }
        #[cfg(not(feature = "encode-lut"))]
        {
            *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
        }
        *destination.offset(2) = B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize];
    }
}

#[inline(always)]
pub(crate) unsafe fn encode_tail_1_with_padding(source: *const u8, destination: *mut u8) {
    let value = unsafe { (*source as u32) << 16 };
    unsafe {
        #[cfg(feature = "encode-lut")]
        {
            let pair_hi = B64_URL_ENCODE_LUT[(value >> 12) as usize];
            *destination = (pair_hi >> 8) as u8;
            *destination.offset(1) = pair_hi as u8;
        }
        #[cfg(not(feature = "encode-lut"))]
        {
            *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
        }
        *destination.offset(2) = B64_URL_PAD;
        *destination.offset(3) = B64_URL_PAD;
    }
}

#[inline(always)]
pub(crate) unsafe fn encode_tail_1_without_padding(source: *const u8, destination: *mut u8) {
    let value = unsafe { (*source as u32) << 16 };
    unsafe {
        #[cfg(feature = "encode-lut")]
        {
            let pair_hi = B64_URL_ENCODE_LUT[(value >> 12) as usize];
            *destination = (pair_hi >> 8) as u8;
            *destination.offset(1) = pair_hi as u8;
        }
        #[cfg(not(feature = "encode-lut"))]
        {
            *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
        }
    }
}

/// # Safety
///
/// Caller must ensure `source` and `destination` are valid for reads/writes of
/// the specified lengths, and that the buffers do not overlap.
#[inline(always)]
pub(crate) unsafe fn b64_url_encode_with_config_to_ptr(
    mut source: *const u8,
    mut source_length: usize,
    mut destination: *mut u8,
    config: &B64Config,
) -> usize {
    let mut bytes = 0;
    #[cfg(feature = "encode-parallel")]
    {
        if source_length >= PARALLEL_ENCODE_THRESHOLD {
            let full_len = source_length - (source_length % 3);
            let out_full_len = (full_len / 3) * 4;
            let chunk_in = PARALLEL_ENCODE_CHUNK - (PARALLEL_ENCODE_CHUNK % 3);
            let parallel_len = full_len - (full_len % chunk_in);
            if parallel_len > 0 {
                let in_slice = std::slice::from_raw_parts(source, parallel_len);
                let out_slice = std::slice::from_raw_parts_mut(destination, out_full_len);
                let out_chunk = (chunk_in / 3) * 4;
                in_slice
                    .par_chunks_exact(chunk_in)
                    .zip(out_slice.par_chunks_exact_mut(out_chunk))
                    .for_each(|(chunk, out_chunk)| unsafe {
                        b64_url_encode_with_config_to_ptr(
                            chunk.as_ptr(),
                            chunk.len(),
                            out_chunk.as_mut_ptr(),
                            config,
                        );
                    });
                source = unsafe { source.add(parallel_len) };
                destination = unsafe { destination.add((parallel_len / 3) * 4) };
                source_length -= parallel_len;
                bytes += (parallel_len / 3) * 4;
            }
        }
    }
    #[cfg(all(
        any(feature = "simd", simd_env),
        any(target_arch = "x86", target_arch = "x86_64")
    ))]
    {
        if source_length >= 3 {
            let mut in_ptr = source;
            let mut out_ptr = destination;
            let mut left = source_length - (source_length % 3);

            #[cfg(any(feature = "simd-avx512-encode", simd_avx512_encode_env))]
            if left >= simd_threshold_encode_avx512()
                && std::arch::is_x86_feature_detected!("avx512f")
            {
                while left >= 48 {
                    out_ptr = unsafe { simd::encode_48_bytes_avx512(in_ptr, out_ptr) };
                    in_ptr = unsafe { in_ptr.add(48) };
                    left -= 48;
                    bytes += 64;
                }
            }

            #[cfg(any(feature = "simd-avx2-encode", simd_avx2_encode_env))]
            if left >= simd_threshold_encode_avx2() && std::arch::is_x86_feature_detected!("avx2") {
                while left >= 24 {
                    out_ptr = unsafe { simd::encode_24_bytes_avx2(in_ptr, out_ptr) };
                    in_ptr = unsafe { in_ptr.add(24) };
                    left -= 24;
                    bytes += 32;
                }
            }

            #[cfg(any(
                feature = "simd-ssse3-encode",
                simd_ssse3_encode_env,
                feature = "simd-sse2-encode",
                simd_sse2_encode_env
            ))]
            if left >= simd_threshold_encode_ssse3() || left >= simd_threshold_encode_sse2() {
                if (cfg!(feature = "simd-ssse3-encode") || cfg!(simd_ssse3_encode_env))
                    && left >= simd_threshold_encode_ssse3()
                    && std::arch::is_x86_feature_detected!("ssse3")
                {
                    while left >= 12 {
                        out_ptr = unsafe { simd::encode_12_bytes_ssse3(in_ptr, out_ptr) };
                        in_ptr = unsafe { in_ptr.add(12) };
                        left -= 12;
                        bytes += 16;
                    }
                } else if (cfg!(feature = "simd-sse2-encode") || cfg!(simd_sse2_encode_env))
                    && left >= simd_threshold_encode_sse2()
                    && std::arch::is_x86_feature_detected!("sse2")
                {
                    while left >= 12 {
                        out_ptr = unsafe { simd::encode_12_bytes_sse2(in_ptr, out_ptr) };
                        in_ptr = unsafe { in_ptr.add(12) };
                        left -= 12;
                        bytes += 16;
                    }
                }
            }

            source = in_ptr;
            destination = out_ptr;
            source_length = left + (source_length % 3);
        }
    }
    while source_length >= 3 {
        let value = unsafe {
            ((*source as u32) << 16)
                | ((*source.offset(1) as u32) << 8)
                | (*source.offset(2) as u32)
        };
        #[cfg(feature = "encode-lut")]
        {
            let pair_hi = B64_URL_ENCODE_LUT[(value >> 12) as usize];
            let pair_lo = B64_URL_ENCODE_LUT[(value & 0x0fff) as usize];
            unsafe {
                *destination = (pair_hi >> 8) as u8;
                *destination.offset(1) = pair_hi as u8;
                *destination.offset(2) = (pair_lo >> 8) as u8;
                *destination.offset(3) = pair_lo as u8;
                source = source.offset(3);
                destination = destination.offset(4);
            }
        }
        #[cfg(not(feature = "encode-lut"))]
        {
            unsafe {
                *destination = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
                *destination.offset(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
                *destination.offset(2) = B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize];
                *destination.offset(3) = B64_URL_ENCODE[(value & 0b11_1111) as usize];
                source = source.offset(3);
                destination = destination.offset(4);
            }
        }
        source_length -= 3;
        bytes += 4;
    }
    match source_length {
        2 => {
            if config.padding.omit {
                unsafe { encode_tail_2_without_padding(source, destination) };
                bytes += 3;
            } else {
                unsafe { encode_tail_2_with_padding(source, destination) };
                bytes += 4;
            }
        }
        1 => {
            if config.padding.omit {
                unsafe { encode_tail_1_without_padding(source, destination) };
                bytes += 2;
            } else {
                unsafe { encode_tail_1_with_padding(source, destination) };
                bytes += 4;
            }
        }
        _ => {}
    }
    bytes
}
