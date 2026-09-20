struct Solution;

impl Solution {
    pub fn reverse_degree(s: String) -> i32 {
        let mut ans = 0;
        for (idx, ch) in s.chars().enumerate() {
            ans += (b'z' - ch as u8 + 1) as i32 * (idx as i32 + 1);
        }

        ans
    }
}
