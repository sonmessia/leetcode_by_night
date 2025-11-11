struct Solution;

impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        // Most m 0s and n 1s we can use
        let m = m as usize;
        let n = n as usize;
        let mut dp = vec![vec![0; n + 1]; m + 1];

        for s in strs {
            let (zeros, ones) =
                s.chars().fold(
                    (0, 0),
                    |(z, o), c| {
                        if c == '0' { (z + 1, o) } else { (z, o + 1) }
                    },
                );

            println!("s: {}, zeros: {}, ones: {}", s, zeros, ones);

            for i in (zeros..=m).rev() {
                for j in (ones..=n).rev() {
                    print!("i: {}, j: {}, dp before: {} -> ", i, j, dp[i][j]);
                    dp[i][j] = dp[i][j].max(dp[i - zeros][j - ones] + 1);
                    println!("dp after: {}", dp[i][j]);
                }
            }
        }

        dp[m][n]
    }
}

fn main() {
    let strs = vec![
        "10".to_string(),
        "0001".to_string(),
        "111001".to_string(),
        "1".to_string(),
        "0".to_string(),
    ];
    let m = 5;
    let n = 3;
    let result = Solution::find_max_form(strs, m, n);
    println!("Result: {}", result); // Expected output: 4
}
