struct Solution;

impl Solution {
    pub fn update_matrix(matrix: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let inf = m as i32 + n as i32;
        let mut dist = vec![vec![inf; n]; m];
        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    dist[i][j] = 0;
                } else {
                    if i > 0 {
                        dist[i][j] = dist[i][j].min(dist[i - 1][j] + 1);
                    }
                    if j > 0 {
                        dist[i][j] = dist[i][j].min(dist[i][j - 1] + 1);
                    }
                }
            }
        }

        println!("{:?}", dist);

        for i in (0..m).rev() {
            for j in (0..n).rev() {
                if i < m - 1 {
                    dist[i][j] = dist[i][j].min(dist[i + 1][j] + 1);
                }
                if j < n - 1 {
                    dist[i][j] = dist[i][j].min(dist[i][j + 1] + 1);
                }
            }
        }

        println!("{:?}", dist);

        dist
    }
}
