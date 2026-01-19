struct Solution;

impl Solution {
    pub fn max_side_length(mat: Vec<Vec<i32>>, threshold: i32) -> i32 {
        let m = mat.len();
        let n = mat[0].len();
        let mut prefix_sum = vec![vec![0; n + 1]; m + 1];

        for i in 1..=m {
            for j in 1..=n {
                prefix_sum[i][j] = mat[i - 1][j - 1] + prefix_sum[i - 1][j] + prefix_sum[i][j - 1]
                    - prefix_sum[i - 1][j - 1];
            }
        }

        let mut ans = 0;

        for i in 1..=m {
            for j in 1..=n {
                let curr_max_side = ans + 1;

                if i >= curr_max_side && j >= curr_max_side {
                    let total = prefix_sum[i][j]
                        - prefix_sum[i - curr_max_side][j]
                        - prefix_sum[i][j - curr_max_side]
                        + prefix_sum[i - curr_max_side][j - curr_max_side];

                    if total <= threshold {
                        ans += 1;
                    }
                }
            }
        }

        ans as i32
    }
}
