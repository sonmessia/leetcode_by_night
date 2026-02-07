struct Solution;

impl Solution {
    pub fn minimum_deletions(s: String) -> i32 {
        let (mut ans, mut cnt_b) = (0, 0);

        for c in s.as_bytes() {
            if *c == b'b' {
                cnt_b += 1;
            } else {
                ans = (ans + 1).min(cnt_b);
            }
        }
        ans
    }
}
