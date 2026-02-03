#[cfg(feature = "encode-lut")]
use super::B64_URL_ENCODE_LUT;
use super::{B64_URL_DECODE, B64_URL_ENCODE};
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline(always)]
unsafe fn decode_4_from_bytes(b0: u8, b1: u8, b2: u8, b3: u8, out: *mut u8) -> *mut u8 {
    let b0 = b0 as usize;
    let b1 = b1 as usize;
    let b2 = b2 as usize;
    let b3 = b3 as usize;
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

#[inline(always)]
unsafe fn encode_3_from_bytes(b0: u8, b1: u8, b2: u8, out: *mut u8) -> *mut u8 {
    let value = ((b0 as u32) << 16) | ((b1 as u32) << 8) | (b2 as u32);
    #[cfg(feature = "encode-lut")]
    {
        let pair_hi = B64_URL_ENCODE_LUT[(value >> 12) as usize];
        let pair_lo = B64_URL_ENCODE_LUT[(value & 0x0fff) as usize];
        unsafe {
            *out = (pair_hi >> 8) as u8;
            *out.add(1) = pair_hi as u8;
            *out.add(2) = (pair_lo >> 8) as u8;
            *out.add(3) = pair_lo as u8;
            return out.add(4);
        }
    }
    #[cfg(not(feature = "encode-lut"))]
    {
        unsafe {
            *out = B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize];
            *out.add(1) = B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize];
            *out.add(2) = B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize];
            *out.add(3) = B64_URL_ENCODE[(value & 0b11_1111) as usize];
            return out.add(4);
        }
    }
}

#[inline(always)]
unsafe fn encode_12_from_m128i(v: __m128i, out: *mut u8) -> *mut u8 {
    let lo = unsafe { _mm_cvtsi128_si64(v) } as u64;
    let hi = unsafe { _mm_cvtsi128_si64(_mm_srli_si128(v, 8)) } as u64;
    let b0 = (lo & 0xff) as u8;
    let b1 = ((lo >> 8) & 0xff) as u8;
    let b2 = ((lo >> 16) & 0xff) as u8;
    let b3 = ((lo >> 24) & 0xff) as u8;
    let b4 = ((lo >> 32) & 0xff) as u8;
    let b5 = ((lo >> 40) & 0xff) as u8;
    let b6 = ((lo >> 48) & 0xff) as u8;
    let b7 = ((lo >> 56) & 0xff) as u8;
    let b8 = (hi & 0xff) as u8;
    let b9 = ((hi >> 8) & 0xff) as u8;
    let b10 = ((hi >> 16) & 0xff) as u8;
    let b11 = ((hi >> 24) & 0xff) as u8;
    let mut out_ptr = out;
    out_ptr = unsafe { encode_3_from_bytes(b0, b1, b2, out_ptr) };
    out_ptr = unsafe { encode_3_from_bytes(b3, b4, b5, out_ptr) };
    out_ptr = unsafe { encode_3_from_bytes(b6, b7, b8, out_ptr) };
    out_ptr = unsafe { encode_3_from_bytes(b9, b10, b11, out_ptr) };
    out_ptr
}

#[inline(always)]
unsafe fn encode_24_from_m256i(v: __m256i, out: *mut u8) -> *mut u8 {
    let lo = unsafe { _mm256_castsi256_si128(v) };
    let hi = unsafe { _mm256_extracti128_si256(v, 1) };
    let mut out_ptr = out;
    out_ptr = unsafe { encode_12_from_m128i(lo, out_ptr) };
    out_ptr = unsafe { encode_12_from_m128i(hi, out_ptr) };
    out_ptr
}

#[inline(always)]
unsafe fn decode_4_from_ptr(input: *const u8, out: *mut u8) -> *mut u8 {
    let b0 = unsafe { *input };
    let b1 = unsafe { *input.add(1) };
    let b2 = unsafe { *input.add(2) };
    let b3 = unsafe { *input.add(3) };
    unsafe { decode_4_from_bytes(b0, b1, b2, b3, out) }
}

#[inline(always)]
unsafe fn decode_4_from_u32_le(value: u32, out: *mut u8) -> *mut u8 {
    let b0 = (value & 0xff) as u8;
    let b1 = ((value >> 8) & 0xff) as u8;
    let b2 = ((value >> 16) & 0xff) as u8;
    let b3 = ((value >> 24) & 0xff) as u8;
    unsafe { decode_4_from_bytes(b0, b1, b2, b3, out) }
}

