#![allow(non_camel_case_types)]
extern crate alloc;

use core::ffi::c_uint;
use blst::{blst_fr, blst_p1, blst_p2, byte};
use sha2::{Digest, Sha256};

use crate::utils::bit_reversal_permutation;

//////////////////////// Constant Values for EIP-4844 ///////////////////////
pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_BLOB: usize = BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB;
pub const BYTES_PER_G1: usize = 48;
pub const BYTES_PER_G2: usize = 96;
pub const BYTES_PER_PROOF: usize = 48;
pub const BYTES_PER_COMMITMENT: usize = 48;

pub const TRUSTED_SETUP_PATH: &str = "src/trusted_setup.txt";
pub static mut TRUSTED_SETUP_NUM_G1_POINTS: usize = 0;
pub const TRUSTED_SETUP_NUM_G2_POINTS: usize = 65;

pub const FIAT_SHAMIR_PROTOCOL_DOMAIN: [u8; 16] = [70, 83, 66, 76, 79, 66, 86, 69, 82, 73, 70, 89, 95, 86, 49, 95]; // "FSBLOBVERIFY_V1_"
pub const CHALLENGE_INPUT_SIZE: usize = FIAT_SHAMIR_PROTOCOL_DOMAIN.len() + 16 + BYTES_PER_BLOB + BYTES_PER_COMMITMENT;
pub const RANDOM_CHALLENGE_KZG_BATCH_DOMAIN: [u8; 16] = [82, 67, 75, 90, 71, 66, 65, 84, 67, 72, 95, 95, 95, 86, 49, 95]; // "RCKZGBATCH___V1_"

//////////// C API for EIP-4844 //////////////

pub type C_KZG_RET = c_uint;

pub const C_KZG_RET_OK: C_KZG_RET = 0;
pub const C_KZG_RET_BADARGS: C_KZG_RET = 1;
pub const C_KZG_RET_ERROR: C_KZG_RET = 2;
pub const C_KZG_RET_MALLOC: C_KZG_RET = 3;

#[repr(C)]
pub struct Bytes32 {
    pub bytes: [u8; 32]
}

#[repr(C)]
pub struct Bytes48 {
    pub bytes: [u8; 48]
}

#[repr(C)]
pub struct BLSFieldELement {
    pub bytes: [u8; BYTES_PER_FIELD_ELEMENT]
}

#[repr(C)]
pub struct Blob {
    pub bytes: [u8; BYTES_PER_BLOB]
}

#[repr(C)]
pub struct KZGCommitment {
    pub bytes: [u8; BYTES_PER_COMMITMENT]
}

pub struct KZGProof {
    pub bytes: [u8; BYTES_PER_PROOF]
}

pub struct CKZSettings {
    pub max_width: u64,
    pub roots_of_unit: *mut blst_fr,
    pub g1_values: *mut blst_p1,
    pub g2_values: *mut blst_p2,
}


pub fn load_trusted_setup_string(contents: &str) -> Result<(Vec<u8>, Vec<u8>), String> {
    let mut offset = 0;

    const TRUSTED_SETUP_ERROR: &str = "Incorrect trusted setup format";

    #[inline(always)]
    fn scan_number(offset: &mut usize, contents: &str) -> Result<usize, String> {
        *offset += contents[(*offset)..].find(|c: char| !c.is_whitespace()).ok_or_else(|| String::from(TRUSTED_SETUP_ERROR))?;
        let start = *offset;
        *offset += contents[(*offset)..].find(|c: char| !c.is_ascii_digit()).ok_or_else(|| String::from(TRUSTED_SETUP_ERROR))?;
        let end = *offset;
        contents[start..end].parse::<usize>().map_err(|_| String::from(TRUSTED_SETUP_ERROR))
    }

    let g1_point_count = scan_number(& mut offset, contents)?;

    // FixME: must be TRUSTED_SETUP_NUM_G1_POINTS
    if g1_point_count != TRUSTED_SETUP_NUM_G1_POINTS {
        return Err(String::from(TRUSTED_SETUP_ERROR));
    }

    let g2_point_count = scan_number(&mut offset, contents)?;

    if g2_point_count != TRUSTED_SETUP_NUM_G2_POINTS {
        return Err(String::from(TRUSTED_SETUP_ERROR));
    }

    let mut g1_bytes = vec![0u8; g1_point_count * BYTES_PER_G1];
    let mut g2_bytes = vec![0u8; g1_point_count * BYTES_PER_G2];

    #[inline(always)]
    fn scan_hex_byte(offset: &mut usize, contents: &str) -> Result<u8, String> {
        *offset += contents[(*offset)..].find(|c: char| !c.is_whitespace()).ok_or_else(|| String::from(TRUSTED_SETUP_ERROR))?;
        let start = *offset;
        let end = if contents.get((*offset + 1)..).map(|it| {
            it.chars().next().map(|c| c.is_ascii_hexdigit()).unwrap_or(false)
        }).unwrap_or(false)
        {
            *offset += 2;
            *offset
        } else {
            *offset += 1;
            *offset
        };
        u8::from_str_radix(&contents[start..end], 16).map_err(|_| String::from(TRUSTED_SETUP_ERROR))
    }

    for byte in &mut g1_bytes {
        *byte = scan_hex_byte(&mut offset, contents)?
    }

    for byte in &mut g2_bytes {
        *byte = scan_hex_byte(&mut offset, contents)?
    }

    Ok((g1_bytes, g2_bytes))
}

pub fn bytes_of_uint64(out: &mut [u8], mut n: u64) {
    for byte in out.iter_mut().rev().take(8) {
        *byte = (n & 0xff) as u8;
        n >>= 8;
    }
}

pub fn hash(x: &[u8]) -> [u8; 32] {
    Sha256::digest(x).into()
}