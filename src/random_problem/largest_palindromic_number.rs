use std::collections::{BTreeSet, HashMap};
struct Solution;

impl Solution {
    pub fn largest_palindrome(num: String) -> String {
        let mut map: HashMap<char, i32> = HashMap::new();

        for ch in num.chars() {
            *map.entry(ch).or_insert(0) += 1;
        }

        String::new()
    }
}

