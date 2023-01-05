#![feature(stdsimd)]

mod gf2n;

use gf2n::{GF128, GF256, GF64};
use horcrux::field::Field;
use horcrux::shamir::{RandomShamir, Shamir};

#[cfg(not(all(target_feature = "neon", target_feature = "aes")))]
mod fallback {
    use super::*;

    #[no_mangle]
    pub unsafe extern "C" fn gf256_shamir_split_10_fallback(
        secret: *const [u8; 32],
        output: *mut u8,
    ) {
        let secret = horcrux::gf2n::GF256::from_bytes(&*secret).unwrap();
        let shares = RandomShamir::split(&secret, 10, 10);
        let output_slice = std::slice::from_raw_parts_mut(
            output as *mut <RandomShamir as Shamir<horcrux::gf2n::GF256>>::Share,
            10,
        );
        output_slice.copy_from_slice(shares.as_slice());
    }

    #[no_mangle]
    pub unsafe extern "C" fn gf64_invert_fallback(bytes: *mut u64) {
        let bytes: &mut [u64; 1] = &mut *(bytes as *mut [u64; 1]);

        let x = GF64::from_words(*bytes);
        *bytes = x.invert().to_words();
    }

    #[no_mangle]
    pub unsafe extern "C" fn gf128_invert_fallback(bytes: *mut u64) {
        let bytes: &mut [u64; 2] = &mut *(bytes as *mut [u64; 2]);

        let x = GF128::from_words(*bytes);
        *bytes = x.invert().to_words();
    }

    #[no_mangle]
    pub unsafe extern "C" fn gf256_invert_fallback(bytes: *mut u64) {
        let bytes: &mut [u64; 4] = &mut *(bytes as *mut [u64; 4]);

        let x = GF256::from_words(*bytes);
        *bytes = x.invert().to_words();
    }

    #[no_mangle]
    pub unsafe extern "C" fn aesenc_fallback(block: *mut u8, key: *const u8) -> u32 {
        let state: &mut [u8; 16] = &mut *(block as *mut [u8; 16]);
        let rkey: &[u8; 16] = &*(key as *const [u8; 16]);

        subbytes(state);
        shiftrows(state);
        mixcolumns(state);
        addroundkey(state, rkey);

        1
    }

    fn subbytes(state: &mut [u8; 16]) {
        for x in state.iter_mut() {
            *x = AES_SBOX[*x as usize];
        }
    }

    fn shiftrows(state: &mut [u8; 16]) {
        let tmp = state[1];
        state[1] = state[5];
        state[5] = state[9];
        state[9] = state[13];
        state[13] = tmp;

        let tmp = state[2];
        state[2] = state[10];
        state[10] = tmp;
        let tmp = state[6];
        state[6] = state[14];
        state[14] = tmp;

        let tmp = state[3];
        state[3] = state[15];
        state[15] = state[11];
        state[11] = state[7];
        state[7] = tmp;
    }

    // multiplication by 2 in GF(2^256)
    fn mul2(x: u8) -> u8 {
        (x << 1) ^ (((x >> 7) & 1) * 0x1b)
    }

    fn mixcolumns(state: &mut [u8; 16]) {
        for i in 0..4 {
            let x0 = state[4 * i];
            let x1 = state[4 * i + 1];
            let x2 = state[4 * i + 2];
            let x3 = state[4 * i + 3];
            let x = x0 ^ x1 ^ x2 ^ x3;
            state[4 * i] ^= mul2(x0 ^ x1) ^ x;
            state[4 * i + 1] ^= mul2(x1 ^ x2) ^ x;
            state[4 * i + 2] ^= mul2(x2 ^ x3) ^ x;
            state[4 * i + 3] ^= mul2(x3 ^ x0) ^ x;
        }
    }

    fn addroundkey(state: &mut [u8; 16], rkey: &[u8; 16]) {
        for i in 0..16 {
            state[i] ^= rkey[i];
        }
    }

