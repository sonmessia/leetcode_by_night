struct Solution;

impl Solution {
    pub fn min_difference(mut nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 4 {
            return 0;
        }
        nums.sort_unstable();

        (nums[n - 3] - nums[0])
            .min(nums[n - 2] - nums[1])
            .min(nums[n - 1] - nums[2])
    }
}
