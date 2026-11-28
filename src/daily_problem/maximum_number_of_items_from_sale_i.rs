struct Solution;

impl Solution {
    pub fn maximum_sale_items(items: Vec<Vec<i32>>, budget: i32) -> i32 {
        let n = items.len();
        let max_budget = budget as usize;

        let mut free_bonus = vec![0i32; n];

        for i in 0..n {
            let fi = items[i][0] as usize;
            for j in 0..n {
                if i != j && items[j][0] % fi as i32 == 0 {
                    free_bonus[i] += 1;
                }
            }
        }

        let mut dp = vec![0i32; max_budget + 1];
        let mut min_price = i32::MAX;

        for (i, item) in items.iter().enumerate() {
            let price = item[1];
            min_price = min_price.min(price);

            if price > max_budget as i32 {
                continue;
            }

            let gain = 1 + free_bonus[i];

            for w in (price..=max_budget as i32).rev() {
                let w_usize = w as usize;
                dp[w_usize] = dp[w_usize].max(dp[w_usize - price as usize] + gain);
            }
        }

        let mut ans = 0;
        for w in 0..=max_budget {
            let remaining_budget = budget - w as i32;
            ans = ans.max(dp[w] + remaining_budget / min_price);
        }

        ans
    }
}
