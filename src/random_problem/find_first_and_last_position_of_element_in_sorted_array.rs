struct Solution;

impl Solution {
    pub fn search_range(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        let mut result = vec![-1, -1];
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                result[0] = mid;
                right = mid - 1;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        left = 0;
        right = nums.len() as i32 - 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                result[1] = mid;
                left = mid + 1;
            } else if nums[mid as usize] < target {
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }
        result
    }
}
