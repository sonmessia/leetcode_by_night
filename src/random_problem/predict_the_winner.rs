struct Solution;

impl Solution {
    fn dfs(
        nums: &Vec<i32>,
        left: usize,
        right: usize,
        turn: i32,
        score: &mut i32,
        is_player1_win: &mut bool,
    ) {
        if left == right {
            *score += turn * nums[left];
            if *score >= 0 {
                *is_player1_win = true;
            }
            *score -= turn * nums[left];
            return;
        }
        // Choose the leftmost number
        *score += turn * nums[left];
        Self::dfs(nums, left + 1, right, -turn, score, is_player1_win);
        *score -= turn * nums[left];
        // Choose the rightmost number
        *score += turn * nums[right];
        Self::dfs(nums, left, right - 1, -turn, score, is_player1_win);
        *score -= turn * nums[right];
    }

    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        // let n = nums.len();
        // let mut dp = vec![vec![0; n]; n];
        // for i in 0..n {
        //     dp[i][i] = nums[i];
        // }
        // for length in 1..n {
        //     for i in 0..n - length {
        //         let j = i + length;
        //         dp[i][j] = (nums[i] - dp[i + 1][j]).max(nums[j] - dp[i][j - 1]);
        //     }
        // }
        // dp[0][n - 1] >= 0
        let mut score = 0;
        let mut is_player1_win = false;
        Self::dfs(&nums, 0, nums.len() - 1, 1, &mut score, &mut is_player1_win);
        is_player1_win
    }
}

fn main() {
    let nums = vec![1, 5, 2];
    let result = Solution::predict_the_winner(nums);
    println!("Result: {}", result); // Output: false
}
