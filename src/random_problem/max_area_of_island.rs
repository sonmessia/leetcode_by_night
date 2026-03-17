struct Solution;

impl Solution {
    pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
        let mut max_area = 0;
        let mut visited = vec![vec![false; grid[0].len()]; grid.len()];

        let directions: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        fn dfs(
            grid: &Vec<Vec<i32>>,
            visited: &mut Vec<Vec<bool>>,
            directions: [(i32, i32); 4],
            x: usize,
            y: usize,
        ) -> i32 {
            let mut stack = vec![(x, y)];
            let mut area = 0;

            while let Some((cx, cy)) = stack.pop() {
                if cx >= grid.len() || cy >= grid[0].len() || visited[cx][cy] || grid[cx][cy] == 0 {
                    continue;
                }

                visited[cx][cy] = true;
                area += 1;

                for &(dx, dy) in &directions {
                    let nx = (cx as i32 + dx) as usize;
                    let ny = (cy as i32 + dy) as usize;

                    if nx < grid.len()
                        && ny < grid[0].len()
                        && !visited[nx][ny]
                        && grid[nx][ny] == 1
                    {
                        stack.push((nx, ny));
                    }
                }
            }
            area
        }

        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == 1 && !visited[i][j] {
                    max_area = max_area.max(dfs(&grid, &mut visited, directions, i, j));
                }
            }
        }

        max_area
    }
}
