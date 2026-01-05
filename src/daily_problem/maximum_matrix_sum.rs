struct Solution;

impl Solution {
    pub fn max_matrix_sum(matrix: Vec<Vec<i32>>) -> i64 {
        let n = matrix.len();
        let mut sum = 0i64;
        let mut negative_count = 0;
        let mut min_abs_value = i32::MAX;

        for i in 0..n {
            for j in 0..n {
                let value = matrix[i][j];
                sum += value.abs() as i64;
                if value < 0 {
                    negative_count += 1;
                }
                min_abs_value = min_abs_value.min(value.abs());
            }
        }

        if negative_count % 2 == 0 {
            sum
        } else {
            sum - 2 * (min_abs_value as i64)
        }
    }
}
