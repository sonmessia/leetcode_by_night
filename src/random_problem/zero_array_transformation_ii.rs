struct Solution;

impl Solution {
    pub fn check(nums: &Vec<i32>, queries: &Vec<Vec<i32>>, k: usize) -> bool {
        let n = nums.len();
        let mut sum = 0;

        let mut diff_array = vec![0; n + 1];
        for i in 0..k {
            let (l, r, val) = (queries[i][0], queries[i][1], queries[i][2]);

            diff_array[l as usize] += val;
            diff_array[r as usize + 1] -= val;
        }

        for i in 0..n {
            sum += diff_array[i];
            if sum < nums[i] {
                return false;
            }
        }
        return true;
    }

    pub fn min_zero_array(nums: Vec<i32>, queries: Vec<Vec<i32>>) -> i32 {
        if nums.iter().all(|&x| x == 0) {
            return 0;
        }

        let (mut left, mut right) = (0, queries.len());

        let mut ans = -1;
        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::check(&nums, &queries, mid) {
                ans = mid as i32;
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        ans
    }
}