    static AES_SBOX: [u8; 256] = [
        0x63, 0x7c, 0x77, 0x7b, 0xf2, 0x6b, 0x6f, 0xc5, 0x30, 0x01, 0x67, 0x2b, 0xfe, 0xd7, 0xab,
        0x76, 0xca, 0x82, 0xc9, 0x7d, 0xfa, 0x59, 0x47, 0xf0, 0xad, 0xd4, 0xa2, 0xaf, 0x9c, 0xa4,
        0x72, 0xc0, 0xb7, 0xfd, 0x93, 0x26, 0x36, 0x3f, 0xf7, 0xcc, 0x34, 0xa5, 0xe5, 0xf1, 0x71,
        0xd8, 0x31, 0x15, 0x04, 0xc7, 0x23, 0xc3, 0x18, 0x96, 0x05, 0x9a, 0x07, 0x12, 0x80, 0xe2,
        0xeb, 0x27, 0xb2, 0x75, 0x09, 0x83, 0x2c, 0x1a, 0x1b, 0x6e, 0x5a, 0xa0, 0x52, 0x3b, 0xd6,
        0xb3, 0x29, 0xe3, 0x2f, 0x84, 0x53, 0xd1, 0x00, 0xed, 0x20, 0xfc, 0xb1, 0x5b, 0x6a, 0xcb,
        0xbe, 0x39, 0x4a, 0x4c, 0x58, 0xcf, 0xd0, 0xef, 0xaa, 0xfb, 0x43, 0x4d, 0x33, 0x85, 0x45,
        0xf9, 0x02, 0x7f, 0x50, 0x3c, 0x9f, 0xa8, 0x51, 0xa3, 0x40, 0x8f, 0x92, 0x9d, 0x38, 0xf5,
        0xbc, 0xb6, 0xda, 0x21, 0x10, 0xff, 0xf3, 0xd2, 0xcd, 0x0c, 0x13, 0xec, 0x5f, 0x97, 0x44,
        0x17, 0xc4, 0xa7, 0x7e, 0x3d, 0x64, 0x5d, 0x19, 0x73, 0x60, 0x81, 0x4f, 0xdc, 0x22, 0x2a,
        0x90, 0x88, 0x46, 0xee, 0xb8, 0x14, 0xde, 0x5e, 0x0b, 0xdb, 0xe0, 0x32, 0x3a, 0x0a, 0x49,
        0x06, 0x24, 0x5c, 0xc2, 0xd3, 0xac, 0x62, 0x91, 0x95, 0xe4, 0x79, 0xe7, 0xc8, 0x37, 0x6d,
        0x8d, 0xd5, 0x4e, 0xa9, 0x6c, 0x56, 0xf4, 0xea, 0x65, 0x7a, 0xae, 0x08, 0xba, 0x78, 0x25,
        0x2e, 0x1c, 0xa6, 0xb4, 0xc6, 0xe8, 0xdd, 0x74, 0x1f, 0x4b, 0xbd, 0x8b, 0x8a, 0x70, 0x3e,
        0xb5, 0x66, 0x48, 0x03, 0xf6, 0x0e, 0x61, 0x35, 0x57, 0xb9, 0x86, 0xc1, 0x1d, 0x9e, 0xe1,
        0xf8, 0x98, 0x11, 0x69, 0xd9, 0x8e, 0x94, 0x9b, 0x1e, 0x87, 0xe9, 0xce, 0x55, 0x28, 0xdf,
        0x8c, 0xa1, 0x89, 0x0d, 0xbf, 0xe6, 0x42, 0x68, 0x41, 0x99, 0x2d, 0x0f, 0xb0, 0x54, 0xbb,
        0x16,
    ];
}

#[cfg(all(target_feature = "neon", target_feature = "aes"))]
mod simd {
    use super::*;
    use std::arch::aarch64::{uint8x16_t, vaeseq_u8, vaesmcq_u8, vdupq_n_u8, veorq_u8};

    #[no_mangle]
    pub unsafe extern "C" fn gf256_shamir_split_10_simd(secret: *const [u8; 32], output: *mut u8) {
        let secret = horcrux::gf2n::GF256::from_bytes(&*secret).unwrap();
        let shares = RandomShamir::split(&secret, 10, 10);
        let output_slice = std::slice::from_raw_parts_mut(
            output as *mut <RandomShamir as Shamir<horcrux::gf2n::GF256>>::Share,
            10,
        );
        output_slice.copy_from_slice(shares.as_slice());
    }

    #[no_mangle]
    pub unsafe extern "C" fn gf64_invert_simd(bytes: *mut u64) {
        let bytes: &mut [u64; 1] = &mut *(bytes as *mut [u64; 1]);

        let x = GF64::from_words(*bytes);
        *bytes = x.invert().to_words();
    }

    #[no_mangle]
    pub unsafe extern "C" fn gf128_invert_simd(bytes: *mut u64) {
        let bytes: &mut [u64; 2] = &mut *(bytes as *mut [u64; 2]);

        let x = GF128::from_words(*bytes);
        *bytes = x.invert().to_words();
    }

    #[no_mangle]
    pub unsafe extern "C" fn gf256_invert_simd(bytes: *mut u64) {
        let bytes: &mut [u64; 4] = &mut *(bytes as *mut [u64; 4]);

        let x = GF256::from_words(*bytes);
        *bytes = x.invert().to_words();
    }

    #[no_mangle]
    pub unsafe extern "C" fn aesenc_simd(block: *mut u8, key: *const u8) -> u32 {
        let simd_block: &mut uint8x16_t = &mut *(block as *mut uint8x16_t);
        let simd_key: &uint8x16_t = &*(key as *const uint8x16_t);

        let zero = vdupq_n_u8(0);
        let x = vaeseq_u8(*simd_block, zero);
        let y = vaesmcq_u8(x);
        *simd_block = veorq_u8(y, *simd_key);

        2
    }
}
