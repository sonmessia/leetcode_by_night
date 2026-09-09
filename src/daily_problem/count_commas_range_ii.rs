struct Solution;

impl Solution {
    pub fn count_commas(n: i64) -> i64 {
        if n < 1000 {
            return 0;
        }

        let mut ans = 0i64;
        let mut commas = 1;
        let mut start = 1000i64;

        while start <= n {
            let end = start * 1000 - 1;
            let total_numbers = n.min(end) - start + 1;
            ans += total_numbers * commas;
            if end > n {
                break;
            }
            start *= 1000;
            commas += 1;
        }
        ans
    }
}
