//! Generic implementation of a finite field GF(2^n).
//!
//! This is based on the existence of a irreducible polynomial of the form
//! `x^n + x^a + x^b + x^c + 1`, where `0 < c < b < a < n`.

use std::convert::TryInto;
use std::fmt::{Debug, Display};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, BitAnd, BitXor, BitXorAssign, Mul, MulAssign, Not, Shl, Shr, Sub};

/// Trait for words that can be used for the representation of elements of GF(2^n).
pub trait Word:
    Copy
    + Eq
    + Hash
    + Debug
    + From<u8>
    + BitAnd<Output = Self>
    + BitXorAssign
    + BitXor<Output = Self>
    + Not<Output = Self>
    + Shl<usize, Output = Self>
    + Shr<usize, Output = Self>
{
    /// Zero.
    const ZERO: Self;
    /// One.
    const ONE: Self;
    /// Number of bytes in the size of the type.
    const NBYTES: usize = std::mem::size_of::<Self>();
    /// Number of bits in the size of the type.
    const NBITS: usize = 8 * Self::NBYTES;

    /// Parses a word from a byte slice. Panics if the slice length is not `NBYTES`.
    fn from_bytes(bytes: &[u8]) -> Self;
}

impl Word for u64 {
    const ZERO: Self = 0;
    const ONE: Self = 1;

    fn from_bytes(bytes: &[u8]) -> Self {
        let array = bytes.try_into().unwrap();
        u64::from_be_bytes(array)
    }
}

/// Implementation of a binary field GF(2^n), with `W::NBYTES * NWORDS` bits, using the
/// irreducible polynomial `x^n + x^a + x^b + x^c + 1`.
#[derive(Clone, Copy)]
pub struct GF2n<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> {
    words: [W; NWORDS],
}

/// Finite field GF(2^64) implemented with 64-bit words and using the following irreducible
/// polynomial: `x^64 + x^4 + x^3 + x + 1`.
pub type GF64 = GF2n<u64, 1, 4, 3, 1>;
/// Finite field GF(2^128) implemented with 64-bit words and using the following irreducible
/// polynomial: `x^128 + x^7 + x^2 + x + 1`.
pub type GF128 = GF2n<u64, 2, 7, 2, 1>;
/// Finite field GF(2^256) implemented with 64-bit words and using the following irreducible
/// polynomial: `x^256 + x^10 + x^5 + x^2 + 1`.
pub type GF256 = GF2n<u64, 4, 10, 5, 2>;

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Debug
    for GF2n<W, NWORDS, A, B, C>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match W::NBITS {
            8 => f.write_fmt(format_args!("{:02x?}", &self.words as &[W])),
            16 => f.write_fmt(format_args!("{:04x?}", &self.words as &[W])),
            32 => f.write_fmt(format_args!("{:08x?}", &self.words as &[W])),
            64 => f.write_fmt(format_args!("{:016x?}", &self.words as &[W])),
            128 => f.write_fmt(format_args!("{:032x?}", &self.words as &[W])),
            _ => f.write_fmt(format_args!("{:x?}", &self.words as &[W])),
        }
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Display
    for GF2n<W, NWORDS, A, B, C>
{
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for d in &self.words as &[W] {
            match W::NBITS {
                8 => f.write_fmt(format_args!("{:02x?}", d))?,
                16 => f.write_fmt(format_args!("{:04x?}", d))?,
                32 => f.write_fmt(format_args!("{:08x?}", d))?,
                64 => f.write_fmt(format_args!("{:016x?}", d))?,
                128 => f.write_fmt(format_args!("{:032x?}", d))?,
                _ => unimplemented!(),
            }
        }
        Ok(())
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> PartialEq
    for GF2n<W, NWORDS, A, B, C>
{
    fn eq(&self, other: &Self) -> bool {
        &self.words as &[W] == &other.words as &[W]
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Eq
    for GF2n<W, NWORDS, A, B, C>
{
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Hash
    for GF2n<W, NWORDS, A, B, C>
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&self.words as &[W]).hash(state)
    }
}

#[cfg(all(
    feature = "clmul",
    target_arch = "x86_64",
    target_feature = "sse2",
    target_feature = "pclmulqdq"
))]
fn mul_clmul_u64<const NWORDS: usize, const A: usize, const B: usize, const C: usize>(
    x: &GF2n<u64, NWORDS, A, B, C>,
    y: &GF2n<u64, NWORDS, A, B, C>,
) -> GF2n<u64, NWORDS, A, B, C> {
    use core::arch::x86_64::{__m128i, _mm_clmulepi64_si128, _mm_set_epi64x, _mm_storeu_si128};

    // Note: we cannot create an array of `NWORDS * 2` elements:
    // error: constant expression depends on a generic parameter
    let mut words = [0u64; NWORDS];
    let mut carry = [0u64; NWORDS];

    for i in 0..NWORDS {
        // Safety: target_feature "sse2" is available in this function.
        let xi: __m128i = unsafe { _mm_set_epi64x(0, x.words[i] as i64) };
        for j in 0..NWORDS {
            // Safety: target_feature "sse2" is available in this function.
            let yj: __m128i = unsafe { _mm_set_epi64x(0, y.words[j] as i64) };
            // Safety: target_feature "pclmulqdq" is available in this function.
            let clmul: __m128i = unsafe { _mm_clmulepi64_si128(xi, yj, 0) };
            let mut cc: [u64; 2] = [0u64, 0u64];
            // Safety:
            // - target_feature "sse2" is available in this function,
            // - cc points to 128 bits (no alignment required by this function).
            unsafe { _mm_storeu_si128(&mut cc as *mut _ as *mut __m128i, clmul) };

            let ij = i + j;
            if ij < NWORDS {
                words[ij] ^= cc[0];
            } else {
                carry[ij - NWORDS] ^= cc[0];
            }

            let ij1 = ij + 1;
            if ij1 < NWORDS {
                words[ij1] ^= cc[1];
            } else {
                carry[ij1 - NWORDS] ^= cc[1];
            }
        }
    }

    GF2n::<u64, NWORDS, A, B, C>::propagate_carries(words, carry)
}

