struct Solution;

impl Solution {
    pub fn stone_game_iii(stone_value: Vec<i32>) -> String {
        let n = stone_value.len();
        let mut dp = vec![None; n];

        fn dfs(stone_value: &Vec<i32>, i: usize, n: usize, dp: &mut Vec<Option<i32>>) -> i32 {
            if i >= n {
                return 0;
            }

            if let Some(ans) = dp[i] {
                return ans;
            }

            let mut best = i32::MIN;
            let mut sum = 0;

            for k in 0..3 {
                if i + k >= n {
                    break;
                }
                sum += stone_value[i + k];
                best = best.max(sum - dfs(stone_value, i + k + 1, n, dp));
            }

            dp[i] = Some(best);
            best
        }

        match dfs(&stone_value, 0, n, &mut dp) {
            x if x > 0 => "Alice".to_string(),
            x if x < 0 => "Bob".to_string(),
            _ => "Tie".to_string(),
        }
    }
}
