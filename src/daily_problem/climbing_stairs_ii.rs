struct Solution;

impl Solution {
    pub fn climb_stairs(n: i32, costs: Vec<i32>) -> i32 {
        let mut dp = vec![0; 3];

        for x in costs {
            let min_cost = (dp[0] + 1).min(dp[1] + 4).min(dp[2] + 9) + x;
            dp[2] = dp[1];
            dp[1] = dp[0];
            dp[0] = min_cost;
        }

        dp[0]
    }
}
