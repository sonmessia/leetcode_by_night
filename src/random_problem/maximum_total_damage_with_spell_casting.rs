struct Solution;

impl Solution {
    pub fn maximum_total_damage(power: Vec<i32>) -> i64 {
        use std::collections::BTreeMap;
        let mut map = BTreeMap::new();
        for &p in &power {
            *map.entry(p).or_insert(0) += 1;
        }

        let mut val: Vec<(i64, i64)> = vec![(-1_000_000_000, 0)];
        for (k, v) in map {
            val.push((k as i64, v as i64));
        }

        let n = val.len();
        let mut dp = vec![0i64; n];
        let mut best = 0i64;
        let mut j = 1;

        for i in 1..n {
            while j < i && val[j].0 < val[i].0 - 2 {
                best = best.max(dp[j]);
                j += 1;
            }
            dp[i] = best + val[i].0 * val[i].1;
        }

        *dp.iter().max().unwrap()
    }
}
