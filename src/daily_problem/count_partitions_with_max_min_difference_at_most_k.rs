struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let total_sum: i32 = nums.iter().sum();
        if total_sum < 2 * k {
            return 0;
        }

        let target = (total_sum - 2 * k) as usize;
        let mut dp = vec![0i64; target + 1];
        dp[0] = 1;

        for &num in &nums {
            let num = num as usize;
            for j in (num..=target).rev() {
                dp[j] = (dp[j] + dp[j - num]) % MOD;
            }
        }

        let mut result = 0i64;
        for count in dp {
            result = (result + count) % MOD;
        }

        // Each partition is counted twice
        result = (result * 500000004) % MOD; // Multiplying by modular inverse of 2

        result as i32
    }
}
