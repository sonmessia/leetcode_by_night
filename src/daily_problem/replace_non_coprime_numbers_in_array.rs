struct Solution;

impl Solution {
    pub fn replace_non_coprime_numbers(nums: Vec<i32>) -> Vec<i32> {
        fn gcd(a: i32, b: i32) -> i32 {
            if b == 0 { a } else { gcd(b, a % b) }
        }
        let mut stack = Vec::new();
        for num in nums {
            let mut current = num;
            while let Some(&last) = stack.last() {
                let g = gcd(last, current);
                if g == 1 {
                    break;
                }
                current *= last / g;
                stack.pop();
            }
            stack.push(current);
        }
        stack
    }
}
