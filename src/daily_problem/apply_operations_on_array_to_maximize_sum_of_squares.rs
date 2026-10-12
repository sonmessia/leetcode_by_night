struct Solution;

impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let mut ans = 0;

        let mut counts = vec![0; 32];
        for num in nums.iter() {
            let mut num = *num;
            for i in 0..32 {
                if num & 1 == 1 {
                    counts[i] += 1;
                }
                num >>= 1;
            }
        }

        for _ in 0..k {
            let mut num = 0i64;
            for j in (0..32).rev() {
                if counts[j] > 0 {
                    num |= 1 << j;
                    counts[j] -= 1;
                }
            }

            num = num * num % MOD as i64;
            ans = (ans + num as i32) % MOD;
        }

        ans
    }
}
