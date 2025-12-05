struct Solution;

impl Solution {
    pub fn count_partitions(nums: Vec<i32>) -> i32 {
        let n = nums.len() as i32;
        let total_sum: i32 = nums.iter().sum();

        if total_sum % 2 != 0 {
            return 0;
        }
        return n - 1
    }
}
