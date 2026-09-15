struct Solution;

impl Solution {
    pub fn max_palindromes(s: String, k: i32) -> i32 {
        let n = s.len();
        if k == 1 {
            return n as i32;
        }

        let mut ans = 0;
        let k = k as usize;

        let slice: Vec<char> = s.chars().collect();
        let mut i = 0;

        while i + k <= n {
            if Self::is_palindrome(&slice, i, i + k - 1) {
                ans += 1;
                i += k - 1;
            } else if i + k < n && Self::is_palindrome(&slice, i, i + k) {
                ans += 1;
                i += k;
            }
            i += 1;
        }

        ans
    }

    fn is_palindrome(slice: &[char], mut left: usize, mut right: usize) -> bool {
        while left < right {
            if slice[left] != slice[right] {
                return false;
            }
            left += 1;
            right -= 1;
        }
        true
    }
}
