struct Solution;

impl Solution {
    pub fn longest_palindromic_subsequence(s: String, k: i32) -> i32 {
        let n = s.len();
        let k = k as usize;
        let s = s.as_bytes();

        let mut dp = vec![vec![vec![0; k + 1]; n]; n];

        for i in 0..n {
            for dist in 0..=k {
                dp[i][i][dist] = 1;
            }
        }

        for len in 2..=n {
            for i in 0..=n - len {
                let j = i + len - 1;
                for cur_k in 0..=k {
                    let mut res = dp[i + 1][j][cur_k].max(dp[i][j - 1][cur_k]);

                    let diff = (s[i] as i32 - s[j] as i32).abs();
                    let cost = diff.min(26 - diff) as usize;

                    if cur_k >= cost {
                        let prev_val = if i + 1 <= j - 1 {
                            dp[i + 1][j - 1][cur_k - cost]
                        } else {
                            0
                        };
                        res = res.max(2 + prev_val);
                    }
                    dp[i][j][cur_k] = res;
                }
            }
        }

        dp[0][n - 1][k]
    }
}
