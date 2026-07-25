struct Solution;

impl Solution {
    pub fn max_product(mut num: i32) -> i32 {
        let (mut first, mut second) = (0, 0);

        while num > 0 {
            let digit = num % 10;
            if digit > first {
                second = first;
                first = digit;
            } else if digit > second {
                second = digit;
            }
            num /= 10;
        }

        first * second
    }
}
