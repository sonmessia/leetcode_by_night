struct Solution;

impl Solution {
    pub fn count_trapezoids(points: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        const MOD: i64 = 1_000_000_007;
        let mut cnt = HashMap::new();
        let (mut ans, mut sum) = (0i64, 0i64);
        for point in points {
            *cnt.entry(point[1]).or_insert(0) += 1;
        }

        for &cnt in cnt.values() {
            let n_edges = (cnt as i64 * (cnt as i64 - 1)) / 2;
            ans = (ans + sum * n_edges) % MOD;
            sum = (sum + n_edges) % MOD;
        }

        ans as i32
    }
}
