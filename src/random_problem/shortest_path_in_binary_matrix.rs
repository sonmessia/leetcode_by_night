struct Solution;

impl Solution {
    pub fn shortest_path_binary_matrix(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        if grid[0][0] == 1 || grid[n - 1][n - 1] == 1 {
            return -1;
        }

        let directions = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0, 1)); // (row, col, distance)
        let mut visited = vec![vec![false; n]; n];
        visited[0][0] = true;

        while let Some((row, col, dist)) = queue.pop_front() {
            if row == n - 1 && col == n - 1 {
                return dist;
            }

            for &(dr, dc) in &directions {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;

                if new_row >= 0 && new_row < n as isize && new_col >= 0 && new_col < n as isize {
                    let (new_row, new_col) = (new_row as usize, new_col as usize);
                    if grid[new_row][new_col] == 0 && !visited[new_row][new_col] {
                        visited[new_row][new_col] = true;
                        queue.push_back((new_row, new_col, dist + 1));
                    }
                }
            }
        }

        -1
    }
}
