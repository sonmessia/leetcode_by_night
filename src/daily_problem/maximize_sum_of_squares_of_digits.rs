struct Solution;

impl Solution {
    pub fn max_sum_of_squares(mut num: i32, mut sum: i32) -> String {
        if sum > 9 * num {
            return "".to_string();
        }

        let mut ans = String::new();

        while sum >= 0 && num > 0 {
            if sum >= 9 {
                ans.push('9');
                sum -= 9;
            } else {
                ans.push_str(&sum.to_string());
                sum = 0;
            }
            num -= 1;
        }

        ans
    }
}
