struct Solution;

impl Solution {
    pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
        let mut row_count = vec![0; grid.len()];
        let mut col_count = vec![0; grid[0].len()];

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 {
                    row_count[i] += 1;
                    col_count[j] += 1;
                }
            }
        }

        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && (row_count[i] > 1 || col_count[j] > 1) {
                    count += 1;
                }
            }
        }

        count
    }
}