#[inline(always)]
unsafe fn decode_16_from_m128i(v: __m128i, out: *mut u8) -> *mut u8 {
    let lo = unsafe { _mm_cvtsi128_si64(v) } as u64;
    let hi = unsafe { _mm_cvtsi128_si64(_mm_srli_si128(v, 8)) } as u64;
    let mut out_ptr = out;
    out_ptr = unsafe { decode_4_from_u32_le(lo as u32, out_ptr) };
    out_ptr = unsafe { decode_4_from_u32_le((lo >> 32) as u32, out_ptr) };
    out_ptr = unsafe { decode_4_from_u32_le(hi as u32, out_ptr) };
    out_ptr = unsafe { decode_4_from_u32_le((hi >> 32) as u32, out_ptr) };
    out_ptr
}

#[inline(always)]
unsafe fn decode_32_from_m256i(v: __m256i, out: *mut u8) -> *mut u8 {
    let lo = unsafe { _mm256_castsi256_si128(v) };
    let hi = unsafe { _mm256_extracti128_si256(v, 1) };
    let mut out_ptr = out;
    out_ptr = unsafe { decode_16_from_m128i(lo, out_ptr) };
    out_ptr = unsafe { decode_16_from_m128i(hi, out_ptr) };
    out_ptr
}

#[cfg(feature = "simd-sse2")]
#[cfg(any(feature = "simd-sse2-decode", simd_sse2_env))]
#[target_feature(enable = "sse2")]
pub unsafe fn decode_16_bytes_sse2(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm_loadu_si128(input as *const __m128i) };
    unsafe { decode_16_from_m128i(v, out) }
}

#[cfg(any(feature = "simd-sse2-encode", simd_sse2_encode_env))]
#[target_feature(enable = "sse2")]
pub unsafe fn encode_12_bytes_sse2(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm_loadu_si128(input as *const __m128i) };
    unsafe { encode_12_from_m128i(v, out) }
}

#[cfg(any(feature = "simd-ssse3-encode", simd_ssse3_encode_env))]
#[target_feature(enable = "ssse3")]
pub unsafe fn encode_12_bytes_ssse3(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm_loadu_si128(input as *const __m128i) };
    unsafe { encode_12_from_m128i(v, out) }
}

#[cfg(any(feature = "simd-avx2-decode", simd_avx2_env))]
#[target_feature(enable = "avx2")]
pub unsafe fn decode_32_bytes_avx2(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm256_loadu_si256(input as *const __m256i) };
    unsafe { decode_32_from_m256i(v, out) }
}

#[cfg(any(feature = "simd-avx2-encode", simd_avx2_encode_env))]
#[target_feature(enable = "avx2")]
pub unsafe fn encode_24_bytes_avx2(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm256_loadu_si256(input as *const __m256i) };
    unsafe { encode_24_from_m256i(v, out) }
}

#[cfg(any(feature = "simd-avx512-decode", simd_avx512_env))]
#[target_feature(enable = "avx512f,avx512dq")]
pub unsafe fn decode_64_bytes_avx512(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm512_loadu_si512(input as *const __m512i) };
    let lo = unsafe { _mm512_castsi512_si256(v) };
    let hi = unsafe { _mm512_extracti64x4_epi64(v, 1) };
    let mut out_ptr = out;
    out_ptr = unsafe { decode_32_from_m256i(lo, out_ptr) };
    out_ptr = unsafe { decode_32_from_m256i(hi, out_ptr) };
    out_ptr
}

#[cfg(any(feature = "simd-avx512-encode", simd_avx512_encode_env))]
#[target_feature(enable = "avx512f,avx512dq")]
pub unsafe fn encode_48_bytes_avx512(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm512_loadu_si512(input as *const __m512i) };
    let lo = unsafe { _mm512_castsi512_si256(v) };
    let hi = unsafe { _mm512_extracti64x4_epi64(v, 1) };
    let mut out_ptr = out;
    out_ptr = unsafe { encode_24_from_m256i(lo, out_ptr) };
    out_ptr = unsafe { encode_24_from_m256i(hi, out_ptr) };
    out_ptr
}
