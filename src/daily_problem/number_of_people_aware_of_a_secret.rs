struct Solution;

impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i32 = 1_000_000_007;
        let n = n as usize;
        let delay = delay as usize;
        let forget = forget as usize;
        let mut knows = vec![0; n];
        knows[0] = 1;
        let (mut shared, mut total) = (0, 1);
        for day in delay..forget {
            shared = (shared + knows[day - delay]) % MOD;
            total = (total + shared) % MOD;
            knows[day] = shared % MOD;
        }

        for day in forget..n {
            shared = (shared + (knows[day - delay] - knows[day - forget] + MOD) % MOD) % MOD;
            total = (total + (shared - knows[day - forget] + MOD) % MOD) % MOD;
            knows[day] = shared % MOD;
        }
        total % MOD
    }
}

fn main() {
    Solution::people_aware_of_secret(6, 2, 4);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_people_aware_of_secret() {
        assert_eq!(Solution::people_aware_of_secret(6, 2, 4), 5);
        assert_eq!(Solution::people_aware_of_secret(4, 1, 3), 6);
        assert_eq!(Solution::people_aware_of_secret(18, 7, 13), 16);
        assert_eq!(Solution::people_aware_of_secret(289, 7, 23), 790409951);
    }
}
