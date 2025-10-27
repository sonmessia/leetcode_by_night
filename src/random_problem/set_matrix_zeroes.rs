struct Solution;

impl Solution {
    pub fn set_zeroes(matrix: &mut Vec<Vec<i32>>) {
        let (n, m) = (matrix.len(), matrix[0].len());
        let (mut row, mut col) = (vec![0; n], vec![0; m]);

        for i in 0..n {
            for j in 0..m {
                if matrix[i][j] == 0 {
                    row[i] = 1;
                    col[j] = 1;
                }
            }
        }
        for i in 0..n {
            for j in 0..m {
                if row[i] == 1 || col[j] == 1 {
                    matrix[i][j] = 0;
                }
            }
        }
    }
}
