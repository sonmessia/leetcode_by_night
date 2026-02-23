struct Solution;

impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let mut n = n as u32;
        let mut prev_bit = n & 1;
        n >>= 1;
        while n > 0 {
            let current_bit = n & 1;
            if current_bit == prev_bit {
                return false;
            }
            prev_bit = current_bit;
            n >>= 1;
        }
        true
    }
}
