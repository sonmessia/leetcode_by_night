struct Solution;

impl Solution {
    fn dfs(i: usize, j: usize, count: i32, m: usize, n: usize, grid: &Vec<Vec<char>>) -> bool {
        if i == m - 1 && j == n - 1 {
            return count == 1;
        }

        if grid[i][j] == '(' {
            if i + 1 < m && Self::dfs(i + 1, j, count + 1, m, n, &grid) {
                return true;
            }
            if j + 1 < n && Self::dfs(i, j + 1, count + 1, m, n, &grid) {
                return true;
            }
        } else {
            if count > 0 {
                if i + 1 < m && Self::dfs(i + 1, j, count - 1, m, n, &grid) {
                    return true;
                }
                if j + 1 < n && Self::dfs(i, j + 1, count - 1, m, n, &grid) {
                    return true;
                }
            }
        }
        false
    }

    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());

        if grid[0][0] != '(' || grid[m - 1][n - 1] != ')' {
            return false;
        }

        Self::dfs(0, 0, 0, m, n, &grid)
    }
}
