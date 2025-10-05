struct Solution;

impl Solution {
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        fn dfs(row: usize, col: usize, visited: &mut Vec<Vec<bool>>, heights: &Vec<Vec<i32>>) {
            visited[row][col] = true;
            let directions = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
            for (dr, dc) in directions {
                let new_row = row as isize + dr;
                let new_col = col as isize + dc;
                if new_row >= 0
                    && new_row < heights.len() as isize
                    && new_col >= 0
                    && new_col < heights[0].len() as isize
                    && !visited[new_row as usize][new_col as usize]
                    && heights[new_row as usize][new_col as usize] >= heights[row][col]
                {
                    dfs(new_row as usize, new_col as usize, visited, heights);
                }
            }
        }

        let (m, n) = (heights.len(), heights[0].len());

        let mut pacific_visited = vec![vec![false; n]; m];
        let mut atlantic_visited = vec![vec![false; n]; m];

        for i in 0..m {
            dfs(i, 0, &mut pacific_visited, &heights);
            dfs(i, n - 1, &mut atlantic_visited, &heights);
        }

        for j in 0..n {
            dfs(0, j, &mut pacific_visited, &heights);
            dfs(m - 1, j, &mut atlantic_visited, &heights);
        }

        println!("{:?}", pacific_visited);
        println!("{:?}", atlantic_visited);

        let mut ans = vec![];

        for i in 0..m {
            for j in 0..n {
                if pacific_visited[i][j] && atlantic_visited[i][j] {
                    ans.push(vec![i as i32, j as i32]);
                }
            }
        }
        ans
    }
}
