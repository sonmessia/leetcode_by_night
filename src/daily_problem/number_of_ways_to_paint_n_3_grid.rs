struct Solution;

impl Solution {
    pub fn num_of_ways(n: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let (mut a, mut b) = (6i64, 6i64);

        for _ in 2..=n {
            let na = (3 * a + 2 * b) % MOD;
            let nb = (2 * a + 2 * b) % MOD;
            a = na;
            b = nb;
        }

        ((a + b) % MOD) as i32
    }
}
