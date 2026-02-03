use std::env;

fn main() {
    println!("cargo:rustc-check-cfg=cfg(simd_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_sse2_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_ssse3_decode_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_sse2_encode_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_ssse3_encode_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_avx2_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_avx2_encode_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_avx512_env)");
    println!("cargo:rustc-check-cfg=cfg(simd_avx512_encode_env)");

    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_ENCODE_SSE2");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_ENCODE_SSSE3");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_ENCODE_AVX2");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_ENCODE_AVX512");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_DECODE_SSE2");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_DECODE_SSSE3");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_DECODE_AVX2");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_THRESHOLD_DECODE_AVX512");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_SSE2_DECODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_SSSE3_DECODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_SSE2_ENCODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_SSSE3_ENCODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_AVX2_DECODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_AVX2_ENCODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_AVX512_DECODE");
    println!("cargo:rerun-if-env-changed=B64_URL__SIMD_AVX512_ENCODE");

    if let Ok(val) = env::var("B64_URL__SIMD_THRESHOLD") {
        match val.as_str() {
            "32" | "64" | "128" | "256" => {
                println!("cargo:rustc-env=B64_URL__SIMD_THRESHOLD={}", val)
            }
            _ => println!("cargo:warning=Unsupported SIMD_THRESHOLD value: {}", val),
        }
    }

    for (key, name) in [
        (
            "B64_URL__SIMD_THRESHOLD_ENCODE_SSE2",
            "SIMD_THRESHOLD_ENCODE_SSE2",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_ENCODE_SSSE3",
            "SIMD_THRESHOLD_ENCODE_SSSE3",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_ENCODE_AVX2",
            "SIMD_THRESHOLD_ENCODE_AVX2",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_ENCODE_AVX512",
            "SIMD_THRESHOLD_ENCODE_AVX512",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_DECODE_SSE2",
            "SIMD_THRESHOLD_DECODE_SSE2",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_DECODE_SSSE3",
            "SIMD_THRESHOLD_DECODE_SSSE3",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_DECODE_AVX2",
            "SIMD_THRESHOLD_DECODE_AVX2",
        ),
        (
            "B64_URL__SIMD_THRESHOLD_DECODE_AVX512",
            "SIMD_THRESHOLD_DECODE_AVX512",
        ),
    ] {
        if let Ok(val) = env::var(key) {
            if val.parse::<usize>().is_ok() {
                println!("cargo:rustc-env={}={}", name, val);
                println!("cargo:rustc-env={}={}", key, val);
            } else {
                println!("cargo:warning=Unsupported {} value: {}", key, val);
            }
        }
    }

    let mut simd_env = false;

    if env::var("B64_URL__SIMD_SSE2_DECODE").is_ok() {
        println!("cargo:rustc-cfg=simd_sse2_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_SSSE3_DECODE").is_ok() {
        println!("cargo:rustc-cfg=simd_ssse3_decode_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_SSE2_ENCODE").is_ok() {
        println!("cargo:rustc-cfg=simd_sse2_encode_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_SSSE3_ENCODE").is_ok() {
        println!("cargo:rustc-cfg=simd_ssse3_encode_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_AVX2_DECODE").is_ok() {
        println!("cargo:rustc-cfg=simd_avx2_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_AVX2_ENCODE").is_ok() {
        println!("cargo:rustc-cfg=simd_avx2_encode_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_AVX512_DECODE").is_ok() {
        println!("cargo:rustc-cfg=simd_avx512_env");
        simd_env = true;
    }
    if env::var("B64_URL__SIMD_AVX512_ENCODE").is_ok() {
        println!("cargo:rustc-cfg=simd_avx512_encode_env");
        simd_env = true;
    }

    if simd_env {
        println!("cargo:rustc-cfg=simd_env");
    }
}
