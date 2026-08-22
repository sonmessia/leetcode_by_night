struct Solution;

impl Solution {
    pub fn check_divisibility(n: i32) -> bool {
        let mut sum = 0;
        let mut product = 1;
        let mut temp = n;

        while temp > 0 {
            let digit = temp % 10;
            sum += digit;
            product *= digit;
            temp /= 10;
        }

        n % (sum + product) == 0
    }
}
