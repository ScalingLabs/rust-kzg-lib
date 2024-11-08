///

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