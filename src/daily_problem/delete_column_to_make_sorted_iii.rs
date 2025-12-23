struct Solution;

impl Solution {
    pub fn min_deletion_size(strs: Vec<String>) -> i32 {
        let m = strs[0].len();

        let mut dp = vec![1; m];

        for i in (0..=(m - 2)).rev() {
            'outer: for j in (i + 1)..m {
                for row in strs.iter() {
                    if row.as_bytes()[i] > row.as_bytes()[j] {
                        continue 'outer;
                    }
                }
                dp[i] = dp[i].max(1 + dp[j]);
            }
        }

        let mut kept = 0;

        for i in dp {
            kept = kept.max(i);
        }

        return m as i32 - kept;
    }
}