#[cfg(all(feature = "clmul", target_arch = "aarch64", target_feature = "aes"))]
unsafe fn mul_clmul_u64<const NWORDS: usize, const A: usize, const B: usize, const C: usize>(
    x: &GF2n<u64, NWORDS, A, B, C>,
    y: &GF2n<u64, NWORDS, A, B, C>,
) -> GF2n<u64, NWORDS, A, B, C> {
    use std::arch::aarch64::vmull_p64;

    // Note: we cannot create an array of `NWORDS * 2` elements:
    // error: constant expression depends on a generic parameter
    let mut words = [0u64; NWORDS];
    let mut carry = [0u64; NWORDS];

    for i in 0..NWORDS {
        let xi = x.words[i];
        for j in 0..NWORDS {
            let yj = y.words[j];
            // Safety: target_feature's "neon" and "aes" are available in this function.
            let clmul: u128 = vmull_p64(xi, yj);
            let low: u64 = clmul as u64;
            let high: u64 = (clmul >> 64) as u64;

            let ij = i + j;
            if ij < NWORDS {
                words[ij] ^= low;
            } else {
                carry[ij - NWORDS] ^= low;
            }

            let ij1 = ij + 1;
            if ij1 < NWORDS {
                words[ij1] ^= high;
            } else {
                carry[ij1 - NWORDS] ^= high;
            }
        }
    }

    GF2n::<u64, NWORDS, A, B, C>::propagate_carries(words, carry)
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize>
    GF2n<W, NWORDS, A, B, C>
{
    const NBITS: usize = W::NBITS * NWORDS;

    const fn new_small(word: W) -> Self {
        let mut words = [W::ZERO; NWORDS];
        words[0] = word;
        Self { words }
    }

    fn shl1(&mut self) {
        let mut carry = W::ZERO;
        for i in 0..NWORDS {
            let d = self.words[i];
            self.words[i] = (d << 1) ^ carry;
            carry = d >> (W::NBITS - 1);
        }
        if carry != W::ZERO {
            self.words[0] ^= W::ONE ^ (W::ONE << A) ^ (W::ONE << B) ^ (W::ONE << C);
        }
    }

    fn mul_as_add(mut self, other: &Self) -> Self {
        let mut result = Self {
            words: [W::ZERO; NWORDS],
        };
        for &word in &other.words as &[W] {
            for i in 0..W::NBITS {
                if word & (W::ONE << i) != W::ZERO {
                    result += &self;
                }
                self.shl1();
            }
        }
        result
    }

    #[cfg(any(
        all(
            feature = "clmul",
            target_arch = "x86_64",
            target_feature = "sse2",
            target_feature = "pclmulqdq"
        ),
        all(
            feature = "clmul",
            target_arch = "aarch64",
            target_feature = "neon",
            target_feature = "aes"
        )
    ))]
    fn propagate_carries(mut words: [W; NWORDS], carry: [W; NWORDS]) -> Self {
        if NWORDS == 1 {
            let mut c = carry[0];
            while c != W::ZERO {
                words[0] ^= c ^ (c << A) ^ (c << B) ^ (c << C);
                c = (c >> (W::NBITS - A)) ^ (c >> (W::NBITS - B)) ^ (c >> (W::NBITS - C));
            }
        } else {
            for i in 0..NWORDS {
                let c = carry[i];
                words[i] ^= c ^ (c << A) ^ (c << B) ^ (c << C);
                if i + 1 < NWORDS {
                    words[i + 1] ^=
                        (c >> (W::NBITS - A)) ^ (c >> (W::NBITS - B)) ^ (c >> (W::NBITS - C));
                } else {
                    let c = (c >> (W::NBITS - A)) ^ (c >> (W::NBITS - B)) ^ (c >> (W::NBITS - C));
                    words[0] ^= c ^ (c << A) ^ (c << B) ^ (c << C);
                    words[1] ^=
                        (c >> (W::NBITS - A)) ^ (c >> (W::NBITS - B)) ^ (c >> (W::NBITS - C));
                }
            }
        }

        Self { words }
    }

    const ONE: Self = Self::new_small(W::ONE);

    pub fn invert(mut self) -> Self {
        // Compute x^(2^n - 2)
        let mut result = Self::ONE;
        for _ in 1..Self::NBITS {
            self = self * &self;
            result *= &self;
        }
        result
    }

    pub fn from_words(words: [W; NWORDS]) -> Self {
        Self { words }
    }

    pub fn to_words(self) -> [W; NWORDS] {
        self.words
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> From<u8>
    for GF2n<W, NWORDS, A, B, C>
{
    fn from(word: u8) -> Self {
        let mut words = [W::ZERO; NWORDS];
        words[0] = W::from(word);
        Self { words }
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Add
    for GF2n<W, NWORDS, A, B, C>
{
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    #[allow(clippy::needless_range_loop)]
    fn add(self, other: Self) -> Self {
        let mut words = [W::ZERO; NWORDS];
        for i in 0..NWORDS {
            words[i] = self.words[i] ^ other.words[i];
        }
        Self { words }
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> AddAssign<&Self>
    for GF2n<W, NWORDS, A, B, C>
{
    #[allow(clippy::suspicious_op_assign_impl)]
    fn add_assign(&mut self, other: &Self) {
        for i in 0..NWORDS {
            self.words[i] ^= other.words[i];
        }
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Sub
    for GF2n<W, NWORDS, A, B, C>
{
    type Output = Self;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn sub(self, other: Self) -> Self {
        self + other
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> Mul<&Self>
    for GF2n<W, NWORDS, A, B, C>
{
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        #[cfg(all(
            feature = "clmul",
            target_arch = "x86_64",
            target_feature = "sse2",
            target_feature = "pclmulqdq"
        ))]
        if W::NBITS == 64 {
            // Safety: W == u64 when NBITS == 64.
            let x: &GF2n<u64, NWORDS, A, B, C> = unsafe { std::mem::transmute(&self) };
            // Safety: W == u64 when NBITS == 64.
            let y: &GF2n<u64, NWORDS, A, B, C> = unsafe { std::mem::transmute(other) };
            let tmp: GF2n<u64, NWORDS, A, B, C> = mul_clmul_u64(x, y);
            // Safety: W == u64 when NBITS == 64.
            let result: &Self = unsafe { std::mem::transmute(&tmp) };
            return *result;
        }
        #[cfg(all(
            feature = "clmul",
            target_arch = "aarch64",
            target_feature = "neon",
            target_feature = "aes"
        ))]
        if W::NBITS == 64 {
            // Safety: W == u64 when NBITS == 64.
            let x: &GF2n<u64, NWORDS, A, B, C> = unsafe { std::mem::transmute(&self) };
            // Safety: W == u64 when NBITS == 64.
            let y: &GF2n<u64, NWORDS, A, B, C> = unsafe { std::mem::transmute(other) };
            let tmp: GF2n<u64, NWORDS, A, B, C> = unsafe { mul_clmul_u64(x, y) };
            // Safety: W == u64 when NBITS == 64.
            let result: &Self = unsafe { std::mem::transmute(&tmp) };
            return *result;
        }
        self.mul_as_add(other)
    }
}

impl<W: Word, const NWORDS: usize, const A: usize, const B: usize, const C: usize> MulAssign<&Self>
    for GF2n<W, NWORDS, A, B, C>
{
    fn mul_assign(&mut self, other: &Self) {
        *self = *self * other;
    }
}
