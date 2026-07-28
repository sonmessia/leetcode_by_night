struct Solution;

impl Solution {
    pub fn smallest_palindrome(s: String) -> String {
        let s = s.as_bytes();

        let mut freq = vec![0; 26];

        for i in 0..s.len() / 2 {
            freq[(s[i] - b'a') as usize] += 1;
        }

        let mut left = String::new();

        for i in 0..26 {
            for _ in 0..freq[i] {
                left.push((i as u8 + b'a') as char);
            }
        }

        let right = left.chars().rev().collect::<String>();

        let mut ans = left.clone();

        if s.len() % 2 == 1 {
            ans.push(s[s.len() / 2] as char);
        }

        ans.push_str(&right);

        ans
    }
}
