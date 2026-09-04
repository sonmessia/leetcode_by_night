struct Solution;

impl Solution {
    pub fn first_stable_index(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        let mut max_num = nums[0];
        let mut min_array = vec![0; n];

        for i in (0..n).rev() {
            if i == n - 1 {
                min_array[i] = nums[i];
            } else {
                min_array[i] = min_array[i + 1].min(nums[i]);
            }
        }

        for i in 0..n {
            max_num = max_num.max(nums[i]);
            if max_num - min_array[i] <= k {
                return i as i32;
            }
        }

        -1
    }
}
