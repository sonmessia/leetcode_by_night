struct Solution;

impl Solution {
    pub fn min_operations(grid: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut flat_grid: Vec<i32> = grid.into_iter().flatten().collect();
        if flat_grid.is_empty() {
            return 0;
        }

        let first_rem = flat_grid[0].rem_euclid(x);
        for &val in &flat_grid {
            if val.rem_euclid(x) != first_rem {
                return -1;
            }
        }

        let mid_idx = flat_grid.len() / 2;
        let (_, &mut median, _) = flat_grid.select_nth_unstable(mid_idx);

        flat_grid.iter().map(|&val| (val - median).abs() / x).sum()
    }
}
