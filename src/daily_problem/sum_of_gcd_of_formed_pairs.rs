struct Solution;

impl Solution {
    pub fn gcd_sum(nums: Vec<i32>) -> i64 {
        let gcd = |mut a: i32, mut b: i32| -> i32 {
            while b != 0 {
                let temp = b;
                b = a % b;
                a = temp;
            }
            a
        };

        let n = nums.len();
        let mut prefix_gdc = Vec::with_capacity(n);
        prefix_gdc.push(nums[0]);
        let mut max_i = nums[0];
        for &num in nums.iter().skip(1) {
            max_i = max_i.max(num);
            prefix_gdc.push(gcd(max_i, num));
        }

        prefix_gdc.sort_unstable();
        let mut ans = 0;
        let (mut l, mut r) = (0, n - 1);

        while l < r {
            ans += gcd(prefix_gdc[l], prefix_gdc[r]) as i64;
            l += 1;
            r -= 1;
        }

        ans
    }
}
