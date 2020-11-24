//! ZkSnarks Supports
//!
//! Provides type definitions and traits for the built-in cryptographic zksnarks

mod private {
    /// Seals the implementation of `CryptoHash` and `HashOutput`.
    pub trait Sealed {}
}

/// The output type of a built-in cryptographic hash function.
pub trait CurvePointOutput: private::Sealed {
    /// The output type of the crypto hash.
    ///
    /// This should be a byte array with some constant size such as `[u8; 64]`.
    type Type: AsRef<[u8]>;

    /// The default value of the target type
    fn default() -> Self::Type;

    /// The bytes length of G1 point
    fn len() -> usize;
}

/// Types that are usable as built-in cryptographic hashes.
pub trait CurvePoint: CurvePointOutput + private::Sealed {
    /// Compute the curve add of the given raw byte input and copies the result into `output`.
    fn inflect_add(input: &[u8], output: &mut <Self as CurvePointOutput>::Type);
    /// Compute the curve mul of the given raw byte input and copies the result into `output`.
    fn inflect_mul(input: &[u8], output: &mut <Self as CurvePointOutput>::Type);
    /// Compute the curve pairing of the given raw byte input and copies the result into `output`.
    fn inflect_pairing(input: &[u8], output: &mut [u8; 1]);
}

/// The AltBn128 with 512-bit output.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum AltBn128 {}

impl private::Sealed for AltBn128 {}

impl CurvePointOutput for AltBn128 {
    type Type = [u8; 64];

    fn default() -> Self::Type {
        [0; 64]
    }

    fn len() -> usize {
        64
    }
}

/// The Bls12381 with 768-bit output.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Bls12381 {}

impl private::Sealed for Bls12381 {}

impl CurvePointOutput for Bls12381 {
    type Type = [u8; 96];

    fn default() -> Self::Type {
        [0; 96]
    }

    fn len() -> usize {
        96
    }
}
