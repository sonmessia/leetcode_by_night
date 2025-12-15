struct Solution;

impl Solution {
    pub fn get_descent_periods(prices: Vec<i32>) -> i64 {
        let n = prices.len();

        let (mut ans, mut subarray) = (1i64, 1i64);

        for i in 1..n {
            subarray = if prices[i - 1] - prices[i] == 1 {
                subarray + 1
            } else {
                1
            };
            ans += subarray;
        }

        ans
    }
}
