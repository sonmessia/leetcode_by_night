struct Solution;

impl Solution {
    pub fn number_of_arrays(zero: i32, one: i32, limit: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let z = zero as usize;
        let o = one as usize;
        let l = limit as usize;

        let mut dp0 = vec![vec![0; o + 1]; z + 1];
        let mut dp1 = vec![vec![0; o + 1]; z + 1];

        for i in 1..=z.min(l) {
            dp0[i][0] = 1;
        }
        for j in 1..=o.min(l) {
            dp1[0][j] = 1;
        }

        for i in 1..=z {
            for j in 1..=o {
                let mut val = dp0[i - 1][j] as i64 + dp1[i - 1][j] as i64;
                if i > l {
                    val -= dp1[i - l - 1][j] as i64;
                }
                dp0[i][j] = ((val % MOD + MOD) % MOD) as i32;

                val = dp0[i][j - 1] as i64 + dp1[i][j - 1] as i64;
                if j > l {
                    val -= dp0[i][j - l - 1] as i64;
                }
                dp1[i][j] = ((val % MOD + MOD) % MOD) as i32;
            }
        }

        ((dp0[z][o] as i64 + dp1[z][o] as i64) % MOD) as i32
    }
}
