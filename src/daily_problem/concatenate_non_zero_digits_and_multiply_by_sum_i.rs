struct Solution;

impl Solution {
    pub fn sum_and_multiply(n: i32) -> i64 {
        let mut sum = 0;
        let mut product = 0;

        for digit in n.to_string().chars() {
            let digit = digit.to_digit(10).unwrap() as i64;
            if digit != 0 {
                sum += digit;
                product = product * 10 + digit;
            }
        }

        sum * product
    }
}
