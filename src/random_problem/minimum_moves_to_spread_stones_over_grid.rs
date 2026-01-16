struct Solution;

impl Solution {
    pub fn minimum_moves(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut zero_count = 0;

        for row in &grid {
            for &cell in row {
                if cell == 0 {
                    zero_count += 1;
                }
            }
        }

        if zero_count == 0 {
            return 0;
        }

        let mut ans = i32::MAX;

        for i in 0..3 {
            for j in 0..3 {
                if grid[i][j] == 0 {
                    for ni in 0..3 {
                        for nj in 0..3 {
                            let d = ((ni - i) as i32).abs() + ((nj - j) as i32).abs();
                            if grid[ni][nj] > 1 {
                                grid[ni][nj] -= 1;
                                grid[i][j] += 1;
                                ans = ans.min(d + Self::minimum_moves(grid.clone()));
                                grid[ni][nj] += 1;
                                grid[i][j] -= 1;
                            }
                        }
                    }
                }
            }
        }
        ans
    }
}
