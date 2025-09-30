struct Solution;

impl Solution {
    pub fn triangular_sum(mut nums: Vec<i32>) -> i32 {
        let mut n = nums.len();
        for i in (0..n).rev() {
            for i in 0..(i - 1) {
                nums[i] = (nums[i] + nums[i + 1]) % 10;
            }
        }
        nums[0]
    }
}
