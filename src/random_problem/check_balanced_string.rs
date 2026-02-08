struct Solution;

impl Solution {
    pub fn is_balanced(num: String) -> bool {
        let mut even_sum: i32 = 0;
        let mut odd_sum: i32 = 0;

        num.chars().enumerate().for_each(|(i, c)| {
            let digit = c.to_digit(10).unwrap() as i32;
            if i % 2 == 0 {
                even_sum += digit;
            } else {
                odd_sum += digit;
            }
        });

        even_sum == odd_sum
    }
}
