use std::collections::HashSet;

struct Solution;

impl Solution {
    pub fn can_be_typed_words(text: String, broken_letters: String) -> i32 {
        let broken_set: HashSet<char> = broken_letters.chars().collect();
        text.split_whitespace()
            .filter(|word| !word.chars().any(|c| broken_set.contains(&c)))
            .count() as i32
    }
}
