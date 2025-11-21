struct Solution;

impl Solution {
    pub fn keep_multiplying_found_values_by_two(nums: Vec<i32>, original: i32) -> i32 {
        let num_set: std::collections::HashSet<i32> = nums.into_iter().collect();
        let mut current = original;

        while num_set.contains(&current) {
            current *= 2;
        }

        current
    }
}
