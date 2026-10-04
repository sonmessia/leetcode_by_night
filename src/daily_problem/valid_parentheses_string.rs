struct Solution;

impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut left_balance = 0;
        let mut right_balance = 0;

        for c in s.chars() {
            if c == '(' {
                left_balance += 1;
                right_balance += 1;
            } else if c == ')' {
                left_balance -= 1;
                right_balance -= 1;
            } else {
                left_balance -= 1;
                right_balance += 1;
            }

            if right_balance < 0 {
                return false;
            }

            if left_balance < 0 {
                left_balance = 0;
            }
        }

        left_balance == 0
    }
}
