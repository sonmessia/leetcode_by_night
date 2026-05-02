struct Solution;

impl Solution {
    pub fn rotated_digits(n: i32) -> i32 {
        let mut ans = 0;
        let is_good = |mut num: i32| -> bool {
            let mut has_different = false;
            while num > 0 {
                match num % 10 {
                    2 | 5 | 6 | 9 => has_different = true,
                    3 | 4 | 7 => return false,
                    _ => {},
                }
                num /= 10;
            }
            has_different
        };
        for i in 1..=n {
            if is_good(i) {
                ans += 1;
            }
        }
        ans
    }
}

