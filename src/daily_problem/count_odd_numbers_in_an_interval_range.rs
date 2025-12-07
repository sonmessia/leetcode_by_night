struct Solution;

impl Solution {
    pub fn count_odd_numbers(low: i32, high: i32) -> i32 {
        (high - low) / 2 + (low % 2 == 1 || high % 2 == 1) as i32
    }
}
