struct Solution;

impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        nums.iter()
            .enumerate()
            .map(|(i, _)| {
                let new_idx = (nums[i] % n as i32) + n as i32;
                nums[(i + new_idx as usize) % n]
            })
            .collect()
    }
}
