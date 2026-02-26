struct Solution;

impl Solution {
    pub fn num_steps(s: String) -> i32 {
        let n = s.len();
        let s = s.as_bytes();

        let (mut ans, mut carry) = (0, 0);

        for i in (1..n).rev() {
            if s[i] - b'0' + carry == 1 {
                ans += 2;
                carry = 1;
            } else {
                ans += 1;
            }
        }

        ans + carry as i32
    }
}
