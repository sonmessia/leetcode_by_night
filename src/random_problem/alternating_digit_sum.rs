struct Solution;

impl Solution {
    pub fn alternate_digit_sum(n: i32) -> i32 {
        let mut sum = 0;
        let mut sign = 1;
        for c in n.to_string().chars() {
            sum += sign * (c as i32 - '0' as i32);
            sign *= -1;
        }
        sum
    }
}
