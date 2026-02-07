struct Solution;

impl Solution {
    pub fn maximum_amount(coins: Vec<Vec<i32>>) -> i32 {
        let n = coins.len();
        let m = coins[0].len();
        let mut dp = vec![vec![vec![i32::MIN; 3]; m]; n];
        dp[0][0][0] = coins[0][0];
        if coins[0][0] < 0 {
            dp[0][0][1] = 0;
        }

        for i in 0..n {
            for j in 0..m {
                if i == 0 && j == 0 {
                    continue;
                }

                for k in 0..3 {
                    if i > 0 && dp[i - 1][j][k] != i32::MIN {
                        dp[i][j][k] = dp[i][j][k].max(dp[i - 1][j][k] + coins[i][j]);

                        if k < 2 && coins[i][j] < 0 {
                            dp[i][j][k + 1] = dp[i][j][k + 1].max(dp[i - 1][j][k]);
                        }
                    }

                    if j > 0 && dp[i][j - 1][k] != i32::MIN {
                        dp[i][j][k] = dp[i][j][k].max(dp[i][j - 1][k] + coins[i][j]);

                        if k < 2 && coins[i][j] < 0 {
                            dp[i][j][k + 1] = dp[i][j][k + 1].max(dp[i][j - 1][k]);
                        }
                    }
                }
            }
        }
        let mut ans = i32::MIN;
        for k in 0..3 {
            ans = ans.max(dp[n - 1][m - 1][k]);
        }

        ans
    }
}
