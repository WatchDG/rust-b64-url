pub const B64_URL_ENCODE: [u8; 64] = [
    0x41, 0x42, 0x43, 0x44, 0x45, 0x46, 0x47, 0x48, 0x49, 0x4a, 0x4b, 0x4c, 0x4d, 0x4e, 0x4f, 0x50,
    0x51, 0x52, 0x53, 0x54, 0x55, 0x56, 0x57, 0x58, 0x59, 0x5a, 0x61, 0x62, 0x63, 0x64, 0x65, 0x66,
    0x67, 0x68, 0x69, 0x6a, 0x6b, 0x6c, 0x6d, 0x6e, 0x6f, 0x70, 0x71, 0x72, 0x73, 0x74, 0x75, 0x76,
    0x77, 0x78, 0x79, 0x7a, 0x30, 0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x2d, 0x5f,
];

pub const B64_URL_DECODE: [u8; 255] = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x3e, 0x00, 0x00,
    0x34, 0x35, 0x36, 0x37, 0x38, 0x39, 0x3a, 0x3b, 0x3c, 0x3d, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
    0x0f, 0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x00, 0x00, 0x00, 0x00, 0x3f,
    0x00, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20, 0x21, 0x22, 0x23, 0x24, 0x25, 0x26, 0x27, 0x28,
    0x29, 0x2a, 0x2b, 0x2c, 0x2d, 0x2e, 0x2f, 0x30, 0x31, 0x32, 0x33, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];

pub const B64_URL_PAD: u8 = 0x3d;

#[inline(always)]
pub fn b64_url_encode(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(length * 4 / 3);
    let mut index = 0;
    if length >= 3 {
        while index <= (length - 3) {
            let value = (bytes[index] as u32) << 16
                | (bytes[index + 1] as u32) << 8
                | (bytes[index + 2] as u32);
            vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
            vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
            vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
            vec.push(B64_URL_ENCODE[(value & 0b11_1111) as usize]);
            index += 3;
        }
    }
    match length - index {
        2 => {
            let value = (bytes[index] as u32) << 16 | (bytes[index + 1] as u32) << 8;
            vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
            vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
            vec.push(B64_URL_ENCODE[((value >> 6) & 0b11_1111) as usize]);
            vec.push(B64_URL_PAD);
        }
        1 => {
            let value = (bytes[index] as u32) << 16;
            vec.push(B64_URL_ENCODE[((value >> 18) & 0b11_1111) as usize]);
            vec.push(B64_URL_ENCODE[((value >> 12) & 0b11_1111) as usize]);
            vec.push(B64_URL_PAD);
            vec.push(B64_URL_PAD);
        }
        _ => {}
    };
    vec
}

#[cfg(test)]
mod b64_url_encode_tests {
    use super::*;

    #[test]
    fn empty() {
        let result_bytes = b64_url_encode(b"");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn f() {
        let result_bytes = b64_url_encode(b"f");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "Zg==");
    }

    #[test]
    fn fo() {
        let result_bytes = b64_url_encode(b"fo");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "Zm8=");
    }

    #[test]
    fn foo() {
        let result_bytes = b64_url_encode(b"foo");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "Zm9v");
    }

    #[test]
    fn foob() {
        let result_bytes = b64_url_encode(b"foob");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "Zm9vYg==");
    }

    #[test]
    fn fooba() {
        let result_bytes = b64_url_encode(b"fooba");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "Zm9vYmE=");
    }

    #[test]
    fn foobar() {
        let result_bytes = b64_url_encode(b"foobar");
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "Zm9vYmFy");
    }
}

/// # Safety
///
/// This function should not be called without checking the input value.
#[inline(always)]
pub unsafe fn b64_url_decode_without_validation(bytes: &[u8]) -> Vec<u8> {
    let length = bytes.len();
    let mut vec = Vec::<u8>::with_capacity(length * 3 / 4);
    let mut index = 0;
    if length > 4 {
        while index < length - 4 {
            let value = ((B64_URL_DECODE[(bytes[index] as usize)] as u32) << 18)
                | ((B64_URL_DECODE[(bytes[index + 1] as usize)] as u32) << 12)
                | ((B64_URL_DECODE[(bytes[index + 2] as usize)] as u32) << 6)
                | (B64_URL_DECODE[(bytes[index + 3] as usize)] as u32);
            vec.push(((value >> 16) & 0b1111_1111) as u8);
            vec.push(((value >> 8) & 0b1111_1111) as u8);
            vec.push((value & 0b1111_1111) as u8);
            index += 4;
        }
    }
    if index + 4 == length {
        let mut value = ((B64_URL_DECODE[(bytes[index] as usize)] as u32) << 18)
            | ((B64_URL_DECODE[(bytes[index + 1] as usize)] as u32) << 12);
        if bytes[index + 2] != B64_URL_PAD {
            value |= (B64_URL_DECODE[(bytes[index + 2] as usize)] as u32) << 6;
            if bytes[index + 3] != B64_URL_PAD {
                value |= B64_URL_DECODE[(bytes[index + 3] as usize)] as u32;
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

#[cfg(test)]
mod b64_url_decode_without_validation_tests {
    use super::*;

    #[test]
    fn empty() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn f() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"Zg==") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "f");
    }

    #[test]
    fn fo() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"Zm8=") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "fo");
    }

    #[test]
    fn foo() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"Zm9v") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "foo");
    }

    #[test]
    fn foob() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"Zm9vYg==") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "foob");
    }

    #[test]
    fn fooba() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"Zm9vYmE=") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "fooba");
    }

    #[test]
    fn foobar() {
        let result_bytes = unsafe { b64_url_decode_without_validation(b"Zm9vYmFy") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "foobar");
    }
}
