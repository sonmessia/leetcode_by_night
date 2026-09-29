struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let max_balance = m + n;
        let mut visited = vec![vec![vec![false; max_balance + 1]; n]; m];
        let mut stack = vec![(0, 0, 0)];

        while let Some((x, y, balance)) = stack.pop() {
            let new_balance = match grid[x][y] {
                '(' => balance + 1,
                ')' => {
                    if balance > 0 {
                        balance - 1
                    } else {
                        continue;
                    }
                }
                _ => balance,
            };

            if new_balance > max_balance {
                continue;
            }

            if visited[x][y][new_balance] {
                continue;
            }

            visited[x][y][new_balance] = true;

            if x == m - 1 && y == n - 1 && new_balance == 0 {
                return true;
            }

            if x + 1 < m {
                stack.push((x + 1, y, new_balance));
            }

            if y + 1 < n {
                stack.push((x, y + 1, new_balance));
            }
        }

        false
    }
}
