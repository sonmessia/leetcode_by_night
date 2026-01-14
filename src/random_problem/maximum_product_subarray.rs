struct Solution;

impl Solution {
    pub fn max_product(nums: Vec<i32>) -> i32 {
        let mut max_product = nums[0];
        let mut min_product = nums[0];
        let mut result = nums[0];

        for &num in nums.iter().skip(1) {
            if num < 0 {
                std::mem::swap(&mut max_product, &mut min_product);
            }

            max_product = std::cmp::max(num, max_product * num);
            min_product = std::cmp::min(num, min_product * num);

            result = std::cmp::max(result, max_product);
        }

        result
    }
}
