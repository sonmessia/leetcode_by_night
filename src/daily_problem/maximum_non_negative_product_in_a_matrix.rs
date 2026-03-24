struct Solution;

impl Solution {
    pub fn max_product_path(grid: Vec<Vec<i32>>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (m, n) = (grid.len(), grid[0].len());

        let (mut max_product, mut min_product) = (vec![vec![0i64; n]; m], vec![vec![0i64; n]; m]);

        max_product[0][0] = grid[0][0] as i64;
        min_product[0][0] = grid[0][0] as i64;

        for i in 1..m {
            max_product[i][0] = max_product[i - 1][0] * grid[i][0] as i64;
            min_product[i][0] = max_product[i][0];
        }

        for j in 1..n {
            max_product[0][j] = max_product[0][j - 1] * grid[0][j] as i64;
            min_product[0][j] = max_product[0][j];
        }

        for i in 1..m {
            for j in 1..n {
                let (max_from_top, min_from_top) = (max_product[i - 1][j], min_product[i - 1][j]);
                let (max_from_left, min_from_left) = (max_product[i][j - 1], min_product[i][j - 1]);
                let current_value = grid[i][j] as i64;

                max_product[i][j] = *[
                    max_from_top * current_value,
                    min_from_top * current_value,
                    max_from_left * current_value,
                    min_from_left * current_value,
                ]
                .iter()
                .max()
                .unwrap();

                min_product[i][j] = *[
                    max_from_top * current_value,
                    min_from_top * current_value,
                    max_from_left * current_value,
                    min_from_left * current_value,
                ]
                .iter()
                .min()
                .unwrap();
            }
        }

        if max_product[m - 1][n - 1] < 0 {
            -1
        } else {
            (max_product[m - 1][n - 1] % MOD) as i32
        }
    }
}
