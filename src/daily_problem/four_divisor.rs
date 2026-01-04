struct Solution;

impl Solution {
    pub fn sum_four_divisors(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for num in nums {
            let mut divisors = Vec::new();
            for k in 1..=num.isqrt() {
                if num % k == 0 {
                    divisors.push(k);
                    if k != num / k {
                        divisors.push(num / k);
                    }
                }
                if divisors.len() > 4 {
                    break;
                }
            }

            if divisors.len() == 4 {
                ans += divisors.iter().sum::<i32>();
            }
        }
        ans
    }
}
