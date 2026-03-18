struct Solution;

impl Solution {
    pub fn count_submatrices(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = 0;

        let mut cols = vec![0; n];
        let mut curr_sum;

        for i in 0..m {
            curr_sum = 0;
            for j in 0..n {
                cols[j] += grid[i][j];
                curr_sum += cols[j];
                ans += (curr_sum < k) as i32;
            }
        }

        ans
    }
}
