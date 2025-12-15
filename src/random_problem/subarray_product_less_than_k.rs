struct Solution;

impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k <= 1 {
            return 0;
        }

        let n = nums.len();

        let (mut ans, mut product) = (0, 1);

        let mut left = 0;
        for right in 0..n {
            product *= nums[right];

            while product >= k {
                product /= nums[left];
                left += 1;
            }

            ans += right - left + 1;
        }

        ans as i32
    }
}
