struct Solution;

impl Solution {
    pub fn partition_string(s: String) -> i32 {
        let mut seen = std::collections::HashSet::new();
        let mut count = 1;

        for ch in s.chars() {
            if seen.contains(&ch) {
                count += 1;
                seen.clear();
            }
            seen.insert(ch);
        }

        count
    }
}
