use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn longest_palindrome(s: String) -> i32 {
        let mut set: HashSet<char> = HashSet::new();
        let mut ans = 0;

        for ch in s.chars() {
            if set.contains(&ch) {
                ans += 2;
                set.remove(&ch);
                continue;
            }
            set.insert(ch);
        }

        if !set.is_empty() {
            ans += 1;
        }

        ans
    }
}
