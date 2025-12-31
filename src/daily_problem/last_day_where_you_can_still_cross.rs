struct Solution;

impl Solution {
    pub fn latest_day_to_cross(row: i32, col: i32, cells: Vec<Vec<i32>>) -> i32 {
        fn can_cross(row: i32, col: i32, cells: &Vec<Vec<i32>>, day: i32) -> bool {
            let mut grid = vec![vec![0; col as usize]; row as usize];
            for i in 0..day as usize {
                let r = cells[i][0] - 1;
                let c = cells[i][1] - 1;
                grid[r as usize][c as usize] = 1;
            }

            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            let mut visited = vec![vec![false; col as usize]; row as usize];
            let mut stack = vec![];

            for c in 0..col {
                if grid[0][c as usize] == 0 {
                    stack.push((0, c));
                    visited[0][c as usize] = true;
                }
            }

            while let Some((r, c)) = stack.pop() {
                if r == row - 1 {
                    return true;
                }
                for (dr, dc) in &directions {
                    let new_r = r + dr;
                    let new_c = c + dc;
                    if new_r >= 0
                        && new_r < row
                        && new_c >= 0
                        && new_c < col
                        && grid[new_r as usize][new_c as usize] == 0
                        && !visited[new_r as usize][new_c as usize]
                    {
                        visited[new_r as usize][new_c as usize] = true;
                        stack.push((new_r, new_c));
                    }
                }
            }

            false
        }

        let mut left = 1;
        let mut right = cells.len() as i32;
        let mut result = 0;

        while left <= right {
            let mid = left + (right - left) / 2;
            if can_cross(row, col, &cells, mid) {
                result = mid;
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        result
    }
}
