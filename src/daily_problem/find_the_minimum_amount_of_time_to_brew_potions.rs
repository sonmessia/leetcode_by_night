struct Solution;

impl Solution {
    fn min_brewing_time(skill: &[i32], mana: &[i32]) -> i64 {
        let n = skill.len();
        let m = mana.len();
        let mut dp = vec![0i64; n + 1];
        for i in 0..n {
            dp[i] = (skill[i] as i64 * mana[0] as i64);
        }

        dp[n]
    }
}
