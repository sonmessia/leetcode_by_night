struct Solution;

impl Solution {
    pub fn max_subarray_length(nums: Vec<i32>, k: i32) -> i32 {
        let mut count = std::collections::HashMap::new();
        let mut left = 0;
        let mut max_length = 0;

        for right in 0..nums.len() {
            *count.entry(nums[right]).or_insert(0) += 1;

            while *count.entry(nums[right]).or_insert(0) > k {
                *count.get_mut(&nums[left]).unwrap() -= 1;
                if count[&nums[left]] == 0 {
                    count.remove(&nums[left]);
                }
                left += 1;
            }

            max_length = max_length.max((right - left + 1) as i32);
        }

        max_length
    }
}
