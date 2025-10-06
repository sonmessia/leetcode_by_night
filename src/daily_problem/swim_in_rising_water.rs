struct Solution;

impl Solution {
    pub fn swim_in_water(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;
        let n = grid.len();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
        let mut visited = vec![vec![false; n]; n];
        let mut heap = BinaryHeap::new();
        heap.push(Reverse((grid[0][0], 0, 0)));
        visited[0][0] = true;
        let mut result = 0;
        while let Some(Reverse((height, x, y))) = heap.pop() {
            result = result.max(height);
            if x == n - 1 && y == n - 1 {
                return result;
            }
            for (dx, dy) in &directions {
                let new_x = x as i32 + dx;
                let new_y = y as i32 + dy;
                if new_x >= 0 && new_x < n as i32 && new_y >= 0 && new_y < n as i32 {
                    let (new_x, new_y) = (new_x as usize, new_y as usize);
                    if !visited[new_x][new_y] {
                        visited[new_x][new_y] = true;
                        heap.push(Reverse((grid[new_x][new_y], new_x, new_y)));
                    }
                }
            }
        }
        result
    }
}
