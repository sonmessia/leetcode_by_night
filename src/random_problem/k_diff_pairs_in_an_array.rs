struct Solution;

impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let mut seen = HashSet::new();
        let mut pairs = HashSet::new();
        for &num in &nums {
            if seen.contains(&(num - k)) {
                pairs.insert((num - k, num));
            }
            if seen.contains(&(num + k)) {
                pairs.insert((num, num + k));
            }
            seen.insert(num);
        }
        pairs.len() as i32
    }
}
