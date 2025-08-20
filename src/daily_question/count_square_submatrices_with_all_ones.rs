struct Solution;

impl Solution {
    pub fn count_squares(matrix: Vec<Vec<i32>>) -> i32 {
        let row = matrix.len();
        let col = matrix[0].len();

        let mut ans = 0;
        let mut dp = vec![vec![0; col + 1]; row + 1];

        for i in 0..row {
            for j in 0..col {
                if matrix[i][j] == 1 {
                    dp[i+1][j+1] = std::cmp::min(dp[i][j+1], std::cmp::min(dp[i+1][j], dp[i][j])) + 1;
                    ans += dp[i+1][j+1];
                }
            }
        }
        ans
    }
}


fn main() {
    let matrix = vec![
        vec![0, 1, 1, 1],
        vec![1, 1, 1, 1],
        vec![0, 1, 1, 1],
    ];
    let result = Solution::count_squares(matrix);
    println!("Number of square submatrices with all ones: {}", result);
}