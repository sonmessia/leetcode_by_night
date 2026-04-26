struct Solution;

impl Solution {
    fn dfs(
        i: usize,
        j: usize,
        parent: Option<(usize, usize)>,
        grid: &Vec<Vec<char>>,
        visited: &mut Vec<Vec<bool>>,
    ) -> bool {
        visited[i][j] = true;

        let directions = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        let m = grid.len() as isize;
        let n = grid[0].len() as isize;

        for (di, dj) in directions {
            let ni = i as isize + di;
            let nj = j as isize + dj;

            if ni >= 0 && ni < m && nj >= 0 && nj < n {
                let (ni_u, nj_u) = (ni as usize, nj as usize);

                if grid[ni_u][nj_u] == grid[i][j] {
                    if visited[ni_u][nj_u] {
                        if Some((ni_u, nj_u)) != parent {
                            return true;
                        }
                    } else {
                        if Self::dfs(ni_u, nj_u, Some((i, j)), grid, visited) {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }

    pub fn contains_cycle(grid: Vec<Vec<char>>) -> bool {
        if grid.is_empty() || grid[0].is_empty() {
            return false;
        }
        let (m, n) = (grid.len(), grid[0].len());
        let mut visited = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                if !visited[i][j] {
                    if Self::dfs(i, j, None, &grid, &mut visited) {
                        return true;
                    }
                }
            }
        }
        false
    }
}
