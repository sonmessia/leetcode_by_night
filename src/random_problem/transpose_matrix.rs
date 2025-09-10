use std::ops::Deref;

struct Solution;

impl Solution {
    pub fn transpose(matrix: &mut Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let m = matrix.len();
        let n = matrix[0].len();
        let mut result = vec![vec![0; m]; n];

        for i in 0..m {
            for j in 0..n {
                result[j][i] = matrix[i][j];
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_transpose() {
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![vec![1, 4, 7], vec![2, 5, 8], vec![3, 6, 9]];
        assert_eq!(Solution::transpose(&mut matrix), expected);
        let mut matrix = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let expected = vec![vec![1, 4], vec![2, 5], vec![3, 6]];
        assert_eq!(Solution::transpose(&mut matrix), expected);
    }
}
