use b64_url::*;

mod b64_url_encode_with_config_tests {
    use super::*;

    mod with_omit_pads {
        use super::*;

        #[test]
        fn empty() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "");
        }

        #[test]
        fn f() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"f", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "Zg");
        }

        #[test]
        fn fo() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"fo", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "Zm8");
        }

        #[test]
        fn foo() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"foo", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "Zm9v");
        }

        #[test]
        fn foob() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"foob", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "Zm9vYg");
        }

        #[test]
        fn fooba() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"fooba", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "Zm9vYmE");
        }

        #[test]
        fn foobar() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = b64_url_encode_with_config(b"foobar", &config);
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "Zm9vYmFy");
        }
    }
}

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

mod unsafe_b64_url_decode_tests {
    use super::*;

    #[test]
    fn empty() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "");
    }

    #[test]
    fn f() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"Zg==") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "f");
    }

    #[test]
    fn fo() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"Zm8=") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "fo");
    }

    #[test]
    fn foo() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"Zm9v") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "foo");
    }

    #[test]
    fn foob() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"Zm9vYg==") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "foob");
    }

    #[test]
    fn fooba() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"Zm9vYmE=") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "fooba");
    }

    #[test]
    fn foobar() {
        let result_bytes = unsafe { unsafe_b64_url_decode(b"Zm9vYmFy") };
        let result = String::from_utf8(result_bytes).unwrap();
        assert_eq!(result, "foobar");
    }
}

mod unsafe_b64_url_decode_with_config_tests {
    use super::*;

    mod with_omit_pads {
        use super::*;

        #[test]
        fn empty() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "");
        }

        #[test]
        fn f() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"Zg", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "f");
        }

        #[test]
        fn fo() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"Zm8", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "fo");
        }

        #[test]
        fn foo() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"Zm9v", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "foo");
        }

        #[test]
        fn foob() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"Zm9vYg", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "foob");
        }

        #[test]
        fn fooba() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"Zm9vYmE", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "fooba");
        }

        #[test]
        fn foobar() {
            let config = B64Config {
                padding: B64ConfigPadding { omit: true },
            };
            let result_bytes = unsafe { unsafe_b64_url_decode_with_config(b"Zm9vYmFy", &config) };
            let result = String::from_utf8(result_bytes).unwrap();
            assert_eq!(result, "foobar");
        }
    }
}
