struct Solution;

impl Solution {
    pub fn sequential_digits(low: i32, high: i32) -> Vec<i32> {
        let mut ans = vec![];

        for i in 1..=9 {
            let mut number = i;
            for j in (i + 1)..=9 {
                number = number * 10 + j;
                if number >= low && number <= high {
                    ans.push(number);
                }
            }
        }
        ans.sort_unstable();
        ans
    }
}
