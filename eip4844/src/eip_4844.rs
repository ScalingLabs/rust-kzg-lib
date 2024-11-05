
//////////////////////// Constant Values for EIP-4844 ///////////////////////
pub const FIELD_ELEMENTS_PER_BLOB: usize = 4096;
pub const BYTES_PER_FIELD_ELEMENT: usize = 32;
pub const BYTES_PER_BLOB: usize = BYTES_PER_FIELD_ELEMENT * FIELD_ELEMENTS_PER_BLOB;
pub const BYTES_PER_G1: usize = 96;
pub const BYTES_PER_PROOF: usize = 48;
pub const BYTES_PER_COMMITMENT: usize = 48;

pub const TRUSTED_SETUP_PATH: &str = "src/trusted_setup.txt";
pub static mut TRUSTED_SETUP_NUM_G1_POINTS: usize = 0;
pub const TRUSTED_SETUP_NUM_G2_POINTS: usize = 65;

pub const FIAT_SHAMIR_PROTOCOL_DOMAIN: [u8; 16] = [70, 83, 66, 76, 79, 66, 86, 69, 82, 73, 70, 89, 95, 86, 49, 95]; // "FSBLOBVERIFY_V1_"
pub const CHALLENGE_INPUT_SIZE: usize = FIAT_SHAMIR_PROTOCOL_DOMAIN.len() + 16 + BYTES_PER_BLOB + BYTES_PER_COMMITMENT;
pub const RANDOM_CHALLENGE_KZG_BATCH_DOMAIN: [u8; 16] = [82, 67, 75, 90, 71, 66, 65, 84, 67, 72, 95, 95, 95, 86, 49, 95]; // "RCKZGBATCH___V1_"