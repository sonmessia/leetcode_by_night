struct Solution;

impl Solution {
    pub fn shift_grid(grid: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        let m = grid.len();
        let n = grid[0].len();
        let total = m * n;
        let k = (k as usize) % total;
        let mut result = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                let new_pos = (i * n + j + k) % total;
                let new_i = new_pos / n;
                let new_j = new_pos % n;
                result[new_i][new_j] = grid[i][j];
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_shift_grid() {
        //  Example 1
        let grid = vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let k = 1;
        let expected = vec![vec![9, 1, 2], vec![3, 4, 5], vec![6, 7, 8]];
        assert_eq!(Solution::shift_grid(grid, k), expected);
        // Example 2
        let grid = vec![
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
            vec![12, 0, 21, 13],
        ];
        let k = 4;
        let expected = vec![
            vec![12, 0, 21, 13],
            vec![3, 8, 1, 9],
            vec![19, 7, 2, 5],
            vec![4, 6, 11, 10],
        ];
        assert_eq!(Solution::shift_grid(grid, k), expected);
        // Example 3
        let grid = vec![
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![7],
            vec![6],
            vec![5],
        ];
        let k = 23;
        let expected = vec![
            vec![6],
            vec![5],
            vec![1],
            vec![2],
            vec![3],
            vec![4],
            vec![7],
        ];
        assert_eq!(Solution::shift_grid(grid, k), expected);
    }
}
