struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        fn gcd(mut x: i32, mut y: i32) -> i32 {
            while y != 0 {
                let tmp = x;
                x = y;
                y = tmp % y;
            }
            x
        }
        let mut cnt_ones = 0;
        let mut g = 0;

        for num in nums.iter() {
            if *num == 1 {
                cnt_ones += 1;
            }
            g = gcd(g, *num);
        }
        if cnt_ones > 0 {
            return (nums.len() - cnt_ones) as i32;
        }

        if g > 1 {
            return -1;
        }

        let mut ans = (nums.len() * nums.len()) as i32;

        let n = nums.len();
        for i in 0..(n - 1) {
            let mut current_gcd = nums[i];
            for j in (i + 1)..n {
                current_gcd = gcd(current_gcd, nums[j]);
                if current_gcd == 1 {
                    ans = ans.min((n + j - i - 1) as i32);
                    break;
                }
            }
        }
        ans
    }
}
