struct Solution;

impl Solution {
    pub fn get_biggest_three(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        let mut d1 = vec![vec![0; n + 2]; m + 2];
        let mut d2 = vec![vec![0; n + 2]; m + 2];

        for i in 0..m {
            for j in 0..n {
                d1[i + 1][j + 1] = d1[i][j] + grid[i][j];

                d2[i + 1][j + 1] = d2[i][j + 2] + grid[i][j];
            }
        }

        let mut top3: Vec<i32> = Vec::with_capacity(4);

        let mut add_to_top3 = |val: i32| {
            if !top3.contains(&val) {
                top3.push(val);
                top3.sort_unstable_by(|a, b| b.cmp(a));
                if top3.len() > 3 {
                    top3.pop();
                }
            }
        };

        for i in 0..m {
            for j in 0..n {
                add_to_top3(grid[i][j]);

                let mut k = 1;
                loop {
                    if i + 2 * k >= m || j + k >= n || j < k {
                        break;
                    }

                    // Top -> Right
                    let edge1 = d1[i + k + 1][j + k + 1] - d1[i][j];

                    // Right -> Bottom
                    let edge2 = d2[i + 2 * k + 1][j + 1] - d2[i + k][j + k + 2];

                    //Bottom -> Left
                    let edge3 = d1[i + 2 * k + 1][j + 1] - d1[i + k][j - k];

                    //Left -> Top
                    let edge4 = d2[i + k + 1][j - k + 1] - d2[i][j + 2];

                    let corners =
                        grid[i][j] + grid[i + k][j + k] + grid[i + 2 * k][j] + grid[i + k][j - k];

                    let sum = edge1 + edge2 + edge3 + edge4 - corners;

                    add_to_top3(sum);
                    k += 1;
                }
            }
        }

        top3
    }
}
