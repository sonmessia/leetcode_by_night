struct Solution;

impl Solution {
    pub fn rotate_the_box(mut box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        let m = box_grid.len();
        let n = box_grid[0].len();

        for i in 0..m {
            let mut next = n as i32 - 1;
            for j in (0..n).rev() {
                if box_grid[i][j] == '*' {
                    next = j as i32 - 1;
                } else if box_grid[i][j] == '#' {
                    let k = next as usize;
                    box_grid[i].swap(j,k);
                    next -= 1;
                }
            }
        }

        let mut rotated = vec![vec![' '; m]; n];
        for i in 0..m {
            for j in 0..n {
                rotated[j][m - 1 - i] = box_grid[i][j];
            }
        }
        rotated
    }
}
