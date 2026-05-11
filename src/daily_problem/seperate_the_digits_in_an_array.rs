struct Solution;

impl Solution {
    pub fn separate_digits(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for num in nums {
            let mut stack = vec![];
            let mut n = num;
            while n > 0 {
                stack.push(n % 10);
                n /= 10;
            }
            while let Some(digit) = stack.pop() {
                ans.push(digit);
            }
        }
        ans
    }
}
