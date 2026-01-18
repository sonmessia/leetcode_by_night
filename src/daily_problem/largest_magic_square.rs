struct Solution;

impl Solution {
    pub fn largest_magic_square(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut row_prefix = vec![vec![0; m + 1]; n];
        let mut col_prefix = vec![vec![0; m]; n + 1];

        for i in 0..n {
            for j in 0..m {
                row_prefix[i][j + 1] = row_prefix[i][j] + grid[i][j];
                col_prefix[i + 1][j] = col_prefix[i][j] + grid[i][j];
            }
        }

        let max_k = n.min(m);

        for k in (2..=max_k).rev() {
            for r in 0..=n - k {
                for c in 0..=m - k {
                    let target_sum = row_prefix[r][c + k] - row_prefix[r][c];

                    let mut is_magic = true;

                    for i in r..r + k {
                        let row_sum = row_prefix[i][c + k] - row_prefix[i][c];
                        if row_sum != target_sum {
                            is_magic = false;
                            break;
                        }
                    }

                    if !is_magic {
                        continue;
                    }

                    for j in c..c + k {
                        let col_sum = col_prefix[r + k][j] - col_prefix[r][j];
                        if col_sum != target_sum {
                            is_magic = false;
                            break;
                        }
                    }

                    if !is_magic {
                        continue;
                    }

                    let mut diag1_sum = 0;
                    let mut diag2_sum = 0;

                    for i in 0..k {
                        diag1_sum += grid[r + i][c + i];
                        diag2_sum += grid[r + i][c + k - 1 - i];
                    }

                    if diag1_sum == target_sum && diag2_sum == target_sum {
                        return k as i32;
                    }
                }
            }
        }

        1
    }
}
