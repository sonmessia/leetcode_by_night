struct Solution;

impl Solution {
    pub fn winner_square_game(n: i32) -> bool {
        if is_square(n) {
            return true;
        }

        let mut dp = vec![false; (n + 1) as usize];

        for i in (1..=n).take_while(|&x| x * x <= n) {
            dp[(i * i) as usize] = true;
        }

        fn is_square(x: i32) -> bool {
            let s = (x as f64).sqrt() as i32;
            s * s == x
        }

        fn dfs(remain: i32, dp: &mut Vec<bool>) -> bool {
            if is_square(remain) {
                dp[remain as usize] = true;
                return true;
            }

            if dp[remain as usize] {
                return dp[remain as usize];
            }

            for i in (1..=remain).take_while(|&x| x * x <= remain) {
                if !dfs(remain - i * i, dp) {
                    dp[remain as usize] = true;
                    return true;
                }
            }

            false
        }

        dfs(n, &mut dp);

        dp[n as usize]
    }
}
