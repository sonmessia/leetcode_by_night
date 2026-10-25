struct Solution;

impl Solution {
    pub fn closed_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut grid = grid;
        let mut count = 0;

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 0 && Self::is_closed(&mut grid, i as i32, j as i32) {
                    count += 1;
                }
            }
        }

        count
    }

    fn is_closed(grid: &mut Vec<Vec<i32>>, i: i32, j: i32) -> bool {
        if i < 0 || j < 0 || i >= grid.len() as i32 || j >= grid[0].len() as i32 {
            return false;
        }

        if grid[i as usize][j as usize] == 1 {
            return true;
        }

        grid[i as usize][j as usize] = 1;

        let up = Self::is_closed(grid, i - 1, j);
        let down = Self::is_closed(grid, i + 1, j);
        let left = Self::is_closed(grid, i, j - 1);
        let right = Self::is_closed(grid, i, j + 1);

        up && down && left && right
    }
}
