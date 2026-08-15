struct Solution;

impl Solution {
    pub fn longest_subsequence(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        let mut full_zero = true;
        for i in 0..n {
            ans ^= nums[i];

            if nums[i] > 0 {
                full_zero = false;
            }
        }

        if ans > 0 {
            return n as i32;
        } else if full_zero {
            return 0;
        } else {
            return n as i32 - 1;
        }
    }
}
