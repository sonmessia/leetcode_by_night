struct Solution;

impl Solution {
    pub fn construct_product_matrix(grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let (n, m) = (grid.len(), grid[0].len());

        let mut ans = vec![vec![1; m]; n];

        let mut tmp = 1;
        for i in 0..n {
            for j in 0..m {
                ans[i][j] = ((ans[i][j] as i64 * tmp) % 12345) as i32;
                tmp = (tmp * grid[i][j] as i64) % 12345;
            }
        }

        tmp = 1;

        for i in (0..n).rev() {
            for j in (0..m).rev() {
                ans[i][j] = ((ans[i][j] as i64 * tmp) % 12345) as i32;
                tmp = (tmp * grid[i][j] as i64) % 12345;
            }
        }

        ans
    }
}
