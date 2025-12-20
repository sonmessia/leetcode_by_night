struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>, strategy: Vec<i32>, mut k: i32) -> i64 {
        let mut ans = 0;
        let n = prices.len();

        for i in 0..n {
            ans += prices[i] as i64 * strategy[i] as i64;
        }

        let mut tmp = ans;
        k /= 2;

        for i in 0..k {
            tmp -= (prices[i as usize] as i64) * (strategy[i as usize] as i64)
                + (prices[(i + k) as usize] as i64) * (strategy[(i + k) as usize] as i64);
            tmp += prices[(i + k) as usize] as i64;
        }

        ans = ans.max(tmp);

        for i in (k as usize * 2)..n {
            tmp += prices[i - 2 * k as usize] as i64 * strategy[i - 2 * k as usize] as i64;
            tmp -= prices[i - k as usize] as i64;
            tmp = tmp - prices[i] as i64 * strategy[i] as i64 + prices[i] as i64;
            ans = ans.max(tmp);
        }
        ans
    }
}
