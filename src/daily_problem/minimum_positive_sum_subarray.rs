struct Solution;

impl Solution {
    pub fn minimum_sum_subarray(nums: Vec<i32>, l: i32, r: i32) -> i32 {
        let mut min_sum = i32::MAX;

        let mut prefix_sum = vec![0; nums.len() + 1];
        for i in 0..nums.len() {
            prefix_sum[i + 1] = prefix_sum[i] + nums[i];
        }

        for i in l as usize..=r as usize {
            for j in 0..=nums.len() - i {
                let sum = prefix_sum[j + i] - prefix_sum[j];
                if sum > 0 {
                    min_sum = min_sum.min(sum);
                }
            }
        }

        if min_sum != i32::MAX { min_sum } else { -1 }
    }
}
