struct Solution;

impl Solution {
    pub fn max_path_score(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let k = k as usize;
        let mut dp = vec![vec![vec![i32::MIN; k + 1]; n]; m];

        dp[0][0][0] = 0;

        for i in 0..m {
            for j in 0..n {
                for l in 0..=k {
                    if dp[i][j][l] == i32::MIN {
                        continue;
                    }

                    if i + 1 < m {
                        let cost = if grid[i + 1][j] == 0 { 0 } else { 1 };
                        if cost + l <= k {
                            dp[i + 1][j][cost + l] =
                                dp[i + 1][j][cost + l].max(dp[i][j][l] + grid[i + 1][j]);
                        }
                    }

                    if j + 1 < n {
                        let cost = if grid[i][j + 1] == 0 { 0 } else { 1 };
                        if cost + l <= k {
                            dp[i][j + 1][cost + l] =
                                dp[i][j + 1][cost + l].max(dp[i][j][l] + grid[i][j + 1]);
                        }
                    }
                }
            }
        }

        let mut ans = -1;
        for i in 0..=k {
            ans = ans.max(dp[m - 1][n - 1][i]);
        }

        ans
    }
}
