struct Solution;

impl Solution {
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        if grid[0][0] == 1 || grid[grid.len() - 1][grid[0].len() - 1] == 1 {
            return 0;
        }
    }
}
