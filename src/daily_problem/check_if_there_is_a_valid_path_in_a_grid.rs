use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<i32>>) -> bool {
        let m = grid.len();
        let n = grid[0].len();
        let mut visited = vec![vec![false; n]; m];
        let mut queue = VecDeque::from([(0, 0)]);
        visited[0][0] = true;

        let get_dirs = |pipe_type: i32| -> &[(i32, i32)] {
            match pipe_type {
                1 => &[(0, -1), (0, 1)],
                2 => &[(-1, 0), (1, 0)],
                3 => &[(0, -1), (1, 0)],
                4 => &[(0, 1), (1, 0)],
                5 => &[(0, -1), (-1, 0)],
                6 => &[(0, 1), (-1, 0)],
                _ => &[],
            }
        };

        while let Some((x, y)) = queue.pop_front() {
            if x == m - 1 && y == n - 1 {
                return true;
            }

            let curr_pipe = grid[x][y];
            for &(dx, dy) in get_dirs(curr_pipe) {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx_u, ny_u) = (nx as usize, ny as usize);

                    if !visited[nx_u][ny_u] {
                        let neighbor_pipe = grid[nx_u][ny_u];
                        let can_connect = get_dirs(neighbor_pipe)
                            .iter()
                            .any(|&(ndx, ndy)| ndx == -dx && ndy == -dy);

                        if can_connect {
                            visited[nx_u][ny_u] = true;
                            queue.push_back((nx_u, ny_u));
                        }
                    }
                }
            }
        }

        false
    }
}
