struct Solution;

impl Solution {
    pub fn largest_submatrix(matrix: Vec<Vec<i32>>) -> i32 {
        let (m, n) = (matrix.len(), matrix[0].len());
        let mut count = vec![0; n];

        let mut ans = 0;

        for i in 0..m {
            for j in 0..n {
                if matrix[i][j] == 0 {
                    count[j] = 0;
                } else {
                    count[j] += 1;
                }
            }

            let tnp_count = {
                let mut tnp_count = count.clone();
                tnp_count.sort_unstable_by(|a, b| b.cmp(a));
                tnp_count
            };

            for j in 0..n {
                ans = ans.max(tnp_count[j] * (j as i32 + 1));
            }
        }
        ans
    }
}
