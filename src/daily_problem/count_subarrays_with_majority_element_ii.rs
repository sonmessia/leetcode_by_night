struct Solution;

impl Solution {
    pub fn count_majority_subarrays(nums: Vec<i32>, target: i32) -> i64 {
        let n = nums.len();
        let mut prefix_sum = vec![0; 2 * n + 1];
        prefix_sum[n] = 1;
        let mut count = n;
        let (mut ans, mut curr_sum) = (0i64, 0i64);

        for num in nums.iter() {
            if *num == target {
                curr_sum += prefix_sum[count] as i64;
                count += 1;
            } else {
                count -= 1;
                curr_sum -= prefix_sum[count] as i64;
            }

            prefix_sum[count] += 1;
            ans += curr_sum;
        }

        ans
    }
}
