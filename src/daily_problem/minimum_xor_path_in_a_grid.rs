struct Solution;

impl Solution {
    pub fn min_path_xor(grid: Vec<Vec<i32>>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let mut dp = vec![vec![false; 1 << 16]; n];
        let mut next_dp = vec![false; 1 << 16];

        dp[0][grid[0][0] as usize] = true;

        for i in 0..m {
            for j in 0..n {
                if i == 0 && j == 0 {
                    continue;
                }
                next_dp.fill(false);
                for k in 0..(1 << 16) {
                    if dp[j][k] || (j > 0 && dp[j - 1][k]) {
                        let new_xor = k ^ grid[i][j] as usize;
                        next_dp[new_xor] = true;
                    }
                }
                dp[j] = next_dp.clone();
            }
        }

        dp[n - 1]
            .iter()
            .enumerate()
            .find(|&(_, &v)| v)
            .map(|(i, _)| i as i32)
            .unwrap_or(-1)
    }
}
