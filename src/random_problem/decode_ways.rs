struct Solution;

impl Solution {
    pub fn num_decodings(s: String) -> i32 {
        if s.is_empty() || s.starts_with('0') {
            return 0;
        }

        let n = s.len();
        let mut dp = vec![0; n + 1];
        dp[0] = 1;
        dp[1] = 1;

        for i in 2..=n {
            let one_digit = &s[(i - 1)..i];
            let two_digits = &s[(i - 2)..i];

            if one_digit != "0" {
                dp[i] += dp[i - 1];
            }

            if two_digits >= "10" && two_digits <= "26" {
                dp[i] += dp[i - 2];
            }
        }

        dp[n]
    }
}
