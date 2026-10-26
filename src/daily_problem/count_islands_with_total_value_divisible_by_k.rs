struct Solution;

impl Solution {
    fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, i: i32, j: i32) -> i64 {
        if i < 0
            || i >= grid.len() as i32
            || j < 0
            || j >= grid[0].len() as i32
            || grid[i as usize][j as usize] == 0
            || visited[i as usize][j as usize]
        {
            return 0;
        }

        visited[i as usize][j as usize] = true;
        let mut total_value = grid[i as usize][j as usize] as i64;

        total_value += Self::dfs(grid, visited, i + 1, j);
        total_value += Self::dfs(grid, visited, i - 1, j);
        total_value += Self::dfs(grid, visited, i, j + 1);
        total_value += Self::dfs(grid, visited, i, j - 1);

        total_value as i64
    }
    pub fn count_islands(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];
        let mut count = 0;
        let k = k as i64;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] != 0 && !visited[i][j] {
                    let total_value = Self::dfs(&grid, &mut visited, i as i32, j as i32);
                    if total_value % k == 0 && total_value != 0 {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}
