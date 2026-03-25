struct Solution;

impl Solution {
    pub fn can_partition_grid(grid: Vec<Vec<i32>>) -> bool {
        let (m, n) = (grid.len(), grid[0].len());
        let total_sum: i64 = grid.iter().flat_map(|r| r.iter()).map(|&x| x as i64).sum();

        if total_sum & 1 != 0 {
            return false;
        }

        let mut sum: i64 = 0;

        for i in 0..m - 1 {
            sum += grid[i].iter().map(|&x| x as i64).sum::<i64>();
            if sum << 1 == total_sum {
                return true;
            }
        }

        for i in 0..m - 1 {
            sum += grid[i].iter().map(|&x| x as i64).sum::<i64>();
            if sum << 1 == total_sum {
                return true;
            }
        }

        sum = 0;

        for i in 0..n - 1 {
            sum += grid.iter().map(|r| r[i] as i64).sum::<i64>();
            if sum << 1 == total_sum {
                return true;
            }
        }

        false
    }
}

