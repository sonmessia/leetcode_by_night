struct Solution;

impl Solution {
    pub fn num_submat(mat: Vec<Vec<i32>>) -> i32 {
        let mut ans = 0;
        let (m, n) = (mat.len(), mat[0].len());
        let mut dp = vec![vec![0; n]; m];

        for i in 0..m {
            for j in 0..n {
                dp[i][j] = if j == 0 {
                    mat[i][j]
                } else {
                    if mat[i][j] == 0 {
                        0
                    } else {
                        dp[i][j - 1] + 1
                    }
                };
                let mut cur = dp[i][j];
                for k in (0..=i).rev() {
                    cur = cur.min(dp[k][j]);
                    if cur == 0 {
                        break;
                    }
                    ans += cur;
                }
            }
        }
        ans
    }
}

fn main() {
    let mat = vec![vec![1, 0, 1], vec![1, 1, 0], vec![1, 1, 0]];
    let result = Solution::num_submat(mat);
    println!("Number of submatrices with all 1's: {}", result);
}
