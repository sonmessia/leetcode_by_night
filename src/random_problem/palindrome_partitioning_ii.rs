struct Solution;

impl Solution {
    pub fn min_cut(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();
        let mut dp = vec![0; n + 1];
        for i in 0..=n {
            dp[i] = i as i32 - 1;
        }

        println!("Initial dp: {:?}", dp);

        for i in 0..n {
            for j in 0..=i {
                if Self::is_palindrome(&s[j..=i]) {
                    dp[i + 1] = dp[i + 1].min(dp[j] + 1);
                }
            }
        }
        dp[n]
    }

    fn is_palindrome(s: &[u8]) -> bool {
        let mut left = 0;
        let mut right = s.len() - 1;
        while left < right {
            if s[left] != s[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
