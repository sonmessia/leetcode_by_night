struct Solution;

impl Solution {
    pub fn ones_minus_zeros(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let mut row_sum = vec![0; m];
        let mut col_sum = vec![0; n];
        for i in 0..m {
            for j in 0..n {
                row_sum[i] += grid[i][j];
                col_sum[j] += grid[i][j];
            }
        }
        let mut ans = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                ans[i][j] =
                    row_sum[i] + col_sum[j] - (m as i32 - row_sum[i]) - (n as i32 - col_sum[j]);
            }
        }
        ans
    }
}
