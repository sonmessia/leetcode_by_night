use std::collections::{HashMap, HashSet};

struct Solution;

impl Solution {
    pub fn number_of_special_chars(word: String) -> i32 {
        let mut s: HashMap<char, usize> = std::collections::HashMap::new();
        let mut ans = HashSet::new();

        for (i, c) in word.chars().enumerate() {
            if c.is_uppercase() {
                if s.contains_key(&c) {
                    continue;
                } else {
                    s.insert(c, i);
                }
            }
        }

        for (i, c) in word.chars().enumerate() {
            if c.is_lowercase() {
                if let Some(idx) = s.get(&(c.to_uppercase().next().unwrap())) {
                    if *idx > i {
                        ans.insert(c);
                    } else {
                        ans.remove(&c);
                    }
                }
            }
        }

        ans.len() as i32
    }
}
