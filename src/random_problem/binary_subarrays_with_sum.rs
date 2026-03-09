struct Solution;

impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        let n = nums.len();
        let mut count = vec![0; n + 1];
        count[0] = 1;
        let mut sum = 0;
        let mut ans = 0;
        for num in nums {
            sum += num;
            if sum >= goal {
                ans += count[sum as usize - goal as usize];
            }
            count[sum as usize] += 1;
        }
        ans
    }
}
