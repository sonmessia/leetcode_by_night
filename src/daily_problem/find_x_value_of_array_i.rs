struct Solution;

impl Solution {
    pub fn result_array(nums: Vec<i32>, k: i32) -> Vec<i64> {
        let k = k as usize;
        let n = nums.len();
        let mut ans = vec![0i64; k];

        let mut dp = vec![0; k];

        for i in 0..n {
            let mut new_dp = vec![0; k];

            new_dp[nums[i] as usize % k] += 1;

            for j in 0..k {
                new_dp[(j * nums[i] as usize) % k] += dp[j];
            }

            dp = new_dp;

            for j in 0..k {
                ans[j] += dp[j] as i64;
            }
        }

        ans
    }
}
