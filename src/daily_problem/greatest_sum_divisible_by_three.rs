struct Solution;

impl Solution {
    pub fn max_sum_div_three(nums: Vec<i32>) -> i32 {
        let mut dp = vec![0; 3];
        for num in nums {
            let mut next_dp = dp.clone();
            // for i in 0..3 {
            //     let new_sum = dp[i] + num;
            //     next_dp[(new_sum % 3) as usize] = next_dp[(new_sum % 3) as usize].max(new_sum);
            // }
            for d in dp.iter() {
                let new_sum = d + num;
                next_dp[(new_sum % 3) as usize] = next_dp[(new_sum % 3) as usize].max(new_sum);
            }
            dp = next_dp;
        }
        dp[0]
    }
}
