struct Solution;

impl Solution {
    pub fn minimum_difference(nums: Vec<i32>, k: i32) -> i32 {
        if k == 1 {
            return 0;
        }
        let mut nums = nums;
        nums.sort_unstable();
        let k = k as usize;
        let mut min_diff = i32::MAX;
        for i in 0..=nums.len() - k {
            let diff = nums[i + k - 1] - nums[i];
            if diff < min_diff {
                min_diff = diff;
            }
        }
        min_diff
    }
}
