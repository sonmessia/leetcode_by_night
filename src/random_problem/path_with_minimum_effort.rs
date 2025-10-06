struct Solution;

impl Solution {
    pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let (m, n) = (heights.len(), heights[0].len());
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut dist = vec![vec![i32::MAX; n]; m];
        let mut heap = BinaryHeap::new();
        dist[0][0] = 0;
        heap.push(Reverse((0, 0, 0)));

        while let Some(Reverse((effort, x, y))) = heap.pop() {
            if x == m - 1 && y == n - 1 {
                return effort;
            }
            if effort > dist[x][y] {
                continue;
            }
            for (dx, dy) in &directions {
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < m as i32 && ny >= 0 && ny < n as i32 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    let next_effort = (heights[nx][ny] - heights[x][y]).abs().max(effort);
                    if next_effort < dist[nx][ny] {
                        dist[nx][ny] = next_effort;
                        heap.push(Reverse((next_effort, nx, ny)));
                    }
                }
            }
        }
        0
    }
}

fn main() {
    let heights = vec![vec![1, 10, 6, 7, 9, 10, 4, 9]];
    let result = Solution::minimum_effort_path(heights);
    println!("Minimum Effort Path: {}", result); // Output: 9

    let heights = vec![
        vec![4, 3, 4, 10, 5, 5, 9, 2],
        vec![10, 8, 2, 10, 9, 7, 5, 6],
        vec![5, 8, 10, 10, 10, 7, 4, 2],
        vec![5, 1, 3, 1, 1, 3, 1, 9],
        vec![6, 4, 10, 6, 10, 9, 4, 6],
    ];
    let result = Solution::minimum_effort_path(heights);
    println!("Minimum Effort Path: {}", result); // Output: 5
}
