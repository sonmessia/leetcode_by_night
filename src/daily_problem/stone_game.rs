struct Solution;

impl Solution {
    pub fn stone_game(piles: Vec<i32>) -> bool {
        let n = piles.len();
        let mut dp = vec![vec![None; n]; n];

        fn dfs(piles: &Vec<i32>, l: usize, r: usize, dp: &mut Vec<Vec<Option<i32>>>) -> i32 {
            if l == r {
                return piles[l];
            }

            if let Some(ans) = dp[l][r] {
                return ans;
            }

            let take_left = piles[l] - dfs(piles, l + 1, r, dp);
            let take_right = piles[r] - dfs(piles, l, r - 1, dp);

            let ans = take_left.max(take_right);
            dp[l][r] = Some(ans);
            ans
        }

        dfs(&piles, 0, n - 1, &mut dp) > 0
    }
}
