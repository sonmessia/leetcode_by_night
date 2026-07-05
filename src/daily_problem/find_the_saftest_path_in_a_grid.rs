struct Solution;

impl Solution {
    pub fn can_reach(dist: &Vec<Vec<i32>>, safeness_factor: i32) -> bool {
        let n = dist.len();
        let m = dist[0].len();
        let mut visited = vec![vec![false; m]; n];
        let mut queue = std::collections::VecDeque::new();

        if dist[0][0] < safeness_factor {
            return false;
        }

        queue.push_back((0, 0));
        visited[0][0] = true;

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((x, y)) = queue.pop_front() {
            if x == n - 1 && y == m - 1 {
                return true;
            }
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if !visited[nx][ny] && dist[nx][ny] >= safeness_factor {
                        visited[nx][ny] = true;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        false
    }
    pub fn maximum_safeness_factor(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        let mut dist = vec![vec![i32::MAX; m]; n];
        let mut queue = std::collections::VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if grid[i][j] == 1 {
                    dist[i][j] = 0;
                    queue.push_back((i, j));
                }
            }
        }

        let directions = vec![(1, 0), (-1, 0), (0, 1), (0, -1)];

        while let Some((x, y)) = queue.pop_front() {
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < n as i32 && ny >= 0 && ny < m as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;
                    if dist[nx][ny] > dist[x][y] + 1 {
                        dist[nx][ny] = dist[x][y] + 1;
                        queue.push_back((nx, ny));
                    }
                }
            }
        }

        let mut left = 0;
        let mut right = *dist.iter().flatten().max().unwrap();
        let mut result = -1;

        while left <= right {
            let mid = left + (right - left) / 2;
            if Self::can_reach(&dist, mid) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        result
    }
}
