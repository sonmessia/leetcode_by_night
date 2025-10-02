use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let mut count = HashMap::new();
        let (mut left, mut right) = (0, 0);
        let mut max_count = 0;
        let mut result = 0;
        while right < s.len() {
            let entry = count.entry(s[right]).or_insert(0);
            *entry += 1;
            max_count = max_count.max(*entry);
            while (right - left + 1) as i32 - max_count > k {
                let left_entry = count.get_mut(&s[left]).unwrap();
                *left_entry -= 1;
                left += 1;
            }
            result = result.max(right - left + 1);
            right += 1;
        }
        result as i32
    }
}
