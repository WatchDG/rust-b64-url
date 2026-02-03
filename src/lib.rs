const B64_URL_ENCODE: [u8; 64] =
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
const B64_URL_ENCODE_LUT: [u16; 4096] = build_b64_url_encode_lut();

const fn build_b64_url_decode_table() -> [u8; 256] {
    let mut table = [0u8; 256];
    let mut i = 0;
    while i < 64 {
        table[B64_URL_ENCODE[i] as usize] = i as u8;
        i += 1;
    }
    table
}

const B64_URL_DECODE: [u8; 256] = build_b64_url_decode_table();

const B64_URL_PAD: u8 = 0x3d;

const DEFAULT_CONFIG: B64Config = B64Config {
    padding: B64ConfigPadding { omit: false },
};

#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
const SIMD_THRESHOLD: usize = match option_env!("B64_URL__SIMD_THRESHOLD") {
    Some("32") => 32,
    Some("128") => 128,
    Some("256") => 256,
    _ => 64,
};

mod decode;
mod encode;
#[cfg(all(
    any(feature = "simd", simd_env),
    any(target_arch = "x86", target_arch = "x86_64")
))]
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
pub fn b64_url_encode_with_config(bytes: &[u8], config: &B64Config) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(encode::b64_url_encode_calculate_destination_capacity(
        length,
    ));
    unsafe {
        let bytes = encode::b64_url_encode_with_config_to_ptr(
            bytes.as_ptr(),
            length,
            vec.as_mut_ptr(),
            config,
        );
        vec.set_len(bytes);
    }
    vec
}

#[inline(always)]
pub fn b64_url_encode_into_with_config(
    bytes: &[u8],
    out: &mut [u8],
    config: &B64Config,
) -> Option<usize> {
    let needed = encode::b64_url_encode_calculate_exact_length(bytes.len(), config.padding.omit);
    if out.len() < needed {
        return None;
    }
    let written = unsafe {
        encode::b64_url_encode_with_config_to_ptr(
            bytes.as_ptr(),
            bytes.len(),
            out.as_mut_ptr(),
            config,
        )
    };
    Some(written)
}

#[inline(always)]
pub fn b64_url_encode(bytes: &[u8]) -> Vec<u8> {
    b64_url_encode_with_config(bytes, &DEFAULT_CONFIG)
}

#[inline(always)]
pub fn b64_url_encode_into(bytes: &[u8], out: &mut [u8]) -> Option<usize> {
    b64_url_encode_into_with_config(bytes, out, &DEFAULT_CONFIG)
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
        return unsafe { decode::unsafe_b64_url_decode_with_omit_padding(bytes) };
    }
    unsafe { decode::unsafe_b64_url_decode_with_padding(bytes) }
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn unsafe_b64_url_decode_into_with_config(
    bytes: &[u8],
    out: &mut [u8],
    config: &B64Config,
) -> Option<usize> {
    let needed = decode::b64_url_decode_calculate_exact_length(bytes, config.padding.omit)?;
    if out.len() < needed {
        return None;
    }
    let written = if config.padding.omit {
        unsafe { decode::unsafe_b64_url_decode_with_omit_padding_to_ptr(bytes, out.as_mut_ptr()) }
    } else {
        unsafe { decode::unsafe_b64_url_decode_with_padding_to_ptr(bytes, out.as_mut_ptr()) }
    };
    Some(written)
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn unsafe_b64_url_decode_into(bytes: &[u8], out: &mut [u8]) -> Option<usize> {
    unsafe { unsafe_b64_url_decode_into_with_config(bytes, out, &DEFAULT_CONFIG) }
}
