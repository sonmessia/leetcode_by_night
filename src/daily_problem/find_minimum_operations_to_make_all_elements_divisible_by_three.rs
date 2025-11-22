struct Solution;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let mut ans = 0;

        for num in nums {
            let remainder = num % 3;
            if remainder != 0 {
                ans += 1;
            }
        }
        ans
    }
}
