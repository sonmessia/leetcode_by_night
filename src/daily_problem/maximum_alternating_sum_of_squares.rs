struct Solution;

impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut nums = nums;
        let n = nums.len();

        for i in 0..n {
            nums[i] = nums[i].pow(2);
        }

        nums.sort_unstable_by(|a, b| b.cmp(a));

        let mut ans = 0i64;

        for i in 0..(nums.len() / 2) {
            ans += nums[i] as i64;
            ans -= nums[nums.len() - 1 - i] as i64;
        }

        if nums.len() % 2 == 1 {
            ans += nums[nums.len() / 2] as i64;
        }
        ans
    }
}
