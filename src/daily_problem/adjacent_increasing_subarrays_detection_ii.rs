struct Solution;

impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        fn check(nums: &Vec<i32>, k: i32) -> bool {
            let (mut cnt, n) = (0, nums.len());
            let k = k as usize;
            if k == 1 {
                return true;
            }
            for i in 1..n - k {
                if nums[i] > nums[i - 1] && nums[i + k] > nums[i + k - 1] {
                    cnt += 1;
                } else {
                    cnt = 0;
                }
                if cnt == k - 1 {
                    return true;
                }
            }
            false
        }

        let (mut left, mut right) = (1, nums.len() as i32);
        let mut ans = 1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if check(&nums, mid) {
                ans = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        ans
    }
}
