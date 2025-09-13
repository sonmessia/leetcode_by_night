struct Solution;

impl Solution {
    pub fn string_hash(s: String, k: i32) -> String {
        let mut ans = String::new();
        let s = s.into_bytes();

        for c in s.chunks(k as usize) {
            let sum = c.iter().map(|&x| (x - b'a') as i32).sum::<i32>();
            ans.push(((sum % 26) as u8 + b'a') as char);
        }
        ans
    }
}
