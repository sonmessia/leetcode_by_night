struct Solution;

impl Solution {
    pub fn digit_sum(s: String, k: i32) -> String {
        let mut s = s;
        let k = k as usize;

        while s.len() > k {
            let mut new_s = String::new();
            for chunk in s.as_bytes().chunks(k) {
                let sum: u32 = chunk.iter().map(|&c| (c - b'0') as u32).sum();
                new_s.push_str(&sum.to_string());
            }
            s = new_s;
        }

        s
    }
}
