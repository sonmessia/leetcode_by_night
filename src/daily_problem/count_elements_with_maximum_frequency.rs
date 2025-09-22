use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn max_frequency_elements(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for x in nums {
            *map.entry(x).or_insert(0) += 1;
        }
        let max_freq = map.values().cloned().max().unwrap_or(0);
        map.into_iter()
            .filter(|(_, v)| v == &max_freq)
            .map(|(_, v)| v)
            .sum()
    }
}
