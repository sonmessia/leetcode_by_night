struct Solution;

impl Solution {
    pub fn find_column_width(grid: Vec<Vec<i32>>) -> Vec<i32> {
        let (m, n) = (grid.len(), grid[0].len());
        let mut ans = vec![0; n];

        for i in 0..n {
            for j in 0..m {
                let width = grid[j][i].to_string().len() as i32;
                ans[i] = ans[i].max(width);
            }
        }
        ans
    }
}
