use std::arch::is_aarch64_feature_detected;

pub fn aesenc(block: &mut [u8; 16], key: &[u8; 16]) -> &'static str {
    let block_ptr: *mut u8 = block.as_mut_ptr();
    let key_ptr: *const u8 = key.as_ptr();

    let status = if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
        unsafe { aesenc_simd(block_ptr, key_ptr) }
    } else {
        unsafe { aesenc_fallback(block_ptr, key_ptr) }
    };
    match status {
        1 => "fallback",
        2 => "simd",
        _ => "unknown",
    }
}

#[link(name = "fallback")]
extern "C" {
    fn aesenc_fallback(block: *mut u8, key: *const u8) -> u32;
}

#[link(name = "simd")]
extern "C" {
    fn aesenc_simd(block: *mut u8, key: *const u8) -> u32;
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    fn aesenc_fallback_wrapper(block: &mut [u8; 16], key: &[u8; 16]) {
        let block_ptr: *mut u8 = block.as_mut_ptr();
        let key_ptr: *const u8 = key.as_ptr();
        unsafe { aesenc_fallback(block_ptr, key_ptr) };
    }

    fn aesenc_simd_wrapper(block: &mut [u8; 16], key: &[u8; 16]) {
        let block_ptr: *mut u8 = block.as_mut_ptr();
        let key_ptr: *const u8 = key.as_ptr();
        unsafe { aesenc_simd(block_ptr, key_ptr) };
    }

    #[target_feature(enable = "neon", enable = "aes")]
    unsafe fn aesenc_direct(block: &mut [u8; 16], key: &[u8; 16]) {
        use std::arch::aarch64::{uint8x16_t, vaeseq_u8, vaesmcq_u8, vdupq_n_u8, veorq_u8};
        use std::mem::transmute;

        let mut simd_block: uint8x16_t = transmute(*block);
        let simd_key: uint8x16_t = transmute(*key);

        let zero = vdupq_n_u8(0);
        let x = vaeseq_u8(simd_block, zero);
        let y = vaesmcq_u8(x);
        simd_block = veorq_u8(y, simd_key);

        *block = transmute(simd_block);
    }

    #[test]
    fn test_aesenc() {
        let mut block = [1; 16];
        let key = [2; 16];
        aesenc(&mut block, &key);
        assert_eq!(block, [0x7e; 16]);
    }

    #[test]
    fn test_fallback() {
        let mut block = [1; 16];
        let key = [2; 16];
        aesenc_fallback_wrapper(&mut block, &key);
        assert_eq!(block, [0x7e; 16]);
    }

    #[test]
    fn test_aesenc_simd() {
        let mut block = [1; 16];
        let key = [2; 16];
        aesenc_simd_wrapper(&mut block, &key);
        assert_eq!(block, [0x7e; 16]);
    }

    #[test]
    fn test_aesenc_direct() {
        let mut block = [1; 16];
        let key = [2; 16];
        unsafe { aesenc_direct(&mut block, &key) };
        assert_eq!(block, [0x7e; 16]);
    }

    #[bench]
    fn bench_aesenc(b: &mut Bencher) {
        let mut block = [1; 16];
        let key = [2; 16];
        b.iter(|| aesenc(black_box(&mut block), black_box(&key)));
    }

    #[bench]
    fn bench_aesenc_simd(b: &mut Bencher) {
        let mut block = [1; 16];
        let key = [2; 16];
        b.iter(|| aesenc_simd_wrapper(black_box(&mut block), black_box(&key)));
    }

    #[bench]
    fn bench_aesenc_fallback(b: &mut Bencher) {
        let mut block = [1; 16];
        let key = [2; 16];
        b.iter(|| aesenc_fallback_wrapper(black_box(&mut block), black_box(&key)));
    }

    #[bench]
    fn bench_aesenc_direct(b: &mut Bencher) {
        let mut block = [1; 16];
        let key = [2; 16];
        b.iter(|| unsafe { aesenc_direct(black_box(&mut block), black_box(&key)) });
    }
}
