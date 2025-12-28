struct Solution;

impl Solution {
    pub fn count_negatives(grid: Vec<Vec<i32>>) -> i32 {
        let (n, m) = (grid.len(), grid[0].len());

        let mut ans = 0;

        let mut i = 0;
        let mut j = m as i32 - 1;

        while i < n as i32 && j >= 0 {
            if grid[i as usize][j as usize] < 0 {
                ans += n as i32 - i;
                j -= 1;
            } else {
                i += 1;
            }
        }

        ans
    }
}
