struct Solution;

impl Solution {
    pub fn total_money(n: i32) -> i32 {
        let (mut ans, mut m) = (0, 1);

        for i in 1..=n {
            ans += m;
            if i % 7 == 0 {
                m += i / 7;
            }
            m += 1;
        }

        ans
    }
}
