struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32) -> i32 {
        nums.sort_unstable();
        let n = nums.len();

        let (mut left, mut total, mut max_freq) = (0, 0i64, 0);

        for right in 0..n {
            total += nums[right] as i64;

            while nums[right] as i64 * (right - left + 1) as i64 > total + k as i64 {
                total -= nums[left] as i64;
                left += 1;
            }

            max_freq = max_freq.max(right - left + 1);
        }
        max_freq as i32
    }
}
