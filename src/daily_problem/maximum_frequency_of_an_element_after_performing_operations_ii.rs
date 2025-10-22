struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        fn binary_search_left(nums: &[i32], target: i32) -> usize {
            let mut left = 0;
            let mut right = nums.len();
            while left < right {
                let mid = left + (right - left) / 2;
                if nums[mid] < target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        fn binary_search_right(nums: &[i32], target: i32) -> usize {
            let mut left = 0;
            let mut right = nums.len();
            while left < right {
                let mid = left + (right - left) / 2;
                if nums[mid] <= target {
                    left = mid + 1;
                } else {
                    right = mid;
                }
            }
            left
        }

        use std::collections::HashMap;
        nums.sort_unstable();
        let mut ans = 0;
        let mut cnt = HashMap::new();

        for &num in &nums {
            *cnt.entry(num).or_insert(0) += 1;
        }

        let check = |num: i32| -> i32 {
            let l = binary_search_left(&nums, num.saturating_sub(k));
            let r = binary_search_right(&nums, num.saturating_add(k));

            let range = (r - l) as i32;

            let count_num = *cnt.get(&num).unwrap_or(&0);

            range.min(count_num + num_operations)
        };

        for i in 0..nums.len() {
            ans = ans.max(
                check(nums[i])
                    .max(check(nums[i] + k))
                    .max(check(nums[i] - k)),
            );
        }
        ans
    }
}
