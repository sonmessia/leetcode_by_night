struct Solution;

impl Solution {
    pub fn total_number(digits: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut digits_count = [0; 10];
        for &digit in &digits {
            digits_count[digit as usize] += 1;
        }

        for i in 1..=9 {
            if digits_count[i] == 0 {
                continue;
            }
            digits_count[i] -= 1;

            for j in 0..=9 {
                if digits_count[j] == 0 {
                    continue;
                }
                digits_count[j] -= 1;

                for k in (0..=8).step_by(2) {
                    if digits_count[k] > 0 {
                        count += 1;
                    }
                }

                digits_count[j] += 1;
            }

            digits_count[i] += 1;
        }

        count
    }
}
