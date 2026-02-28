struct Solution;

impl Solution {
    pub fn concatenated_binary(n: i32) -> i32 {
        let mut shifter = 1;
        let mut val = 1;
        let mut answer: i64 = 0;
        const MOD: i64 = 1_000_000_007;

        for i in 1..=n {
            if val * 2 == i {
                shifter += 1;
                val = i;
            }
            answer = ((answer << shifter) | (i as i64)) % MOD;
        }

        answer as i32
    }
}
