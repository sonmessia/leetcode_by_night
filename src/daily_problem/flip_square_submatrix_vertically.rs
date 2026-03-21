struct Solution;

impl Solution {
    pub fn flip_vertically(mut grid: Vec<Vec<i32>>, x: i32, y: i32, k: i32) -> Vec<Vec<i32>> {
        let (x_usize, y_usize, k_usize) = (x as usize, y as usize, k as usize);

        for i in x_usize..x_usize + k_usize / 2 {
            for j in y_usize..y_usize + k_usize {
                let tmp = grid[i][j];
                grid[i][j] = grid[x_usize + k_usize - 1 - (i - x_usize)][j];
                grid[x_usize + k_usize - 1 - (i - x_usize)][j] = tmp;
            }
        }

        grid.clone()
    }
}
