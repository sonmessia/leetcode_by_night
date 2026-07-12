use std::sync::OnceLock;

static POW10: OnceLock<Vec<i64>> = OnceLock::new();

fn get_pow10() -> &'static Vec<i64> {
    POW10.get_or_init(|| {
        const MOD: i64 = 1_000_000_007i64;
        let mut p = vec![1i64; 100005];
        for i in 1..100005 {
            p[i] = (p[i - 1] * 10) % MOD;
        }
        p
    })
}

struct Solution;

impl Solution {
    pub fn sum_and_multiply(s: String, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let pow10 = get_pow10();
        let n = s.len();
        let s_bytes = s.as_bytes();
        const MOD: i32 = 1_000_000_007;
        let mut sum = vec![0; n + 1];
        let mut x = vec![0i64; n + 1];
        let mut cnt = vec![0; n + 1];

        for i in 0..n {
            let digit = (s_bytes[i] - b'0') as i32;
            sum[i + 1] = (sum[i] + digit) % MOD;
            if digit != 0 {
                x[i + 1] = (x[i] * 10 + digit as i64) % MOD as i64;
                cnt[i + 1] = cnt[i] + 1;
            } else {
                x[i + 1] = x[i];
                cnt[i + 1] = cnt[i];
            }
        }

        let mut ans = vec![];

        for query in queries {
            let l = query[0] as usize;
            let r = query[1] as usize + 1;
            let total_sum = (sum[r] - sum[l] + MOD) % MOD;
            let total_x = (x[r] - x[l] * pow10[(cnt[r] - cnt[l]) as usize] % MOD as i64
                + MOD as i64)
                % MOD as i64;
            ans.push((total_sum as i64 * total_x) % MOD as i64);
        }

        ans.iter().map(|&x| x as i32).collect()
    }
}
