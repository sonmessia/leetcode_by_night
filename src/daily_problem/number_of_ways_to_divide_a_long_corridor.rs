struct Solution;

impl Solution {
    pub fn number_of_ways(corridor: String) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let (mut ans, mut cnt, mut prev) = (1, 0, -1);

        for (idx, ch) in corridor.chars().enumerate() {
            if ch == 'S' {
                cnt += 1;
                if cnt % 2 == 1 && prev != -1 {
                    ans = (ans * (idx as i32 - prev)) % MOD;
                }
                prev = idx as i32;
            }
        }

        if cnt == 0 || cnt % 2 != 0 {
            return 0;
        }

        ans
    }
}
