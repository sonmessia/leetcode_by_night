struct Solution;

impl Solution {
    pub fn smallest_number(n: i32, t: i32) -> i32 {
        for i in n..=100 {
            let mut product = 1;
            let mut num = i;
            while num > 0 {
                let digit = num % 10;
                product *= digit;
                if product % t == 0 {
                    return i;
                }
                num /= 10;
            }
            if product % t == 0 {
                return i;
            }
        }
        0
    }
}
