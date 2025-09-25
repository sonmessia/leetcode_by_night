struct Solution;

impl Solution {
    pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        let mut dp = vec![0; n + 1];
        for i in (0..n).rev() {
            for j in 0..=i {
                dp[j] = dp[j].min(dp[j + 1]) + triangle[i][j];
            }
        }
        dp[0]
    }
}
