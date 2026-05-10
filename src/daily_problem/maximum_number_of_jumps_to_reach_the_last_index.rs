struct Solution;

impl Solution {
    pub fn maximum_jumps(nums: Vec<i32>, target: i32) -> i32 {
        let n = nums.len();
        let mut dp = vec![-1; n];
        dp[0] = 0;
        for i in 1..n {
            for j in (0..i).rev() {
                if dp[j] != -1 && (nums[i] - nums[j]).abs() <= target {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        dp[n - 1]
    }
}