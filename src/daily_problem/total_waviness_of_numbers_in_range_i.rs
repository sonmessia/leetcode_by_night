struct Solution;

impl Solution {
    pub fn total_waviness(num1: i32, num2: i32) -> i32 {
        let mut ans = 0;

        let count_wavely = |num: i32| -> i32 {
            let s = num
                .to_string()
                .chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect::<Vec<_>>();

            let mut res = 0;

            for i in 1..s.len() - 1 {
                if (s[i] > s[i - 1] && s[i] > s[i + 1]) || (s[i] < s[i - 1] && s[i] < s[i + 1]) {
                    res += 1;
                }
            }

            return res;
        };
        for i in num1..=num2 {
            ans += count_wavely(i);
        }

        ans
    }
}
