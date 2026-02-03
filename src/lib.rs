mod constants;
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

const DEFAULT_CONFIG: B64Config = B64Config {
    padding: B64ConfigPadding { omit: false },
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum B64DecodeError {
    InvalidLength,
    InvalidPadding,
    InvalidByte { index: usize, byte: u8 },
    OutputTooSmall { needed: usize, available: usize },
}

#[inline(always)]
pub fn b64_url_encode_with_config(bytes: &[u8], config: &B64Config) -> Vec<u8> {
    #[cfg(feature = "encode-empty-check")]
    if bytes.is_empty() {
        return Vec::new();
    }
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
    #[cfg(feature = "encode-empty-check")]
    if bytes.is_empty() {
        return Some(0);
    }
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
    #[cfg(feature = "encode-empty-check")]
    if bytes.is_empty() {
        return Vec::new();
    }
    b64_url_encode_with_config(bytes, &DEFAULT_CONFIG)
}

#[inline(always)]
pub fn b64_url_encode_into(bytes: &[u8], out: &mut [u8]) -> Option<usize> {
    #[cfg(feature = "encode-empty-check")]
    if bytes.is_empty() {
        return Some(0);
    }
    b64_url_encode_into_with_config(bytes, out, &DEFAULT_CONFIG)
}

#[inline(always)]
pub fn b64_url_decode(bytes: &[u8]) -> Result<Vec<u8>, B64DecodeError> {
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    b64_url_decode_with_config(bytes, &DEFAULT_CONFIG)
}

#[inline(always)]
pub fn b64_url_decode_with_config(
    bytes: &[u8],
    config: &B64Config,
) -> Result<Vec<u8>, B64DecodeError> {
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Ok(Vec::new());
    }
    let needed = decode::b64_url_decode_validate(bytes, config.padding.omit)?;
    let mut vec = Vec::<u8>::with_capacity(needed);
    let written = if config.padding.omit {
        unsafe { decode::unsafe_b64_url_decode_with_omit_padding_to_ptr(bytes, vec.as_mut_ptr()) }
    } else {
        unsafe { decode::unsafe_b64_url_decode_with_padding_to_ptr(bytes, vec.as_mut_ptr()) }
    };
    unsafe {
        vec.set_len(written);
    }
    Ok(vec)
}

#[inline(always)]
pub fn b64_url_decode_into_with_config(
    bytes: &[u8],
    out: &mut [u8],
    config: &B64Config,
) -> Result<usize, B64DecodeError> {
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Ok(0);
    }
    let needed = decode::b64_url_decode_validate(bytes, config.padding.omit)?;
    if out.len() < needed {
        return Err(B64DecodeError::OutputTooSmall {
            needed,
            available: out.len(),
        });
    }
    let written = if config.padding.omit {
        unsafe { decode::unsafe_b64_url_decode_with_omit_padding_to_ptr(bytes, out.as_mut_ptr()) }
    } else {
        unsafe { decode::unsafe_b64_url_decode_with_padding_to_ptr(bytes, out.as_mut_ptr()) }
    };
    Ok(written)
}

#[inline(always)]
pub fn b64_url_decode_into(bytes: &[u8], out: &mut [u8]) -> Result<usize, B64DecodeError> {
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Ok(0);
    }
    b64_url_decode_into_with_config(bytes, out, &DEFAULT_CONFIG)
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn unsafe_b64_url_decode(bytes: &[u8]) -> Vec<u8> {
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Vec::new();
    }
    unsafe { unsafe_b64_url_decode_with_config(bytes, &DEFAULT_CONFIG) }
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn unsafe_b64_url_decode_with_config(bytes: &[u8], config: &B64Config) -> Vec<u8> {
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Vec::new();
    }
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
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Some(0);
    }
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
    #[cfg(feature = "decode-empty-check")]
    if bytes.is_empty() {
        return Some(0);
    }
    unsafe { unsafe_b64_url_decode_into_with_config(bytes, out, &DEFAULT_CONFIG) }
}
