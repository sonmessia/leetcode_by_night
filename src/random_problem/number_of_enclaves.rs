struct Solution;

impl Solution {
    pub fn num_enclaves(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let m = grid.len();
        let n = grid[0].len();

        for i in 0..m {
            for j in 0..n {
                if (i == 0 || i == m - 1 || j == 0 || j == n - 1) && grid[i][j] == 1 {
                    Self::dfs(&mut grid, i as i32, j as i32);
                }
            }
        }

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 1 {
                    count += 1;
                }
            }
        }

        count
    }

    fn dfs(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) {
        let m = grid.len() as i32;
        let n = grid[0].len() as i32;

        if i < 0 || i >= m || j < 0 || j >= n || grid[i as usize][j as usize] == 0 {
            return;
        }

        grid[i as usize][j as usize] = 0;

        Self::dfs(grid, i + 1, j);
        Self::dfs(grid, i - 1, j);
        Self::dfs(grid, i, j + 1);
        Self::dfs(grid, i, j - 1);
    }
}
