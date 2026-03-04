struct Solution;

impl Solution {
    pub fn num_special(mat: Vec<Vec<i32>>) -> i32 {
        let mut row_sum = vec![0; mat.len()];
        let mut col_sum = vec![0; mat[0].len()];

        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                row_sum[i] += mat[i][j];
                col_sum[j] += mat[i][j];
            }
        }

        let mut count = 0;
        for i in 0..mat.len() {
            for j in 0..mat[0].len() {
                if mat[i][j] == 1 && row_sum[i] == 1 && col_sum[j] == 1 {
                    count += 1;
                }
            }
        }

        count
    }
}
