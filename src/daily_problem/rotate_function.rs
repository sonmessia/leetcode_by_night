struct Solution;

impl Solution {
    pub fn max_rotate_function(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let total_sum: i32 = nums.iter().sum();
        let mut curr_max = nums
            .iter()
            .enumerate()
            .fold(0, |acc, (i, &x)| acc + i as i32 * x);
        let mut max_ans = curr_max;

        for i in 1..n {
            curr_max = curr_max + total_sum - n as i32 * nums[n - i];
            max_ans = max_ans.max(curr_max);
        }
        max_ans
    }
}
