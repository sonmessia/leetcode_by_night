struct Solution;

impl Solution {
    pub fn count_unguarded(m: i32, n: i32, guards: Vec<Vec<i32>>, walls: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (m as usize, n as usize);
        let mut grid = vec![vec![0; n]; m];
        for guard in &guards {
            grid[guard[0] as usize][guard[1] as usize] = 1;
        }
        for wall in &walls {
            grid[wall[0] as usize][wall[1] as usize] = 2;
        }

        for guard in &guards {
            let (x, y) = (guard[0] as usize, guard[1] as usize);
            for i in (0..x).rev() {
                if grid[i][y] == 2 || grid[i][y] == 1 {
                    break;
                }
                grid[i][y] = -1;
            }
            for i in x + 1..m {
                if grid[i][y] == 2 || grid[i][y] == 1 {
                    break;
                }
                grid[i][y] = -1;
            }
            for j in (0..y).rev() {
                if grid[x][j] == 2 || grid[x][j] == 1 {
                    break;
                }
                grid[x][j] = -1;
            }
            for j in y + 1..n {
                if grid[x][j] == 2 || grid[x][j] == 1 {
                    break;
                }
                grid[x][j] = -1;
            }
        }

        let mut count = 0;
        for i in 0..m {
            for j in 0..n {
                if grid[i][j] == 0 {
                    count += 1;
                }
            }
        }
        count
    }
}
