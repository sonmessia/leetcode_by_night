struct Solution;

impl Solution {
    pub fn left_right_difference(nums: Vec<i32>) -> Vec<i32> {
        let mut left_sum = 0;
        let mut right_sum: i32 = nums.iter().sum();
        let mut result = Vec::with_capacity(nums.len());

        for num in nums {
            right_sum -= num;
            result.push((left_sum - right_sum).abs());
            left_sum += num;
        }

        result
    }
}
