struct Solution;

impl Solution {
    pub fn sum_game(num: String) -> bool {
        let mut sum_left = 0;
        let mut sum_right = 0;
        let mut count_left = 0;
        let mut count_right = 0;

        for (i, c) in num.chars().enumerate() {
            if i < num.len() / 2 {
                if c == '?' {
                    count_left += 1;
                } else {
                    sum_left += c.to_digit(10).unwrap();
                }
            } else {
                if c == '?' {
                    count_right += 1;
                } else {
                    sum_right += c.to_digit(10).unwrap();
                }
            }
        }

        (count_left + count_right) % 2 == 1
            || (sum_left - sum_right) * 2 != (count_right - count_left) * 9
    }
}
