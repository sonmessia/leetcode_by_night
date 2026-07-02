use std::collections::VecDeque;

struct Solution;

impl Solution {
    pub fn find_safe_walk(grid: Vec<Vec<i32>>, health: i32) -> bool {
        let (m, n) = (grid.len(), grid[0].len());

        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        let mut distance = vec![vec![i32::MAX; n]; m];
        let mut queue = VecDeque::new();
        queue.push_back((0, 0));
        distance[0][0] = grid[0][0];

        while let Some((x, y)) = queue.pop_front() {
            if x == m - 1 && y == n - 1 {
                return true;
            }

            for &dir in &directions {
                let (new_x, new_y) = (x as i32 + dir.0, y as i32 + dir.1);
                if new_x >= 0 && new_x < m as i32 && new_y >= 0 && new_y < n as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    let new_health = distance[x][y] + grid[new_x][new_y];

                    if new_health >= health {
                        continue;
                    }

                    if new_health < distance[new_x][new_y] {
                        distance[new_x][new_y] = new_health;
                        if grid[new_x][new_y] == 0 {
                            queue.push_back((new_x, new_y));
                        } else {
                            queue.push_front((new_x, new_y));
                        }
                    }
                }
            }
        }
        false
    }
}
