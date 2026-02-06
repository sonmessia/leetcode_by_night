struct Solution;

impl Solution {
    pub fn count_texts(pressed_keys: String) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = pressed_keys.len();
        let mut dp = vec![0i64; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            let current_char = pressed_keys.as_bytes()[i - 1];
            dp[i] = (dp[i] + dp[i - 1]) % MOD;

            if i >= 2 && pressed_keys.as_bytes()[i - 2] == current_char {
                dp[i] = (dp[i] + dp[i - 2]) % MOD;

                if i >= 3 && pressed_keys.as_bytes()[i - 3] == current_char {
                    dp[i] = (dp[i] + dp[i - 3]) % MOD;

                    if (current_char == b'7' || current_char == b'9')
                        && i >= 4
                        && pressed_keys.as_bytes()[i - 4] == current_char
                    {
                        dp[i] = (dp[i] + dp[i - 4]) % MOD;
                    }
                }
            }
        }

        dp[n] as i32
    }
}

