extern crate alloc;

use alloc::string::String;
use core::mem;


pub fn bit_reversal_permutation<T>(vals: &mut [T]) -> Result<(), String> 
where 
    T: Clone,
{
    if vals.is_empty() {
        return Err(String::from("Values cannot be empty"));
    }

    if vals.len() == 1 {
        return Ok(());
    }

    if !vals.len().is_power_of_two() {
        return Err(String::from("Values length has to be a power of 2"));
    }

    let unused_bit_len = vals.len().leading_zeros() + 1;
    for i in 0..vals.len() - 1 {
        let r = i.reverse_bits() >> unused_bit_len;
        if r > i {
            let tmp = vals[r].clone();
            vals[r] = vals[i].clone();
            vals[i] = tmp;
        }
    }
    Ok(())
}

pub fn is_power_of_two(n: usize) -> bool {
    n & (n - 1) == 0
}

pub fn log2_pow2(mut n: usize) -> usize {
    let mut position = 0;
    while n > 1 {
        n >>= 1;
        position += 1;
    }
    position
}

pub fn reverse_bits_limited(n: usize, value: usize) -> usize {
    let unused_bits_len = n.leading_zeros();
    value.reverse_bits() >> unused_bits_len
}



