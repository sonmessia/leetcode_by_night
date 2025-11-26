struct Solution;

impl Solution {
    pub fn number_of_paths(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        const MOD: u32 = 1_000_000_007;
        let mut dp = vec![vec![vec![0; k as usize]; n]; m];
        dp[0][0][(grid[0][0] % k) as usize] = 1;

        for i in 0..m {
            for j in 0..n {
                for r in 0..k as usize {
                    if i > 0 {
                        let new_r = (r as i32 - grid[i][j] % k + k) % k;
                        dp[i][j][r] = (dp[i][j][r] + dp[i - 1][j][new_r as usize]) % MOD;
                    }
                    if j > 0 {
                        let new_r = (r as i32 - grid[i][j] % k + k) % k;
                        dp[i][j][r] = (dp[i][j][r] + dp[i][j - 1][new_r as usize]) % MOD;
                    }
                }
            }
        }

        dp[m - 1][n - 1][0] as i32
    }
}
