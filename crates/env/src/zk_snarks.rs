//! ZkSnarks Supports
//!
//! Provides type definitions and traits for the built-in cryptographic zksnarks

mod private {
    /// Seals the implementation of `CryptoHash` and `HashOutput`.
    pub trait Sealed {}
}

/// Default supports any types
pub trait Default {
    /// Default value
    fn default() -> Self;
}

impl Default for [u8; 64] {
    fn default() -> Self {
        [0; 64]
    }
}

/// The output type of a built-in cryptographic hash function.
pub trait CurvePointOutput: private::Sealed {
    /// The output type of the crypto hash.
    ///
    /// This should be a byte array with some constant size such as `[u8; 64]`.
    type Type: Default;
}

/// Types that are usable as built-in cryptographic hashes.
pub trait CurvePoint: CurvePointOutput + private::Sealed {
    /// Hashes the given raw byte input and copies the result into `output`.
    fn inflect_add(g1: &[u8], g2: &[u8], output: &mut <Self as CurvePointOutput>::Type);
    /// Hashes the given raw byte input and copies the result into `output`.
    fn inflect_mul(
        input: &[u8],
        scalar: u64,
        output: &mut <Self as CurvePointOutput>::Type,
    );
}

/// The AltBn128Add with 512-bit output.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AltBn128 {}

// /// The AltBn128Pairing with 512-bit output.
// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
// pub enum AltBn128Pairing {}

impl private::Sealed for AltBn128 {}
// impl private::Sealed for AltBn128Pairing {}

impl CurvePointOutput for AltBn128 {
    type Type = [u8; 64];
}
