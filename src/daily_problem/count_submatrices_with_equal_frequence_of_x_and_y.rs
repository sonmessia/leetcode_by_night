struct Solution;

impl Solution {
    pub fn number_of_submatrices(grid: Vec<Vec<char>>) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());

        let mut cnt_x = vec![vec![0; n + 1]; m + 1];
        let mut cnt_y = vec![vec![0; n + 1]; m + 1];

        let mut ans = 0;

        for i in 1..=m {
            for j in 1..=n {
                let (add_x, add_y) = match grid[i - 1][j - 1] {
                    'X' => (1, 0),
                    'Y' => (0, 1),
                    _ => (0, 0),
                };

                cnt_x[i][j] = cnt_x[i - 1][j] + cnt_x[i][j - 1] - cnt_x[i - 1][j - 1] + add_x;
                cnt_y[i][j] = cnt_y[i - 1][j] + cnt_y[i][j - 1] - cnt_y[i - 1][j - 1] + add_y;

                if cnt_x[i][j] == cnt_y[i][j] && cnt_x[i][j] > 0 {
                    ans += 1;
                }
            }
        }
        ans
    }
}
