#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod eip_4844;
pub mod utils;

pub trait Fr: Default + Clone + PartialEq + Sync {
    fn null() -> Self;

    fn zero() -> Self;

    fn one() -> Self;

    #[cfg(feature = "rand")]
    fn rand() -> Self;

    fn from_bytes(bytes: &[u8]) -> Result<Self, String>;

    fn from_bytes_unchecked(bytes: &[u8]) -> Result<Self, String> {
        Self::from_bytes(bytes)
    }

    fn from_hex(hex: &str) -> Result<Self, String>;

    fn from_u64_arr(u: &[u64; 4]) -> Self;

    fn from_u64(u: u64) -> Self;

    fn to_bytes(&self) -> [u8; 32];

    fn to_u64_arr(&self) -> [u64; 4];

    fn is_one(&self) -> bool;

    fn is_zero(&self) -> bool;

    fn is_null(&self) -> bool;

    fn sqr(&self) -> Self;

    fn mul(&self, b: &Self) -> Self;

    fn add(&self, b: &Self) -> Self;

    fn sub(&self, b: &Self) -> Self;

    fn eucl_inverse(&self) -> Self;

    fn negate(&self) -> Self;

    fn inverse(&self) -> Self;

    fn pow(&self, n: usize) -> Self;

    fn div(&self, b: &Self) -> Result<Self, String>;

    fn equals(&self, b: &Self) -> bool;

    fn eq(&self, other: &Self) -> bool {
        self.equals(other)
    }

    fn to_scalar(&self) -> Scalar256;

}