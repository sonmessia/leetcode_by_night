struct Solution;

impl Solution {
    pub fn count_nice_pairs(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut count_map = HashMap::new();
        let mut result = 0;
        const MOD: i32 = 1_000_000_007;
        for &num in &nums {
            let rev_num = num
                .to_string()
                .chars()
                .rev()
                .collect::<String>()
                .parse::<i32>()
                .unwrap();
            let diff = num - rev_num;
            let entry = count_map.entry(diff).or_insert(0);
            result = (result + *entry) % MOD;
            *entry += 1;
        }
        result
    }
}
