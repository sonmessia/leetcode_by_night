struct Solution;

impl Solution {
    pub fn min_element(nums: Vec<i32>) -> i32 {
        nums.into_iter()
            .map(|num| {
                let mut sum = 0;
                let mut n = num;
                while n > 0 {
                    sum += n % 10;
                    n /= 10;
                }
                sum
            })
            .min()
            .unwrap_or(0)
    }
}
