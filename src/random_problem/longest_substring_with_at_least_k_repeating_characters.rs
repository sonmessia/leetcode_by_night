struct Solution;

impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        fn helper(s: &[u8], k: i32) -> i32 {
            if s.len() < k as usize {
                return 0;
            }

            let mut count = [0; 26];
            for &c in s {
                count[(c - b'a') as usize] += 1;
            }

            for (i, &c) in count.iter().enumerate() {
                if c > 0 && c < k {
                    let split_char = (i as u8) + b'a';
                    return s
                        .split(|&ch| ch == split_char)
                        .map(|sub| helper(sub, k))
                        .max()
                        .unwrap_or(0);
                }
            }

            s.len() as i32
        }

        helper(s.as_bytes(), k)
    }
}
