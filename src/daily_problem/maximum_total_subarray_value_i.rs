struct Solution;

impl Solution {
    pub fn max_total_value(nums: Vec<i32>, k: i32) -> i64 {
        let max_value = nums.iter().map(|&x| x as i64).max().unwrap_or(0);
        let min_value = nums.iter().map(|&x| x as i64).min().unwrap_or(0);

        (max_value - min_value) * k as i64
    }
}
