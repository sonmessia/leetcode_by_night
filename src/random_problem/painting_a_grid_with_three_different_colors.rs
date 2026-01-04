struct Solution;

impl Solution {
    pub fn color_the_grid(m: i32, n: i32) -> i32 {
        fn generate_valid_columns(
            row: usize,
            m: usize,
            curr_col: &mut Vec<i32>,
            valid_cols: &mut Vec<Vec<i32>>,
        ) {
            if row == m {
                valid_cols.push(curr_col.clone());
                return;
            }

            for color in 0..3 {
                if row == 0 || color != curr_col[row - 1] {
                    curr_col.push(color);
                    generate_valid_columns(row + 1, m, curr_col, valid_cols);
                    curr_col.pop();
                }
            }
        }

        fn is_compatible(col1: &Vec<i32>, col2: &Vec<i32>) -> bool {
            for (c1, c2) in col1.iter().zip(col2.iter()) {
                if c1 == c2 {
                    return false;
                }
            }
            true
        }

        const MOD: i64 = 1_000_000_007;
        let (m, n) = (m as usize, n as usize);

        let mut valid_cols = vec![];
        let mut curr_col = vec![];

        generate_valid_columns(0, m, &mut curr_col, &mut valid_cols);

        let num_states = valid_cols.len();

        println!("Number of valid columns: {}", num_states);
        for i in 0..num_states {
            println!("{:?}", valid_cols[i]);
        }
        let mut adj = vec![vec![]; num_states];

        for i in 0..num_states {
            for j in 0..num_states {
                if is_compatible(&valid_cols[i], &valid_cols[j]) {
                    adj[i].push(j);
                }
            }
        }

        println!("{:?}", adj);
        let mut dp = vec![1i64; num_states];

        for _ in 1..n {
            let mut new_dp = vec![0i64; num_states];
            for i in 0..num_states {
                for &j in &adj[i] {
                    new_dp[j] = (new_dp[j] + dp[i]) % MOD;
                }
            }
            dp = new_dp;
        }

        (dp.iter().sum::<i64>() % MOD) as i32
    }
}

fn main() {
    let m = 5;
    let n = 5;
    let result = Solution::color_the_grid(m, n);
    println!("Number of ways to color the grid: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_the_grid() {
        assert_eq!(Solution::color_the_grid(1, 1), 3);
        assert_eq!(Solution::color_the_grid(1, 2), 6);
        assert_eq!(Solution::color_the_grid(5, 5), 580986);
    }
}
