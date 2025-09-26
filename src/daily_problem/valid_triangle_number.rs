struct Solution;

impl Solution {
    pub fn triangle_number(nums: Vec<i32>) -> i32 {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut count = 0;
        for i in (2..n).rev() {
            let mut left = 0;
            let mut right = i - 1;
            while left < right {
                if nums[left] + nums[right] > nums[i] {
                    count += (right - left) as i32;
                    right -= 1;
                } else {
                    left += 1;
                }
            }
        }
        count
    }
}
