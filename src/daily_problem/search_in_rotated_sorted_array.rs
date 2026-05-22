struct Solution;

impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> i32 {
        let (mut left, mut right) = (0, nums.len() as i32 - 1);
        while left <= right {
            let mid = left + (right - left) / 2;
            if nums[mid as usize] == target {
                return mid;
            }
            if nums[left as usize] <= nums[mid as usize] {
                match nums[left as usize] <= target && target < nums[mid as usize] {
                    true => {
                        right = mid - 1;
                    }
                    false => {
                        left = mid + 1;
                    }
                }
            } else {
                match nums[mid as usize] < target && target <= nums[right as usize] {
                    true => {
                        left = mid + 1;
                    }
                    false => {
                        right = mid - 1;
                    }
                }
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_search() {
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 0;
        assert_eq!(Solution::search(nums, target), 4);
        let nums = vec![4, 5, 6, 7, 0, 1, 2];
        let target = 3;
        assert_eq!(Solution::search(nums, target), -1);
        let nums = vec![1];
        let target = 0;
        assert_eq!(Solution::search(nums, target), -1);
    }
}
