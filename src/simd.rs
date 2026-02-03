use super::B64_URL_DECODE;
#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[inline(always)]
unsafe fn decode_4_from_ptr(input: *const u8, out: *mut u8) -> *mut u8 {
    let b0 = unsafe { *input as usize };
    let b1 = unsafe { *input.add(1) as usize };
    let b2 = unsafe { *input.add(2) as usize };
    let b3 = unsafe { *input.add(3) as usize };
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

#[cfg(feature = "simd-sse2")]
#[target_feature(enable = "sse2")]
pub unsafe fn decode_16_bytes_sse2(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm_loadu_si128(input as *const __m128i) };
    let mut tmp = [0u8; 16];
    unsafe { _mm_storeu_si128(tmp.as_mut_ptr() as *mut __m128i, v) };
    let mut out_ptr = out;
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr(), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(4), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(8), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(12), out_ptr) };
    out_ptr
}

#[cfg(feature = "simd-avx2")]
#[target_feature(enable = "avx2")]
pub unsafe fn decode_32_bytes_avx2(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm256_loadu_si256(input as *const __m256i) };
    let mut tmp = [0u8; 32];
    unsafe { _mm256_storeu_si256(tmp.as_mut_ptr() as *mut __m256i, v) };
    let mut out_ptr = out;
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr(), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(4), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(8), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(12), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(16), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(20), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(24), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(28), out_ptr) };
    out_ptr
}

#[cfg(feature = "simd-avx512")]
#[target_feature(enable = "avx512f")]
pub unsafe fn decode_64_bytes_avx512(input: *const u8, out: *mut u8) -> *mut u8 {
    let v = unsafe { _mm512_loadu_si512(input as *const __m512i) };
    let mut tmp = [0u8; 64];
    unsafe { _mm512_storeu_si512(tmp.as_mut_ptr() as *mut __m512i, v) };
    let mut out_ptr = out;
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr(), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(4), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(8), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(12), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(16), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(20), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(24), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(28), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(32), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(36), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(40), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(44), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(48), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(52), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(56), out_ptr) };
    out_ptr = unsafe { decode_4_from_ptr(tmp.as_ptr().add(60), out_ptr) };
    out_ptr
}
