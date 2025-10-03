use std::cmp::Reverse;
use std::collections::BinaryHeap;

struct Solution;

impl Solution {
    pub fn trap_rain_water(height_map: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (height_map.len(), height_map[0].len());
        let mut ans = 0;
        let mut visited = vec![vec![false; n]; m];

        let mut min_heap = BinaryHeap::new();

        for i in 0..m {
            visited[i][0] = true;
            visited[i][n - 1] = true;
            min_heap.push(Reverse((height_map[i][0], i, 0)));
            min_heap.push(Reverse((height_map[i][n - 1], i, n - 1)));
        }

        for j in 0..n {
            if !visited[0][j] {
                visited[0][j] = true;
                min_heap.push(Reverse((height_map[0][j], 0, j)));
            }
            if !visited[m - 1][j] {
                visited[m - 1][j] = true;
                min_heap.push(Reverse((height_map[m - 1][j], m - 1, j)));
            }
        }
        println!("{:?}", visited);

        let directions = [-1, 0, 1, 0, -1];
        while let Some(Reverse((height, x, y))) = min_heap.pop() {
            println!("Processing cell ({}, {}) with height {}", x, y, height);

            for k in 0..4 {
                let (nx, ny) = (x as i32 + directions[k], y as i32 + directions[k + 1]);
                if nx >= 0
                    && nx < m as i32
                    && ny >= 0
                    && ny < n as i32
                    && !visited[nx as usize][ny as usize]
                {
                    visited[nx as usize][ny as usize] = true;
                    ans += (height - height_map[nx as usize][ny as usize]).max(0);
                    min_heap.push(Reverse((
                        height.max(height_map[nx as usize][ny as usize]),
                        nx as usize,
                        ny as usize,
                    )));
                }
            }
        }
        ans
    }
}

fn main() {
    let height_map = vec![
        vec![3, 3, 3, 3, 3],
        vec![3, 2, 2, 2, 3],
        vec![3, 2, 1, 2, 3],
        vec![3, 2, 2, 2, 3],
        vec![3, 3, 3, 3, 3],
    ];
    let result = Solution::trap_rain_water(height_map);
    println!("Trapped rain water: {}", result);

    let height_map2 = vec![
        vec![1, 4, 3, 1, 3, 2],
        vec![3, 2, 1, 3, 2, 4],
        vec![2, 3, 3, 2, 3, 1],
    ];
    let result2 = Solution::trap_rain_water(height_map2);
    println!("Trapped rain water: {}", result2);
}
