pub fn pmul(a: u64, b: u64) -> (u128, &'static str) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse2") && is_x86_feature_detected!("pclmulqdq") {
            // Safety: target_features "sse2" and "pclmulqdq" are available in this block.
            return unsafe { pmul_x86_clmul(a, b) };
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::is_aarch64_feature_detected;
        if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
            // Safety: target_features "neon" and "aes" are available in this block.
            return unsafe { pmul_aarch64_neon(a, b) };
        }
    }
    pmul_nosimd(a, b)
}

pub fn pmul_cheat(a: u64, b: u64) -> (u128, &'static str) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("sse2") && is_x86_feature_detected!("pclmulqdq") {
            // Safety: target_features "sse2" and "pclmulqdq" are available in this block.
            return unsafe { pmul_x86_clmul(a, b) };
        }
    }
    #[cfg(target_arch = "aarch64")]
    {
        use std::arch::is_aarch64_feature_detected;
        // FIXME: Here we cheat and omit to detect the "aes" feature.
        if is_aarch64_feature_detected!("neon") {
            // Safety: target_features "neon" and "aes" are available in this block.
            return unsafe { pmul_aarch64_neon(a, b) };
        }
    }
    pmul_nosimd(a, b)
}

pub fn pmul_nosimd(a: u64, b: u64) -> (u128, &'static str) {
    let mut tmp: u128 = b as u128;
    let mut result: u128 = 0;
    for i in 0..64 {
        if a & (1 << i) != 0 {
            result ^= tmp;
        }
        tmp <<= 1;
    }
    (result, "nosimd")
}

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "sse2", enable = "pclmulqdq")]
unsafe fn pmul_x86_clmul(a: u64, b: u64) -> (u128, &'static str) {
    #[cfg(target_arch = "x86")]
    use std::arch::x86::{__m128i, _mm_clmulepi64_si128, _mm_set_epi64x, _mm_storeu_si128};
    #[cfg(target_arch = "x86_64")]
    use std::arch::x86_64::{__m128i, _mm_clmulepi64_si128, _mm_set_epi64x, _mm_storeu_si128};

    // Safety: target_feature "sse2" is available in this function.
    let x: __m128i = _mm_set_epi64x(0, a as i64);
    // Safety: target_feature "sse2" is available in this function.
    let y: __m128i = _mm_set_epi64x(0, b as i64);
    // Safety: target_feature "pclmulqdq" is available in this function.
    let clmul: __m128i = _mm_clmulepi64_si128(x, y, 0);

    let mut result: u128 = 0;
    // Safety:
    // - target_feature "sse2" is available in this function,
    // - result points to 128 bits (no alignment required by this function).
    _mm_storeu_si128(&mut result as *mut _ as *mut __m128i, clmul);

    (result, "x86_clmul")
}

#[cfg(target_arch = "aarch64")]
#[target_feature(enable = "neon", enable = "aes")]
unsafe fn pmul_aarch64_neon(a: u64, b: u64) -> (u128, &'static str) {
    use std::arch::aarch64::vmull_p64;

    // Safety: target_features "neon" and "aes" are available in this function.
    let result: u128 = vmull_p64(a, b);
    (result, "aarch64_neon")
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[test]
    fn test_pmul() {
        let (result, strategy) = pmul(0x1234567890abcdef, 0xfedcba0987654321);
        assert_eq!(result, 0x0e038d8eab3af47a1f31f87ebb8c810f);
        #[cfg(target_arch = "aarch64")]
        assert_eq!(strategy, "aarch64_neon");
    }

    #[test]
    fn test_pmul_cheat() {
        let (result, strategy) = pmul_cheat(0x1234567890abcdef, 0xfedcba0987654321);
        assert_eq!(result, 0x0e038d8eab3af47a1f31f87ebb8c810f);
        #[cfg(target_arch = "aarch64")]
        assert_eq!(strategy, "aarch64_neon");
    }

    #[test]
    fn test_pmul_nosimd() {
        let (result, strategy) = pmul_nosimd(0x1234567890abcdef, 0xfedcba0987654321);
        assert_eq!(result, 0x0e038d8eab3af47a1f31f87ebb8c810f);
        assert_eq!(strategy, "nosimd");
    }

    #[bench]
    fn bench_pmul(b: &mut Bencher) {
        b.iter(|| pmul(black_box(0x1234567890abcdef), black_box(0xfedcba0987654321)));
    }

    #[bench]
    fn bench_pmul_cheat(b: &mut Bencher) {
        b.iter(|| pmul_cheat(black_box(0x1234567890abcdef), black_box(0xfedcba0987654321)));
    }

    #[bench]
    fn bench_pmul_nosimd(b: &mut Bencher) {
        b.iter(|| pmul_nosimd(black_box(0x1234567890abcdef), black_box(0xfedcba0987654321)));
    }

    #[cfg(target_arch = "aarch64")]
    #[bench]
    fn bench_pmul_aarch64_neon(b: &mut Bencher) {
        b.iter(|| unsafe {
            pmul_aarch64_neon(black_box(0x1234567890abcdef), black_box(0xfedcba0987654321))
        });
    }
}
