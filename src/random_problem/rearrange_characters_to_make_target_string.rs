use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn rearrange_characters(s: String, target: String) -> i32 {
        let mut freq_s = HashMap::new();
        let mut freq_t = HashMap::new();

        for ch in s.chars() {
            freq_s.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }

        for ch in target.chars() {
            freq_t.entry(ch).and_modify(|e| *e += 1).or_insert(1);
        }

        for (ch, count) in freq_t.iter_mut() {
            let count_s = freq_s.get(ch).unwrap_or(&0);
            *count = count_s / *count;
        }

        *freq_t.values().min().unwrap_or(&0) as i32
    }
}
