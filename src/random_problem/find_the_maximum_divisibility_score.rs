struct Solution;

impl Solution {
    pub fn max_div_score(nums: Vec<i32>, divisors: Vec<i32>) -> i32 {
        let mut max_score = 0;
        let mut result_divisor = divisors[0];

        for &divisor in &divisors {
            let mut current_score = 0;

            for &num in &nums {
                if num % divisor == 0 {
                    current_score += 1;
                }
            }

            if current_score > max_score || (current_score == max_score && divisor < result_divisor)
            {
                max_score = current_score;
                result_divisor = divisor;
            }
        }

        result_divisor
    }
}
