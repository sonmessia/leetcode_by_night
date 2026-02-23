struct Solution;

impl Solution {
    pub fn count_prime_set_bits(left: i32, right: i32) -> i32 {
        let prime_set_bits = [2, 3, 5, 7, 11, 13, 17, 19];
        let mut count = 0;

        for num in left..=right {
            let set_bits = num.count_ones();
            if prime_set_bits.contains(&(set_bits as i32)) {
                count += 1;
            }
        }
        count
    }
}
