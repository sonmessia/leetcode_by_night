struct Solution;

impl Solution {
    pub fn minimum_deletions(nums: Vec<i32>) -> i32 {
        let n = nums.len();

        if n <= 2 {
            return n as i32;
        }
        let (mut min_index, mut max_index) = (0, 0);
        for i in 0..n {
            if nums[i] < nums[min_index] {
                min_index = i;
            }
            if nums[i] > nums[max_index] {
                max_index = i;
            }
        }

        let left = min_index.min(max_index) + 1;
        let right = n - min_index.max(max_index);

        (left + right).min(left).min(n - min_index.min(max_index)) as i32
    }
}
