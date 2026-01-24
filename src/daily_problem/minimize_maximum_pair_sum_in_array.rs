struct Solution;

impl Solution {
    pub fn min_pair_sum(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        let n = nums.len();
        let (mut left, mut right) = (0, n - 1);
        let mut ans = 0;
        while left < right {
            ans = ans.max(nums[left] + nums[right]);
            left += 1;
            right -= 1;
        }
        ans
    }
}
