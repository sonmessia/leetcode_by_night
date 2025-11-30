struct Solution;

impl Solution {
    pub fn max_subarray_sum_div_by_k(nums: Vec<i32>, k: i32) -> i64 {
        let n = nums.len();
        let k = k as usize;
        let (mut prefix_sums, mut ans) = (0i64, i64::MIN);

        let mut k_sum = vec![i64::MAX / 2; k as usize];

        k_sum[k - 1] = 0;

        for i in 0..n {
            prefix_sums += nums[i] as i64;
            ans = ans.max(prefix_sums - k_sum[(i% k)]);
            k_sum[(i % k)] = k_sum[(i % k)].min(prefix_sums);
        }
        ans
    }
}
