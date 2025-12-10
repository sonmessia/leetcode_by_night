struct Solution;

impl Solution {
    pub fn count_permutations(complexity: Vec<i32>) -> i32 {
        const MOD: i64 = 1_000_000_007;
        let n = complexity.len();
        let mut ans: i64 = 1;

        if complexity[0] >= complexity[1] {
            return 0;
        }
        for i in 2..n {
            ans = (ans * (i as i64)) % MOD;
        }

        ans as i32
    }
}
