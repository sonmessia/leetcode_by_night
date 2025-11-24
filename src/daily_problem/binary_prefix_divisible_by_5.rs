struct Solution;

impl Solution {
    pub fn prefixes_div_by5(nums: Vec<i32>) -> Vec<bool> {
        let mut result = Vec::with_capacity(nums.len());
        let mut prefix_mod = 0;

        for &num in &nums {
            prefix_mod = (prefix_mod * 2 + num) % 5;
            result.push(prefix_mod == 0);
        }

        result
    }
}
