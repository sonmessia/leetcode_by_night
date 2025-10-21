struct Solution;

impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        use std::collections::HashMap;
        nums.sort_unstable();

        let mut cnt = HashMap::new();

        for &num in &nums {
            *cnt.entry(num).or_insert(0) += 1;
        }

        let (mut max_freq, mut start_idx, mut end_idx) = (0, 0, 0);

        for ele in nums[0]..=nums[nums.len() - 1] {
            while end_idx < nums.len() && nums[end_idx] <= ele + k {
                end_idx += 1;
            }

            while nums[start_idx] < ele - k {
                start_idx += 1;
            }

            let window_size = end_idx - start_idx;
            let max_window_size = cnt.get(&ele).unwrap_or(&0) + num_operations as usize;
            let freq = window_size.min(max_window_size);

            max_freq = max_freq.max(freq);
        }

        max_freq as i32
    }
}

fn main() {
    let nums = vec![1, 3, 5, 5, 7];
    let k = 2;
    let num_operations = 2;
    let result = Solution::max_frequency(nums, k, num_operations);
    println!("Maximum frequency: {}", result); // Output: 4
}
