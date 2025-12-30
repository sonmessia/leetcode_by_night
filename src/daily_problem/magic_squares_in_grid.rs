struct Solution;

impl Solution {
    pub fn num_magic_squares_inside(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        if n < 3 || m < 3 {
            return 0;
        }

        let mut ans = 0;

        for i in 1..n - 1 {
            for j in 1..m - 1 {
                if grid[i][j] != 5 {
                    continue;
                }

                let mut seen = vec![false; 10];
                let mut is_magic = true;

                for x in i - 1..=i + 1 {
                    for y in j - 1..=j + 1 {
                        let val = grid[x][y];
                        if val < 1 || val > 9 || seen[val as usize] {
                            is_magic = false;
                            break;
                        }
                        seen[val as usize] = true;
                    }
                    if !is_magic {
                        break;
                    }
                }

                if !is_magic {
                    continue;
                }

                let row_sums = [
                    grid[i - 1][j - 1] + grid[i - 1][j] + grid[i - 1][j + 1],
                    grid[i][j - 1] + grid[i][j] + grid[i][j + 1],
                    grid[i + 1][j - 1] + grid[i + 1][j] + grid[i + 1][j + 1],
                ];

                let col_sums = [
                    grid[i - 1][j - 1] + grid[i][j - 1] + grid[i + 1][j - 1],
                    grid[i - 1][j] + grid[i][j] + grid[i + 1][j],
                    grid[i - 1][j + 1] + grid[i][j + 1] + grid[i + 1][j + 1],
                ];

                let diag_sums = [
                    grid[i - 1][j - 1] + grid[i][j] + grid[i + 1][j + 1],
                    grid[i - 1][j + 1] + grid[i][j] + grid[i + 1][j - 1],
                ];

                if row_sums.iter().all(|&x| x == 15)
                    && col_sums.iter().all(|&x| x == 15)
                    && diag_sums.iter().all(|&x| x == 15)
                {
                    ans += 1;
                }
            }
        }

        ans
    }
}
