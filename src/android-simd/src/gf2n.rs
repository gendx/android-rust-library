use std::arch::is_aarch64_feature_detected;

pub fn gf256_shamir_split_10(secret: &[u8; 32], output: &mut [u8; 640]) {
    let secret_ptr = secret as *const [u8; 32];
    let output_ptr = output.as_mut_ptr();

    if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
        unsafe { gf256_shamir_split_10_simd(secret_ptr, output_ptr) }
    } else {
        unsafe { gf256_shamir_split_10_fallback(secret_ptr, output_ptr) }
    };
}

pub fn gf64_invert(data: &mut [u64; 1]) {
    let data_ptr: *mut u64 = data.as_mut_ptr();

    if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
        unsafe { gf64_invert_simd(data_ptr) }
    } else {
        unsafe { gf64_invert_fallback(data_ptr) }
    };
}

pub fn gf128_invert(data: &mut [u64; 2]) {
    let data_ptr: *mut u64 = data.as_mut_ptr();

    if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
        unsafe { gf128_invert_simd(data_ptr) }
    } else {
        unsafe { gf128_invert_fallback(data_ptr) }
    };
}

pub fn gf256_invert(data: &mut [u64; 4]) {
    let data_ptr: *mut u64 = data.as_mut_ptr();

    if is_aarch64_feature_detected!("neon") && is_aarch64_feature_detected!("aes") {
        unsafe { gf256_invert_simd(data_ptr) }
    } else {
        unsafe { gf256_invert_fallback(data_ptr) }
    };
}

#[link(name = "fallback")]
extern "C" {
    fn gf256_shamir_split_10_fallback(secret: *const [u8; 32], output: *mut u8);
    fn gf64_invert_fallback(bytes: *mut u64);
    fn gf128_invert_fallback(bytes: *mut u64);
    fn gf256_invert_fallback(bytes: *mut u64);
}

#[link(name = "simd")]
extern "C" {
    fn gf256_shamir_split_10_simd(secret: *const [u8; 32], output: *mut u8);
    fn gf64_invert_simd(bytes: *mut u64);
    fn gf128_invert_simd(bytes: *mut u64);
    fn gf256_invert_simd(bytes: *mut u64);
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use std::hint::black_box;
    use test::Bencher;

    #[bench]
    fn bench_gf256_shamir_split_10(b: &mut Bencher) {
        let data = [1; 32];
        let mut output = [0; 640];
        b.iter(|| {
            gf256_shamir_split_10(black_box(&data), &mut output);
            output
        });
    }

    #[bench]
    fn bench_gf64_invert(b: &mut Bencher) {
        let mut data = [1];
        b.iter(|| gf64_invert(black_box(&mut data)));
    }

    #[bench]
    fn bench_gf128_invert(b: &mut Bencher) {
        let mut data = [1, 2];
        b.iter(|| gf128_invert(black_box(&mut data)));
    }

    #[bench]
    fn bench_gf256_invert(b: &mut Bencher) {
        let mut data = [1, 2, 3, 4];
        b.iter(|| gf256_invert(black_box(&mut data)));
    }
}
