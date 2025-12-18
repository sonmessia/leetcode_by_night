struct Solution;

impl Solution {
    pub fn maximum_profit(prices: Vec<i32>, k: i32) -> i64 {
        let mut best = vec![0i64; (k + 1) as usize];
        let mut hold = vec![i64::MIN; k as usize];
        let mut cash = vec![0i64; k as usize];

        for price in prices {
            for j in (1..=k as usize).rev() {
                best[j] = best[j]
                    .max(cash[j - 1] - price as i64)
                    .max(hold[j - 1] + price as i64);
                cash[j - 1] = cash[j - 1].max(best[j - 1] + price as i64);
                hold[j - 1] = hold[j - 1].max(best[j - 1] - price as i64);
            }
        }

        best.iter().copied().max().unwrap_or(0)
    }
}
