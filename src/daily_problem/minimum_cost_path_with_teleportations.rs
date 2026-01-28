struct Solution;

impl Solution {
    pub fn min_cost(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());
        let mut points = vec![];

        for i in 0..n {
            for j in 0..m {
                points.push((i, j));
            }
        }

        points.sort_by_key(|&(i, j)| grid[i][j]);

        let mut dp = vec![vec![i32::MAX; m]; n];

        for _ in 0..=k {
            let mut min_cost = i32::MAX;
            let (mut i, mut j) = (0, 0);

            while i < points.len() {
                min_cost = min_cost.min(dp[points[i].0][points[i].1]);
                if i + 1 < points.len()
                    && grid[points[i].0][points[i].1] == grid[points[i + 1].0][points[i + 1].1]
                {
                    i += 1;
                    continue;
                }

                for l in j..=i {
                    let p = points[l];
                    dp[p.0][p.1] = min_cost;
                }
                j = i + 1;
                i += 1;
            }

            for i in (0..n).rev() {
                for j in (0..m).rev() {
                    if i == n - 1 && j == m - 1 {
                        dp[i][j] = 0;
                        continue;
                    }
                    if i != n - 1 {
                        dp[i][j] = dp[i][j].min(dp[i + 1][j] + grid[i + 1][j]);
                    }
                    if j != m - 1 {
                        dp[i][j] = dp[i][j].min(dp[i][j + 1] + grid[i][j + 1]);
                    }
                }
            }
        }
        dp[0][0]
    }
}
