struct Solution;

impl Solution {
    pub fn stone_game_ii(piles: Vec<i32>) -> i32 {
        let n = piles.len();

        let mut dp = vec![vec![0; n + 1]; n + 1];

        for i in (0..n).rev() {
            dp[i][n] = dp[i + 1][n] + piles[i];
        }

        fn dfs(i: usize, m: usize, n: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
            if i >= n {
                return 0;
            }

            if i + 2 * m >= n {
                return dp[i][n];
            }
            if dp[i][m] != 0 {
                return dp[i][m];
            }

            let mut best = 0;

            for x in 1..=2 * m {
                if i + x > n {
                    break;
                }

                best = best.max(dp[i][n] - dfs(i + x, x.max(m), n, dp));
            }

            dp[i][m] = best;
            best
        }

        dfs(0, 1, n, &mut dp)
    }
}
