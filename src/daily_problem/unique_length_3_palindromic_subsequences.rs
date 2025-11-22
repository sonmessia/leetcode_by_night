use std::collections::HashSet;
struct Solution;

impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        let chars: Vec<char> = s.chars().collect();
        let mut result = 0;

        for c in 'a'..='z' {
            let first = chars.iter().position(|&x| x == c);
            let last = chars.iter().rposition(|&x| x == c);

            if let (Some(l), Some(r)) = (first, last)
                && (l < r)
            {
                let mut mids = HashSet::new();
                for c in chars.iter().take(r).skip(l + 1) {
                    mids.insert(c);
                }
                result += mids.len();
            }
        }

        result as i32
    }
}
