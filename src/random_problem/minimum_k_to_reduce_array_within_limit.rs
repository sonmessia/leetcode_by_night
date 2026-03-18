struct Solution;

impl Solution {
    pub fn minimum_k(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let (mut left, mut right) = (1, 1_000_005);

        let non_positive = |nums: &[i32], k: i32| -> i64 {
            let mut total_cnt: i64 = 0;
            for &num in nums {
                if num <= k {
                    total_cnt += 1;
                } else {
                    total_cnt += (num as i64 + k as i64 - 1) / k as i64;
                }
            }
            total_cnt
        };

        while left <= right {
            let mid = left + (right - left) / 2;
            if mid == 0 {
                break;
            }

            if non_positive(&nums, mid) <= (mid as i64 * mid as i64) {
                ans = mid;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }

        ans
    }
}
