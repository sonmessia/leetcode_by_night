struct Solution;

impl Solution {
    fn gcd(a: i32, b: i32) -> i32 {
        if b == 0 { a } else { Self::gcd(b, a % b) }
    }

    fn lcm(a: i32, b: i32) -> i32 {
        (a * b) / Self::gcd(a, b)
    }

    pub fn max_length(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ans = 0;

        for i in 0..n {
            let mut product = 1;
            let mut lcm_value = 1;
            let mut gcd_value = 0;
            for j in i..n {
                product *= nums[j];
                lcm_value = Self::lcm(lcm_value, nums[j]);
                gcd_value = if gcd_value == 0 {
                    nums[j]
                } else {
                    Self::gcd(gcd_value, nums[j])
                };

                if product == lcm_value * gcd_value {
                    ans = ans.max(j - i + 1);
                }
            }
        }

        ans as i32
    }
}
