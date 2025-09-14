struct Solution;

impl Solution {
    pub fn maximum_prime_difference(nums: Vec<i32>) -> i32 {
        fn is_prime(num: i32) -> bool {
            if num < 2 {
                return false;
            }
            for i in 2..=((num as f64).sqrt() as i32) {
                if num % i == 0 {
                    return false;
                }
            }
            true
        }
        let n = nums.len();

        let (mut left, mut right) = (0, n - 1);
        while left <= right {
            if is_prime(nums[left]) && is_prime(nums[right]) {
                return (right - left) as i32;
            }
            if !is_prime(nums[left]) {
                left += 1;
            }
            if !is_prime(nums[right]) {
                if right == 0 {
                    break;
                }
                right -= 1;
            }
        }
        -1
    }
}
