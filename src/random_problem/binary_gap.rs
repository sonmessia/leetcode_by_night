struct Solution;

impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut last = -1;
        let mut ans = 0;

        for i in 0..32 {
            if (n >> i) & 1 == 1 {
                if last != -1 {
                    ans = std::cmp::max(ans, i - last);
                }
                last = i;
            }
        }
        ans
    }
}
